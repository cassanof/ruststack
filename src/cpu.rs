use std::{cell::RefCell, io::BufRead};

use crate::{
    memory::{InspectableAddr, Memory},
    opcodes::OpCode,
    register::Register,
    to_u16, REGISTER_SIZE,
};
use strum::{EnumCount, IntoEnumIterator};

/// Represents the CPU of the emulator. Where there are two main registers, and 8 general purpose registers.
/// There is one single memory buffer, which is used to store the data. The registers store 16 bit values,
/// and the memory buffer stores 8 bit values for each cell.
pub struct CPU {
    memory: Memory,
    registers_memory: Memory,
}

impl CPU {
    /// Creates a new CPU with the given memory buffer.
    pub fn new(memory: Memory) -> CPU {
        let memory = memory;
        let registers = Memory::new((Register::COUNT * REGISTER_SIZE).try_into().unwrap());

        // set stack and base pointer to max mem

        let sp_idx = Register::SP.to_index() * REGISTER_SIZE;
        let bp_idx = Register::BP.to_index() * REGISTER_SIZE;
        let memsize = &u16::to_be_bytes((memory.len() - 2) as u16);
        registers.set_buf(sp_idx, sp_idx + 2, memsize);
        registers.set_buf(bp_idx, bp_idx + 2, memsize);

        CPU {
            memory,
            registers_memory: registers,
        }
    }

    /// Gets the value of the given register.
    pub fn get_register(&self, reg: &Register) -> u16 {
        let index = reg.to_index() * REGISTER_SIZE;
        let buf = self.registers_memory.get_buf(index, index + 2).unwrap();
        to_u16(&buf)
    }

    /// Sets the value of the given register.
    pub fn set_register(&self, reg: &Register, value: u16) {
        let index = reg.to_index() * REGISTER_SIZE;
        self.registers_memory
            .set_buf(index, index + 2, &value.to_be_bytes());
    }

    /// Fetches the value pointed by the ip register, then increments ip by 1. Returns the fetched value.
    pub fn fetch(&self) -> Result<u8, CpuError> {
        let ipval = self.get_register(&Register::IP);
        let instruction = self
            .memory
            .get(ipval as usize)
            .ok_or(CpuError::InvalidAddress(ipval))?;
        self.set_register(&Register::IP, ipval + 1);
        Ok(instruction)
    }

    /// Fetches the given value pointed by the ip register `n` amount of times, then increments ip by `n`.
    /// Returns the fetched values.
    pub fn fetch_buf(&self, n: usize) -> Result<Vec<u8>, CpuError> {
        let mut buf: Vec<u8> = vec![0; n];
        for cell in buf.iter_mut().take(n) {
            *cell = self.fetch()?;
        }
        Ok(buf)
    }

    /// Pushes the given value to the stack, then decrements sp by 1.
    fn push(&self, value: &[u8]) {
        let sp = self.get_register(&Register::SP);
        self.memory.set_buf(sp as usize, (sp + 2) as usize, value);
        self.set_register(&Register::SP, sp - 2);
    }

    /// Pops the value from the stack, then increments sp by 1. Returns the popped value.
    fn pop(&self) -> [u8; REGISTER_SIZE] {
        let next_sp = self.get_register(&Register::SP) + 2;
        self.set_register(&Register::SP, next_sp);
        self.memory
            .get_buf(next_sp as usize, (next_sp as usize) + 2)
            .unwrap()
            .try_into()
            .unwrap()
    }

    fn fetch_reg_idx(&self) -> Result<usize, CpuError> {
        Ok(((self.fetch()? as usize) % Register::COUNT) * 2)
    }

