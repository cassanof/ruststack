pub mod cpu;
pub mod memory;
pub mod opcodes;
pub mod register;

/// The size of a register in bytes. 16 bits in this case.
pub const REGISTER_SIZE: usize = 2;

/// Helper function to convert an iterator of bytes to a u16 bit value.
pub fn to_u16(iter: &[u8]) -> u16 {
    u16::from_be_bytes([iter[0], iter[1]])
}

#[cfg(test)]
mod tests {
    use crate::{cpu::CPU, memory::Memory, opcodes::OpCode, register::Register};

    #[test]
    fn get_index_reg() {
        let reg = Register::IP;
        assert_eq!(reg.to_index(), 0);
        let reg = Register::ACC;
        assert_eq!(reg.to_index(), 2);
        let reg = Register::R1;
        assert_eq!(reg.to_index(), 4);
        let reg = Register::R2;
        assert_eq!(reg.to_index(), 6);
        let reg = Register::R3;
        assert_eq!(reg.to_index(), 8);
    }

    #[test]
    fn test_cpu_basic_regs() {
        let cpu = CPU::new(Memory::new(100));
        assert_eq!(cpu.get_register(&Register::IP), 0);
        assert_eq!(cpu.get_register(&Register::ACC), 0);
        assert_eq!(cpu.get_register(&Register::R1), 0);
        assert_eq!(cpu.get_register(&Register::R2), 0);
        assert_eq!(cpu.get_register(&Register::R3), 0);
        assert_eq!(cpu.get_register(&Register::R4), 0);

        cpu.set_register(&Register::IP, 10);
        cpu.set_register(&Register::ACC, 20);
        assert_eq!(cpu.get_register(&Register::IP), 10);
        assert_eq!(cpu.get_register(&Register::ACC), 20);
    }

    #[test]
    fn test_cpu_add() {
        let mem = Memory::new(256);

        mem.set(0, OpCode::MovLitR1.into());
        mem.set(1, 0x12);
        mem.set(2, 0x34);

        mem.set(3, OpCode::MovLitR2.into());
        mem.set(4, 0xAB);
        mem.set(5, 0xCD);

        mem.set(6, OpCode::AddRegReg.into());
        mem.set(7, 0x2); // r1 idx
        mem.set(8, 0x3); // r2 idx

        let cpu = CPU::new(mem);
        assert_eq!(
            "IP: 0x0, ACC: 0x0, R1: 0x0, R2: 0x0, R3: 0x0, R4: 0x0, R5: 0x0, R6: 0x0, R7: 0x0, R8: 0x0",
            cpu.to_string()
        );
        cpu.step().unwrap();
        assert_eq!(
            "IP: 0x3, ACC: 0x0, R1: 0x1234, R2: 0x0, R3: 0x0, R4: 0x0, R5: 0x0, R6: 0x0, R7: 0x0, R8: 0x0",
            cpu.to_string()
        );
        cpu.step().unwrap();
        assert_eq!(
            "IP: 0x6, ACC: 0x0, R1: 0x1234, R2: 0xabcd, R3: 0x0, R4: 0x0, R5: 0x0, R6: 0x0, R7: 0x0, R8: 0x0",
            cpu.to_string()
        );
        cpu.step().unwrap();
        assert_eq!(
            "IP: 0x9, ACC: 0xbe01, R1: 0x1234, R2: 0xabcd, R3: 0x0, R4: 0x0, R5: 0x0, R6: 0x0, R7: 0x0, R8: 0x0",
            cpu.to_string()
        );
    }
}
