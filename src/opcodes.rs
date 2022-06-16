use crate::cpu::CpuError;

/// Represents an opcode that is executable by the CPU.
pub enum OpCode {
    // Moves the literal value into the r1 register
    MovLitR1,
    // Moves the literal value into the r3 register
    MovLitR2,
    // Adds two registers together, pointed by the ip register and stores the result in the acc register
    AddRegReg,
}

impl From<OpCode> for u8 {
    fn from(op: OpCode) -> Self {
        match op {
            OpCode::MovLitR1 => 0x10,
            OpCode::MovLitR2 => 0x11,
            OpCode::AddRegReg => 0x12,
        }
    }
}

impl TryFrom<u8> for OpCode {
    type Error = CpuError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x10 => Ok(OpCode::MovLitR1),
            0x11 => Ok(OpCode::MovLitR2),
            0x12 => Ok(OpCode::AddRegReg),
            _ => Err(CpuError::InvalidInstruction),
        }
    }
}
