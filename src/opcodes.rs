/// Represents an opcode that is executable by the CPU.
pub enum OpCode {
    /// No operation
    Nop,
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
    JmpNELit,
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
    /// Moves the literal value into the memory location
    MovLitMem,
    /// Moves the pointer register to the given register
    MovRegPtrReg,
    /// Adds the given literal with the given register and stores the result in the acc register
    AddLitReg,
    /// Subtracts the given literal with the given register and stores the result in the acc register
    SubLitReg,
    /// Subtracts the given register with the given literal and stores the result in the acc register
    SubRegLit,
    /// Subtracts the given register with the given register and stores the result in the acc register
    SubRegReg,
    /// Multiplies the given literal with the given register and stores the result in the acc register
    MulLitReg,
    /// Multiplies the given register with the given register and stores the result in the acc register
    MulRegReg,
    /// Increments the given register by 1 in palce
    IncReg,
    /// Decrements the given register by 1 in palce
    DecReg,
    /// Shifts the given register left by the given amount in palce
    ShlRegLit,
    /// Shifts the given register left by the given register in place
    ShlRegReg,
    /// Shifts the given register right by the given amount in place
    ShrRegLit,
    /// Shifts the given register right by the given register in place
    ShrRegReg,
    /// Bitwise ANDs the given register with the given literal and stores the result in the acc register
    AndRegLit,
    /// Bitwise ANDs the given register with the given register and stores the result in the acc register
    AndRegReg,
    /// Bitwise ORs the given register with the given literal and stores the result in the acc register
    OrRegLit,
    /// Bitwise ORs the given register with the given register and stores the result in the acc register
    OrRegReg,
    /// Bitwise XORs the given register with the given literal and stores the result in the acc register
    XorRegLit,
    /// Bitwise XORs the given register with the given register and stores the result in the acc register
    XorRegReg,
    /// Bitwise NOTs the given register and stores the result in the acc register
    NotReg,
    /// Jumps to the given address if the acc register is not equal to the given register
    JmpNEReg,
    /// Jumps to the given address if the acc register is equal to the given literal
    JmpEQLit,
    /// Jumps to the given address if the acc register is equal to the given register
    JmpEQReg,
    /// Jumps to the given address if the given literal is less than the acc register
    JmpLTLit,
    /// Jumps to the given address if the given register is less than the acc register
    JmpLTReg,
    /// Jumps to the given address if the given literal is greater than the acc register
    JmpGTLit,
    /// Jumps to the given address if the given register is greater than the acc register
    JmpGTReg,
    /// Jumps to the given address if the given literal is less than or equal to the acc register
    JmpLELit,
    /// Jumps to the given address if the given register is less than or equal to the acc register
    JmpLEReg,
    /// Jumps to the given address if the given literal is greater than or equal to the acc register
    JmpGELit,
    /// Jumps to the given address if the given register is greater than or equal to the acc register
    JmpGEReg,
    /// Jumps to the given address
    Jmp,
    /// System call, value retrieved from the ACC register, used to call a function in the VM
    SysLit,
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
            JmpNELit => 0x15,
            PshReg => 0x16,
            PshLit => 0x17,
            Pop => 0x18,
            CalLit => 0x19,
            CalReg => 0x1A,
            Ret => 0x1B,
            Hlt => 0x1C,
            MovLitMem => 0x1D,
            MovRegPtrReg => 0x1E,
            AddLitReg => 0x20,
            SubLitReg => 0x21,
            SubRegLit => 0x22,
            SubRegReg => 0x23,
            MulLitReg => 0x24,
            MulRegReg => 0x25,
            IncReg => 0x26,
            DecReg => 0x27,
            ShlRegLit => 0x28,
            ShlRegReg => 0x29,
            ShrRegLit => 0x2A,
            ShrRegReg => 0x2B,
            AndRegLit => 0x2C,
            AndRegReg => 0x2D,
            OrRegLit => 0x2E,
            OrRegReg => 0x2F,
            XorRegLit => 0x30,
            XorRegReg => 0x31,
            NotReg => 0x32,
            JmpNEReg => 0x33,
            JmpEQLit => 0x34,
            JmpEQReg => 0x35,
            JmpLTLit => 0x36,
            JmpLTReg => 0x37,
            JmpGTLit => 0x38,
            JmpGTReg => 0x39,
            JmpLELit => 0x3A,
            JmpLEReg => 0x3B,
            JmpGELit => 0x3C,
            JmpGEReg => 0x3D,
            Jmp => 0x3E,
            SysLit => 0x3F,
            Nop => 0x00,
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
            0x15 => JmpNELit,
            0x16 => PshReg,
            0x17 => PshLit,
            0x18 => Pop,
            0x19 => CalLit,
            0x1A => CalReg,
            0x1B => Ret,
            0x1C => Hlt,
            0x1D => MovLitMem,
            0x1E => MovRegPtrReg,
            0x20 => AddLitReg,
            0x21 => SubLitReg,
            0x22 => SubRegLit,
            0x23 => SubRegReg,
            0x24 => MulLitReg,
            0x25 => MulRegReg,
            0x26 => IncReg,
            0x27 => DecReg,
            0x28 => ShlRegLit,
            0x29 => ShlRegReg,
            0x2A => ShrRegLit,
            0x2B => ShrRegReg,
            0x2C => AndRegLit,
            0x2D => AndRegReg,
            0x2E => OrRegLit,
            0x2F => OrRegReg,
            0x30 => XorRegLit,
            0x31 => XorRegReg,
            0x32 => NotReg,
            0x33 => JmpNEReg,
            0x34 => JmpEQLit,
            0x35 => JmpEQReg,
            0x36 => JmpLTLit,
            0x37 => JmpLTReg,
            0x38 => JmpGTLit,
            0x39 => JmpGTReg,
            0x3A => JmpLELit,
            0x3B => JmpLEReg,
            0x3C => JmpGELit,
            0x3D => JmpGEReg,
            0x3E => Jmp,
            0x3F => SysLit,
            _ => Nop,
        }
    }
}
