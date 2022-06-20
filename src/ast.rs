use std::str::FromStr;

use crate::register::Register;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ASTNode {
    Label(String),
    Mov(ASTArg, ASTArg),
    Add(ASTArg, ASTArg),
    Sub(ASTArg, ASTArg),
    Mul(ASTArg, ASTArg),
    Shl(ASTArg, ASTArg),
    Shr(ASTArg, ASTArg),
    And(ASTArg, ASTArg),
    Or(ASTArg, ASTArg),
    Xor(ASTArg, ASTArg),
    Jne(ASTArg, ASTArg),
    Jeq(ASTArg, ASTArg),
    Jlt(ASTArg, ASTArg),
    Jgt(ASTArg, ASTArg),
    Jle(ASTArg, ASTArg),
    Jge(ASTArg, ASTArg),
    Not(ASTArg),
    Jmp(ASTArg),
    Psh(ASTArg),
    Pop(ASTArg),
    Cal(ASTArg),
    Inc(ASTArg),
    Dec(ASTArg),
    Ret,
    Hlt,
    Nop,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ASTArg {
    Label(String),
    Lit(u16),
    Reg(Register),
    Mem(u16),
    Offset(Box<ASTArg>, u16),
}
