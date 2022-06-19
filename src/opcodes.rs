use crate::cpu::CpuError;

/// Represents an opcode that is executable by the CPU.
pub enum OpCode {
    /// No operation
    NOP,
    /// Moves the literal value into the register
    MovLitReg,
    /// Moves the value in the register into the register
    MovRegReg,
    /// Moves the value in the register into the memory location
    MovRegMem,
    /// Moves the value in the memory location into the register
    MovMemReg,
    /// Adds two registers together, pointed by the ip register and stores the result in the acc register
    AddRegReg,
    /// Jumps to the given address if the acc register is not equal to the given value
    JmpNE,
    /// Pushes the value of the given register onto the stack
    PshReg,
    /// Pushes the value of the given literal onto the stack
    PshLit,
    /// Pops the value off the stack and stores it in the given register
    Pop,
    /// Calls the given literal address
    CalLit,
    /// Calls the given register address
    CalReg,
    /// Returns from the current function
    Ret,
    /// Halts the CPU
    Hlt,
}

impl From<OpCode> for u8 {
    fn from(op: OpCode) -> Self {
        use OpCode::*;
        match op {
            MovLitReg => 0x10,
            MovRegReg => 0x11,
            MovRegMem => 0x12,
            MovMemReg => 0x13,
            AddRegReg => 0x14,
            JmpNE => 0x15,
            PshReg => 0x16,
            PshLit => 0x17,
            Pop => 0x18,
            CalLit => 0x19,
            CalReg => 0x1A,
            Ret => 0x1B,
            Hlt => 0x1C,
            NOP => 0x00,
        }
    }
}

impl From<u8> for OpCode {
    fn from(value: u8) -> Self {
        use OpCode::*;
        match value {
            0x10 => MovLitReg,
            0x11 => MovRegReg,
            0x12 => MovRegMem,
            0x13 => MovMemReg,
            0x14 => AddRegReg,
            0x15 => JmpNE,
            0x16 => PshReg,
            0x17 => PshLit,
            0x18 => Pop,
            0x19 => CalLit,
            0x1A => CalReg,
            0x1B => Ret,
            0x1C => Hlt,
            _ => NOP,
        }
    }
}
