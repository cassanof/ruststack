use std::{collections::HashMap, num::ParseIntError};

use crate::{
    ast::{ASTArg, ASTNode},
    memory::{Memory, MemoryBuilder},
    opcodes::OpCode,
    parser,
};

pub struct Assembler;

// macro to get index of register
macro_rules! reg_i {
    ($arg:expr) => {
        $arg.to_index() as u8
    };
}

impl Assembler {
    pub fn assemble(input: Vec<ASTNode>) -> Result<Memory, AssemblerError> {
        let mut builder = MemoryBuilder::new(Memory::default());
        // the labels encountered so far
        let mut label_addrs: HashMap<String, u16> = HashMap::new();
        // the pending jumps/calls that need to be patched with the correct label address
        let mut need_patching: Vec<(String, usize)> = vec![];
        for node in input {
            match node {
                ASTNode::Label(name) => {
                    let addr = builder.get_counter() as u16;
                    label_addrs.insert(name, addr);
                }
                ASTNode::Mov(a1, a2) => match (&a1, &a2) {
                    (ASTArg::Lit(lit), ASTArg::Reg(reg)) => {
                        builder.push(OpCode::MovLitReg.into());
                        builder.push_u16(*lit);
                        builder.push(reg_i!(reg));
                    }
                    (ASTArg::Reg(reg1), ASTArg::Reg(reg2)) => {
                        builder.push(OpCode::MovRegReg.into());
                        builder.push(reg_i!(reg1));
                        builder.push(reg_i!(reg2));
                    }
                    (ASTArg::Reg(reg1), ASTArg::Mem(mem)) => {
                        builder.push(OpCode::MovRegMem.into());
                        builder.push(reg_i!(reg1));
                        match &**mem {
                            ASTArg::Label(label) => {
                                need_patching.push((label.to_string(), builder.get_counter()));
                                builder.incr();
                            }
                            ASTArg::Lit(lit) => {
                                builder.push_u16(*lit);
                            }
                            ASTArg::Reg(ptrreg) => {
                                builder.push(reg_i!(ptrreg));
                            }
                            _ => return Err(AssemblerError::InvalidArgument(a2)),
                        }
                    }
                    (ASTArg::Mem(mem), ASTArg::Reg(reg2)) => {
                        match &**mem {
                            ASTArg::Label(label) => {
                                builder.push(OpCode::MovMemReg.into());
                                need_patching.push((label.to_string(), builder.get_counter()));
                                builder.incr();
                            }
                            ASTArg::Lit(lit) => {
                                builder.push(OpCode::MovMemReg.into());
                                builder.push_u16(*lit);
                            }
                            ASTArg::Reg(reg) => {
                                builder.push(OpCode::MovRegPtrReg.into());
                                builder.push(reg_i!(reg));
                            }
                            _ => return Err(AssemblerError::InvalidArgument(a2)),
                        }
                        builder.push(reg_i!(reg2));
                    }
                    _ => return Err(AssemblerError::InvalidArgument(a1)),
                },
                ASTNode::Add(a, reg) => match reg {
                    ASTArg::Reg(reg) => match a {
                        ASTArg::Reg(reg2) => {
                            builder.push(OpCode::AddRegReg.into());
                            builder.push(reg_i!(reg2));
                            builder.push(reg_i!(reg));
                        }
                        ASTArg::Lit(lit) => {
                            builder.push(OpCode::AddLitReg.into());
                            builder.push_u16(lit);
                            builder.push(reg_i!(reg));
                        }
                        _ => return Err(AssemblerError::InvalidArgument(a)),
                    },
                    _ => return Err(AssemblerError::InvalidArgument(reg)),
                },
                ASTNode::Sub(a1, a2) => match (&a1, &a2) {
                    (ASTArg::Reg(r1), ASTArg::Reg(r2)) => {
                        builder.push(OpCode::SubRegReg.into());
                        builder.push(reg_i!(r1));
                        builder.push(reg_i!(r2));
                    }
                    (ASTArg::Reg(r1), ASTArg::Lit(lit)) => {
                        builder.push(OpCode::SubRegLit.into());
                        builder.push(reg_i!(r1));
                        builder.push_u16(*lit);
                    }
                    (ASTArg::Lit(lit), ASTArg::Reg(r2)) => {
                        builder.push(OpCode::SubRegLit.into());
                        builder.push_u16(*lit);
                        builder.push(reg_i!(r2));
                    }
                    _ => return Err(AssemblerError::InvalidArgument(a1)), // lazy zzzz
                },
                ASTNode::Mul(a, reg) => match reg {
                    ASTArg::Reg(reg) => match a {
                        ASTArg::Reg(reg2) => {
                            builder.push(OpCode::MulRegReg.into());
                            builder.push(reg_i!(reg2));
                            builder.push(reg_i!(reg));
                        }
                        ASTArg::Lit(lit) => {
                            builder.push(OpCode::MulLitReg.into());
                            builder.push_u16(lit);
                            builder.push(reg_i!(reg));
                        }
                        _ => return Err(AssemblerError::InvalidArgument(a)),
                    },
                    _ => return Err(AssemblerError::InvalidArgument(reg)),
                },
                ASTNode::Shl(reg, a) => match reg {
                    ASTArg::Reg(reg) => match a {
                        ASTArg::Reg(reg2) => {
                            builder.push(OpCode::ShlRegReg.into());
                            builder.push(reg_i!(reg));
                            builder.push(reg_i!(reg2));
                        }
                        ASTArg::Lit(lit) => {
                            builder.push(OpCode::ShlRegLit.into());
                            builder.push(reg_i!(reg));
                            builder.push_u16(lit);
                        }
                        _ => return Err(AssemblerError::InvalidArgument(a)),
                    },
                    _ => return Err(AssemblerError::InvalidArgument(reg)),
                },
                ASTNode::Shr(reg, a) => match reg {
                    ASTArg::Reg(reg) => match a {
                        ASTArg::Reg(reg2) => {
                            builder.push(OpCode::ShrRegReg.into());
                            builder.push(reg_i!(reg));
                            builder.push(reg_i!(reg2));
                        }
                        ASTArg::Lit(lit) => {
                            builder.push(OpCode::ShrRegLit.into());
                            builder.push(reg_i!(reg));
                            builder.push_u16(lit);
                        }
                        _ => return Err(AssemblerError::InvalidArgument(a)),
                    },
                    _ => return Err(AssemblerError::InvalidArgument(reg)),
                },
                ASTNode::And(reg, a) => match reg {
                    ASTArg::Reg(reg) => match a {
                        ASTArg::Reg(reg2) => {
                            builder.push(OpCode::AndRegReg.into());
                            builder.push(reg_i!(reg));
                            builder.push(reg_i!(reg2));
                        }
                        ASTArg::Lit(lit) => {
                            builder.push(OpCode::AndRegLit.into());
                            builder.push(reg_i!(reg));
                            builder.push_u16(lit);
                        }
                        _ => return Err(AssemblerError::InvalidArgument(a)),
                    },
                    _ => return Err(AssemblerError::InvalidArgument(reg)),
                },
                ASTNode::Or(reg, a) => match reg {
                    ASTArg::Reg(reg) => match a {
                        ASTArg::Reg(reg2) => {
                            builder.push(OpCode::OrRegReg.into());
                            builder.push(reg_i!(reg));
                            builder.push(reg_i!(reg2));
                        }
                        ASTArg::Lit(lit) => {
                            builder.push(OpCode::OrRegLit.into());
                            builder.push(reg_i!(reg));
                            builder.push_u16(lit);
                        }
                        _ => return Err(AssemblerError::InvalidArgument(a)),
                    },
                    _ => return Err(AssemblerError::InvalidArgument(reg)),
                },
                ASTNode::Not(reg) => match reg {
                    ASTArg::Reg(r) => {
                        builder.push(OpCode::NotReg.into());
                        builder.push(reg_i!(r));
                    }
                    _ => return Err(AssemblerError::InvalidArgument(reg)),
                },
                ASTNode::Xor(reg, a) => match reg {
                    ASTArg::Reg(reg) => match a {
                        ASTArg::Lit(lit) => {
                            builder.push(OpCode::XorRegLit.into());
                            builder.push(reg_i!(reg));
                            builder.push_u16(lit);
                        }
                        ASTArg::Reg(reg2) => {
                            builder.push(OpCode::XorRegLit.into());
                            builder.push(reg_i!(reg));
                            builder.push(reg_i!(reg2));
                        }
                        _ => return Err(AssemblerError::InvalidArgument(a)),
                    },
                    _ => return Err(AssemblerError::InvalidArgument(reg)),
                },
                ASTNode::Jne(label, a) => match label {
                    ASTArg::Label(label) => {
                        match a {
                            ASTArg::Lit(lit) => {
                                builder.push(OpCode::JmpNELit.into());
                                builder.set_counter(builder.get_counter() + 1);
                                builder.push_u16(lit);
                            }
                            ASTArg::Reg(reg) => {
                                builder.push(OpCode::JmpNEReg.into());
                                builder.set_counter(builder.get_counter() + 1);
                                builder.push(reg_i!(reg));
                            }
                            _ => return Err(AssemblerError::InvalidArgument(a)),
                        };
                        need_patching.push((label, builder.get_counter() - 2));
                    }
                    _ => return Err(AssemblerError::InvalidArgument(label)),
                },
                ASTNode::Jeq(label, a) => match label {
                    ASTArg::Label(label) => {
                        match a {
                            ASTArg::Lit(lit) => {
                                builder.push(OpCode::JmpEQLit.into());
                                builder.set_counter(builder.get_counter() + 1);
                                builder.push_u16(lit);
                            }
                            ASTArg::Reg(reg) => {
                                builder.push(OpCode::JmpEQReg.into());
                                builder.set_counter(builder.get_counter() + 1);
                                builder.push(reg_i!(reg));
                            }
                            _ => return Err(AssemblerError::InvalidArgument(a)),
                        };
                        need_patching.push((label, builder.get_counter() - 2));
                    }
                    _ => return Err(AssemblerError::InvalidArgument(label)),
                },
                ASTNode::Jlt(label, a) => match label {
                    ASTArg::Label(label) => {
                        match a {
                            ASTArg::Lit(lit) => {
                                builder.push(OpCode::JmpLTLit.into());
                                builder.set_counter(builder.get_counter() + 1);
                                builder.push_u16(lit);
                            }
                            ASTArg::Reg(reg) => {
                                builder.push(OpCode::JmpLTReg.into());
                                builder.set_counter(builder.get_counter() + 1);
                                builder.push(reg_i!(reg));
                            }
                            _ => return Err(AssemblerError::InvalidArgument(a)),
                        };
                        need_patching.push((label, builder.get_counter() - 2));
                    }
                    _ => return Err(AssemblerError::InvalidArgument(label)),
                },
                ASTNode::Jgt(label, a) => match label {
                    ASTArg::Label(label) => {
                        match a {
                            ASTArg::Lit(lit) => {
                                builder.push(OpCode::JmpGTLit.into());
                                builder.set_counter(builder.get_counter() + 1);
                                builder.push_u16(lit);
                            }
                            ASTArg::Reg(reg) => {
                                builder.push(OpCode::JmpGTReg.into());
                                builder.set_counter(builder.get_counter() + 1);
                                builder.push(reg_i!(reg));
                            }
                            _ => return Err(AssemblerError::InvalidArgument(a)),
                        };
                        need_patching.push((label, builder.get_counter() - 2));
                    }
                    _ => return Err(AssemblerError::InvalidArgument(label)),
                },
                ASTNode::Jle(label, a) => match label {
                    ASTArg::Label(label) => {
                        match a {
                            ASTArg::Lit(lit) => {
                                builder.push(OpCode::JmpLELit.into());
                                builder.set_counter(builder.get_counter() + 1);
                                builder.push_u16(lit);
                            }
                            ASTArg::Reg(reg) => {
                                builder.push(OpCode::JmpLEReg.into());
                                builder.set_counter(builder.get_counter() + 1);
                                builder.push(reg_i!(reg));
                            }
                            _ => return Err(AssemblerError::InvalidArgument(a)),
                        };
                        need_patching.push((label, builder.get_counter() - 2));
                    }
                    _ => return Err(AssemblerError::InvalidArgument(label)),
                },
                ASTNode::Jge(label, a) => match label {
                    ASTArg::Label(label) => {
                        match a {
                            ASTArg::Lit(lit) => {
                                builder.push(OpCode::JmpGELit.into());
                                builder.set_counter(builder.get_counter() + 1);
                                builder.push_u16(lit);
                            }
                            ASTArg::Reg(reg) => {
                                builder.push(OpCode::JmpGEReg.into());
                                builder.set_counter(builder.get_counter() + 1);
                                builder.push(reg_i!(reg));
                            }
                            _ => return Err(AssemblerError::InvalidArgument(a)),
                        };
                        need_patching.push((label, builder.get_counter() - 2));
                    }
                    _ => return Err(AssemblerError::InvalidArgument(label)),
                },
                ASTNode::Jmp(label) => match label {
                    ASTArg::Label(label) => {
                        builder.push(OpCode::Jmp.into());
                        need_patching.push((label, builder.get_counter()));
                        builder.incr();
                    }
                    _ => return Err(AssemblerError::InvalidArgument(label)),
                },
                ASTNode::Psh(a) => match a {
                    ASTArg::Lit(lit) => {
                        builder.push(OpCode::PshLit.into());
                        builder.push_u16(lit);
                    }
                    ASTArg::Reg(reg) => {
                        builder.push(OpCode::PshReg.into());
                        builder.push(reg_i!(reg));
                    }
                    _ => return Err(AssemblerError::InvalidArgument(a)),
                },
                ASTNode::Pop(reg) => {
                    builder.push(OpCode::Pop.into());
                    match reg {
                        ASTArg::Reg(reg) => builder.push(reg_i!(reg)),
                        _ => return Err(AssemblerError::InvalidArgument(reg)),
                    };
                }
                ASTNode::Cal(a) => match a {
                    ASTArg::Lit(lit) => {
                        builder.push(OpCode::CalLit.into());
                        builder.push_u16(lit);
                    }
                    ASTArg::Reg(reg) => {
                        builder.push(OpCode::CalReg.into());
                        builder.push(reg_i!(reg));
                    }
                    ASTArg::Label(label) => {
                        builder.push(OpCode::CalLit.into());
                        need_patching.push((label, builder.get_counter()));
                        builder.incr();
                    }
                    _ => return Err(AssemblerError::InvalidArgument(a)),
                },
                ASTNode::Inc(reg) => {
                    builder.push(OpCode::IncReg.into());
                    match reg {
                        ASTArg::Reg(reg) => builder.push(reg_i!(reg)),
                        _ => return Err(AssemblerError::InvalidArgument(reg)),
                    };
                }
                ASTNode::Dec(reg) => {
                    builder.push(OpCode::DecReg.into());
                    match reg {
                        ASTArg::Reg(reg) => builder.push(reg_i!(reg)),
                        _ => return Err(AssemblerError::InvalidArgument(reg)),
                    };
                }
                ASTNode::Sys(val) => {
                    builder.push(OpCode::SysLit.into());
                    match val {
                        ASTArg::Lit(lit) => builder.push(lit as u8),
                        ASTArg::Label(_) => todo!("Here, we should make some kind of functin that converts function names to u8"),
                        _ => return Err(AssemblerError::InvalidArgument(val)),
                    };
                }
                ASTNode::Ret => {
                    builder.push(OpCode::Ret.into());
                }
                ASTNode::Hlt => {
                    builder.push(OpCode::Hlt.into());
                }
                ASTNode::Nop => {
                    builder.push(OpCode::Nop.into());
                }
            };
        }

        for (label, mem_idx) in need_patching {
            let addr = label_addrs
                .get(&label)
                .ok_or(AssemblerError::InvalidLabel(label))?;
            builder.set_counter(mem_idx);
            builder.push_u16(*addr);
        }

        Ok(builder.build())
    }
}

#[derive(Debug)]
pub enum AssemblerError {
    Parser(String),
    Io(std::io::Error),
    InvalidLabel(String),
    InvalidArgument(ASTArg), // the node and the argument that was invalid
}

impl From<std::io::Error> for AssemblerError {
    fn from(error: std::io::Error) -> Self {
        AssemblerError::Io(error)
    }
}

impl From<pest::error::Error<parser::Rule>> for AssemblerError {
    fn from(error: pest::error::Error<parser::Rule>) -> Self {
        AssemblerError::Parser(format!("{}", error))
    }
}

impl From<ParseIntError> for AssemblerError {
    fn from(error: ParseIntError) -> Self {
        AssemblerError::Parser(format!("{}", error))
    }
}

impl std::fmt::Display for AssemblerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AssemblerError::Parser(s) => write!(f, "Parser error: {}", s),
            AssemblerError::Io(e) => write!(f, "IO error: {}", e),
            AssemblerError::InvalidLabel(s) => write!(f, "Invalid label: {}", s),
            AssemblerError::InvalidArgument(arg) => {
                write!(f, "Invalid argument: {:?}", arg)
            }
        }
    }
}

impl std::error::Error for AssemblerError {}
