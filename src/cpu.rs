use crate::{memory::Memory, opcodes::OpCode, register::Register, to_u16, REGISTER_SIZE};
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
        let registers = Memory::new(Register::COUNT * REGISTER_SIZE);
        CPU {
            memory,
            registers_memory: registers,
        }
    }

    /// Gets the value of the given register.
    pub fn get_register(&self, reg: &Register) -> u16 {
        let index = reg.to_index();
        let buf = self.registers_memory.get_buf(index, index + 2).unwrap();
        to_u16(&buf)
    }

    /// Sets the value of the given register.
    pub fn set_register(&self, reg: &Register, value: u16) {
        let index = reg.to_index();
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

    /// Executes the given instruction opcode.
    pub fn execute(&self, instruction: OpCode) -> Result<(), CpuError> {
        match instruction {
            OpCode::MovLitR1 => {
                let value = self.fetch_buf(2)?;
                self.set_register(&Register::R1, to_u16(&value));
            }
            OpCode::MovLitR2 => {
                let value = self.fetch_buf(2)?;
                self.set_register(&Register::R2, to_u16(&value));
            }
            OpCode::AddRegReg => {
                let r1_idx = {
                    let r1 = self.fetch()?;
                    (r1 as usize) * 2
                };
                let r2_idx = {
                    let r2 = self.fetch()?;
                    (r2 as usize) * 2
                };
                let reg_val1 = to_u16(&self.registers_memory.get_buf(r1_idx, r1_idx + 2).unwrap());
                let reg_val2 = to_u16(&self.registers_memory.get_buf(r2_idx, r2_idx + 2).unwrap());
                let result = reg_val1 + reg_val2;
                self.set_register(&Register::ACC, result);
            }
        }
        Ok(())
    }

    pub fn step(&self) -> Result<(), CpuError> {
        let instruction = self.fetch()?;
        self.execute(OpCode::try_from(instruction)?)?;
        Ok(())
    }
}

impl std::fmt::Display for CPU {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = Register::iter()
            .map(|reg| format!("{}: 0x{:x}", reg.as_ref(), self.get_register(&reg)))
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