    /// Executes the given instruction opcode. Returns true if the halt instruction is reached.
    /// False otherwise
    pub fn execute(&self, instruction: OpCode) -> Result<bool, CpuError> {
        match instruction {
            OpCode::MovLitReg => {
                let lit = self.fetch_buf(2)?;
                // modulus to loop back if we get a register index larger than the number of registers
                let reg = self.fetch_reg_idx()?;
                self.registers_memory.set_buf(reg, reg + 2, &lit);
            }
            OpCode::MovRegReg => {
                let reg_from = self.fetch_reg_idx()?;
                let reg_to = self.fetch_reg_idx()?;
                let val = to_u16(
                    &self
                        .registers_memory
                        .get_buf(reg_from, reg_from + 2)
                        .unwrap(),
                );
                self.registers_memory
                    .set_buf(reg_to, reg_to + 2, &val.to_be_bytes());
            }
            OpCode::MovRegMem => {
                let reg = self.fetch_reg_idx()?;
                let addr = to_u16(&self.fetch_buf(2)?);
                let val = &self.registers_memory.get_buf(reg, reg + 2).unwrap();
                self.memory.set_buf(addr as usize, (addr as usize) + 2, val);
            }
            OpCode::MovMemReg => {
                let addr = to_u16(&self.fetch_buf(2)?);
                let reg = self.fetch_reg_idx()?;
                let val = &self
                    .memory
                    .get_buf(addr as usize, (addr as usize) + 2)
                    .unwrap();
                self.registers_memory
                    .set_buf(reg as usize, (reg as usize) + 2, val);
            }
            OpCode::AddRegReg => {
                let r1_idx = self.fetch_reg_idx()?;
                let r2_idx = self.fetch_reg_idx()?;
                let reg_val1 = to_u16(&self.registers_memory.get_buf(r1_idx, r1_idx + 2).unwrap());
                let reg_val2 = to_u16(&self.registers_memory.get_buf(r2_idx, r2_idx + 2).unwrap());
                let result = reg_val1 + reg_val2;
                self.set_register(&Register::ACC, result);
            }
            OpCode::JmpNE => {
                let value = to_u16(&self.fetch_buf(2)?);
                let addr = to_u16(&self.fetch_buf(2)?);
                if value != self.get_register(&Register::ACC) {
                    self.set_register(&Register::IP, addr);
                }
            }
            OpCode::PshLit => {
                let value = &self.fetch_buf(2)?;
                self.push(value)
            }
            OpCode::PshReg => {
                let reg = self.fetch_reg_idx()?;
                let value = &self.registers_memory.get_buf(reg, reg + 2).unwrap();
                self.push(value)
            }
            OpCode::Pop => {
                let reg_idx = self.fetch_reg_idx()?;
                self.registers_memory
                    .set_buf(reg_idx, reg_idx + 2, &self.pop());
            }
            OpCode::CalLit => {
                let addr = to_u16(&self.fetch_buf(2)?);
                self.push(&self.get_register(&Register::IP).to_be_bytes());
                self.set_register(&Register::IP, addr);
            }
            OpCode::CalReg => {
                let reg_idx = self.fetch_reg_idx()?;
                let addr = to_u16(&self.registers_memory.get_buf(reg_idx, reg_idx + 2).unwrap());
                self.push(&self.get_register(&Register::IP).to_be_bytes());
                self.set_register(&Register::IP, addr);
            }
            OpCode::Ret => {
                let addr = to_u16(&self.pop());
                self.set_register(&Register::IP, addr);
            }
            OpCode::Hlt => {
                return Ok(true);
            }
            OpCode::NOP => (),
        }
        Ok(false)
    }

    pub fn step(&self) -> Result<bool, CpuError> {
        let instruction = self.fetch()?;
        self.execute(OpCode::from(instruction))
    }

    pub fn run(&self) -> Result<(), CpuError> {
        for line in std::io::stdin().lock().lines() {
            if self.step()? {
                break;
            }
            println!("{}", self);
        }
        Ok(())
    }
}

impl InspectableAddr for CPU {
    type Error = CpuError;

    fn inspect_addr(&self, addr: u16) -> Result<String, Self::Error> {
        self.memory.inspect_addr(addr)
    }
}

impl std::fmt::Display for CPU {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = Register::iter()
            .map(|reg| format!("{}: 0x{:X}", reg.as_ref(), self.get_register(&reg)))
            .collect::<Vec<_>>()
            .join(", ");
        write!(f, "{}", &s)
    }
}

#[derive(Debug)]
pub enum CpuError {
    InvalidInstruction,
    InvalidRegister(String),
    InvalidAddress(u16),
    InvalidValue,
}

impl std::error::Error for CpuError {}

impl std::fmt::Display for CpuError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
