use std::str::FromStr;
use strum_macros::{EnumCount as EnumCountMacro, EnumIter, AsRefStr};

use crate::{cpu::CpuError, REGISTER_SIZE};

#[derive(Debug, Copy, Clone, EnumCountMacro, EnumIter, AsRefStr, PartialEq)]
/// Represents the registers available in the CPU.
pub enum Register {
    IP,
    ACC,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
}

impl Register {
    pub fn to_index(&self) -> usize {
        (*self as usize) * REGISTER_SIZE
    }
}

impl FromStr for Register {
    type Err = CpuError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
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
            _ => Err(CpuError::InvalidRegister(s.to_string())),
        }
    }
}
