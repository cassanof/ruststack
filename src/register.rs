use std::str::FromStr;
use strum_macros::{AsRefStr, EnumCount as EnumCountMacro, EnumIter};

use crate::cpu::CpuError;

#[derive(Debug, Copy, Clone, EnumCountMacro, EnumIter, AsRefStr, PartialEq, Eq)]
/// Represents the registers available in the CPU.
pub enum Register {
    IP,  // Instruction pointer
    ACC, // Accumulator
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
    SP, // Stack pointer
    BP, // Base pointer
}

impl Register {
    pub fn to_index(&self) -> usize {
        *self as usize
    }
}

impl FromStr for Register {
    type Err = CpuError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "ip" => Ok(Register::IP),
            "acc" => Ok(Register::ACC),
            "r1" => Ok(Register::R1),
            "r2" => Ok(Register::R2),
            "r3" => Ok(Register::R3),
            "r4" => Ok(Register::R4),
            "r5" => Ok(Register::R5),
            "r6" => Ok(Register::R6),
            "r7" => Ok(Register::R7),
            "r8" => Ok(Register::R8),
            "sp" => Ok(Register::SP),
            "bp" => Ok(Register::BP),
            _ => Err(CpuError::InvalidRegister(s.to_string())),
        }
    }
}
