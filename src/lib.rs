pub mod cpu;
pub mod memory;
pub mod opcodes;
pub mod register;
pub mod parser;
pub mod ast;
pub mod assembler;

/// The size of a register in bytes. 16 bits in this case.
pub const REGISTER_SIZE: usize = 2;

/// Helper function to convert an iterator of bytes to a u16 bit value.
pub fn to_u16(iter: &[u8]) -> u16 {
    u16::from_be_bytes([iter[0], iter[1]])
}

#[cfg(test)]
mod tests {
    use crate::{
        cpu::CPU,
        memory::{InspectableAddr, Memory},
        opcodes::OpCode,
        register::Register,
    };

    /// Memory buffer builder for easy testing.
    struct MemoryBuilder {
        memory: Memory,
        counter: usize,
    }

    impl MemoryBuilder {
        fn new(memory: Memory) -> MemoryBuilder {
            MemoryBuilder { memory, counter: 0 }
        }

        fn push(&mut self, value: u8) -> usize {
            self.memory.set(self.counter, value);
            self.counter += 1;
            self.counter
        }

        fn set_counter(&mut self, counter: usize) {
            self.counter = counter;
        }

        fn build(self) -> Memory {
            self.memory
        }
    }

    #[test]
    fn get_index_reg() {
        let reg = Register::IP;
        assert_eq!(reg.to_index(), 0);
        let reg = Register::ACC;
        assert_eq!(reg.to_index(), 1);
        let reg = Register::R1;
        assert_eq!(reg.to_index(), 2);
        let reg = Register::R2;
        assert_eq!(reg.to_index(), 3);
        let reg = Register::R3;
        assert_eq!(reg.to_index(), 4);
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
        let mut mem = MemoryBuilder::new(Memory::new(256 * 255));
        mem.push(OpCode::MovLitReg.into());
        mem.push(0x12);
        mem.push(0x34);
        mem.push(Register::R1.to_index() as u8);

        mem.push(OpCode::MovLitReg.into());
        mem.push(0xAB);
        mem.push(0xCD);
        mem.push(Register::R2.to_index() as u8);

        mem.push(OpCode::AddRegReg.into());
        mem.push(Register::R1.to_index() as u8); // r1 idx
        mem.push(Register::R2.to_index() as u8); // r2 idx

        let cpu = CPU::new(mem.build());
        assert_eq!(
            "IP: 0x0, ACC: 0x0, R1: 0x0, R2: 0x0, R3: 0x0, R4: 0x0, R5: 0x0, R6: 0x0, R7: 0x0, R8: 0x0, SP: 0xFEFE, BP: 0xFEFE",
            cpu.to_string()
        );
        cpu.step().unwrap();
        assert_eq!(
            "IP: 0x4, ACC: 0x0, R1: 0x1234, R2: 0x0, R3: 0x0, R4: 0x0, R5: 0x0, R6: 0x0, R7: 0x0, R8: 0x0, SP: 0xFEFE, BP: 0xFEFE",
            cpu.to_string()
        );
        cpu.step().unwrap();
        assert_eq!(
            "IP: 0x8, ACC: 0x0, R1: 0x1234, R2: 0xABCD, R3: 0x0, R4: 0x0, R5: 0x0, R6: 0x0, R7: 0x0, R8: 0x0, SP: 0xFEFE, BP: 0xFEFE",
            cpu.to_string()
        );
        cpu.step().unwrap();
        assert_eq!(
            "IP: 0xB, ACC: 0xBE01, R1: 0x1234, R2: 0xABCD, R3: 0x0, R4: 0x0, R5: 0x0, R6: 0x0, R7: 0x0, R8: 0x0, SP: 0xFEFE, BP: 0xFEFE",
            cpu.to_string()
        );
    }

    #[test]
    fn test_cpu_add_mov_mem() {
        let mut mem = MemoryBuilder::new(Memory::new(256 * 255));
        mem.push(OpCode::MovLitReg.into());
        mem.push(0x12);
        mem.push(0x34);
        mem.push(Register::R1.to_index() as u8);

        mem.push(OpCode::MovLitReg.into());
        mem.push(0xAB);
        mem.push(0xCD);
        mem.push(Register::R2.to_index() as u8);

        mem.push(OpCode::AddRegReg.into());
        mem.push(Register::R1.to_index() as u8); // r1 idx
        mem.push(Register::R2.to_index() as u8); // r2 idx

        mem.push(OpCode::MovRegMem.into());
        mem.push(Register::ACC.to_index() as u8);
        mem.push(0x01); // 0x0100
        mem.push(0x00);

        let cpu = CPU::new(mem.build());
        assert_eq!(
            "IP: 0x0, ACC: 0x0, R1: 0x0, R2: 0x0, R3: 0x0, R4: 0x0, R5: 0x0, R6: 0x0, R7: 0x0, R8: 0x0, SP: 0xFEFE, BP: 0xFEFE",
            cpu.to_string()
        );
        assert_eq!(
            "0x0000: 0x10 0x12 0x34 0x02 0x10 0xAB 0xCD 0x03",
            cpu.inspect_addr(cpu.get_register(&Register::IP)).unwrap()
        );
        assert_eq!(
            "0x0100: 0x00 0x00 0x00 0x00 0x00 0x00 0x00 0x00",
            cpu.inspect_addr(0x0100).unwrap()
        );
        cpu.step().unwrap();
        assert_eq!(
            "IP: 0x4, ACC: 0x0, R1: 0x1234, R2: 0x0, R3: 0x0, R4: 0x0, R5: 0x0, R6: 0x0, R7: 0x0, R8: 0x0, SP: 0xFEFE, BP: 0xFEFE",
            cpu.to_string()
        );
        assert_eq!(
            "0x0004: 0x10 0xAB 0xCD 0x03 0x14 0x02 0x03 0x12",
            cpu.inspect_addr(cpu.get_register(&Register::IP)).unwrap()
        );
        assert_eq!(
            "0x0100: 0x00 0x00 0x00 0x00 0x00 0x00 0x00 0x00",
            cpu.inspect_addr(0x0100).unwrap()
        );
        cpu.step().unwrap();
        assert_eq!(
            "IP: 0x8, ACC: 0x0, R1: 0x1234, R2: 0xABCD, R3: 0x0, R4: 0x0, R5: 0x0, R6: 0x0, R7: 0x0, R8: 0x0, SP: 0xFEFE, BP: 0xFEFE",
            cpu.to_string()
        );
        assert_eq!(
            "0x0008: 0x14 0x02 0x03 0x12 0x01 0x01 0x00 0x00",
            cpu.inspect_addr(cpu.get_register(&Register::IP)).unwrap()
        );
        assert_eq!(
            "0x0100: 0x00 0x00 0x00 0x00 0x00 0x00 0x00 0x00",
            cpu.inspect_addr(0x0100).unwrap()
        );
        cpu.step().unwrap();
        assert_eq!(
            "IP: 0xB, ACC: 0xBE01, R1: 0x1234, R2: 0xABCD, R3: 0x0, R4: 0x0, R5: 0x0, R6: 0x0, R7: 0x0, R8: 0x0, SP: 0xFEFE, BP: 0xFEFE",
            cpu.to_string()
        );
        assert_eq!(
            "0x000B: 0x12 0x01 0x01 0x00 0x00 0x00 0x00 0x00",
            cpu.inspect_addr(cpu.get_register(&Register::IP)).unwrap()
        );
        assert_eq!(
            "0x0100: 0x00 0x00 0x00 0x00 0x00 0x00 0x00 0x00",
            cpu.inspect_addr(0x0100).unwrap()
        );
        cpu.step().unwrap();
        assert_eq!(
            "IP: 0xF, ACC: 0xBE01, R1: 0x1234, R2: 0xABCD, R3: 0x0, R4: 0x0, R5: 0x0, R6: 0x0, R7: 0x0, R8: 0x0, SP: 0xFEFE, BP: 0xFEFE",
            cpu.to_string()
        );
        assert_eq!(
            "0x000F: 0x00 0x00 0x00 0x00 0x00 0x00 0x00 0x00",
            cpu.inspect_addr(cpu.get_register(&Register::IP)).unwrap()
        );
        assert_eq!(
            "0x0100: 0xBE 0x01 0x00 0x00 0x00 0x00 0x00 0x00",
            cpu.inspect_addr(0x0100).unwrap()
        );
    }

    #[test]
    fn test_conditional_jne() {
        // should loop 3 times
        let mut mem = MemoryBuilder::new(Memory::new(256 * 255));
        mem.push(OpCode::MovMemReg.into());
        mem.push(0x01);
        mem.push(0x00);
        mem.push(Register::R1.to_index() as u8);
        mem.push(OpCode::MovLitReg.into());
        mem.push(0x00);
        mem.push(0x10); // 0x0010
        mem.push(Register::R2.to_index() as u8);
        mem.push(OpCode::AddRegReg.into());
        mem.push(Register::R1.to_index() as u8);
        mem.push(Register::R2.to_index() as u8);
        mem.push(OpCode::MovRegMem.into());
        mem.push(Register::ACC.to_index() as u8);
        mem.push(0x01);
        mem.push(0x00);
        mem.push(OpCode::JmpNELit.into());
        mem.push(0x00);
        mem.push(0x03); // 0x0003
        mem.push(0x00);
        mem.push(0x00); // 0x0000, aka the start
        let cpu = CPU::new(mem.build());

        // check if it loops three times
        for i in 0..4 {
            assert_eq!(0, cpu.get_register(&Register::IP));
            cpu.step().unwrap();
            cpu.step().unwrap();
            cpu.step().unwrap();
            cpu.step().unwrap();
            cpu.step().unwrap();
            assert_eq!(
            format!("IP: 0x0, ACC: 0x{}, R1: 0x{}, R2: 0x10, R3: 0x0, R4: 0x0, R5: 0x0, R6: 0x0, R7: 0x0, R8: 0x0, SP: 0xFEFE, BP: 0xFEFE", (i+1) * 10, (i * 10)), 
            cpu.to_string()
        );
        }
        assert_eq!(
            "IP: 0x0, ACC: 0x40, R1: 0x30, R2: 0x10, R3: 0x0, R4: 0x0, R5: 0x0, R6: 0x0, R7: 0x0, R8: 0x0, SP: 0xFEFE, BP: 0xFEFE",
            cpu.to_string()
        );
        assert_eq!(
            "0x0100: 0x00 0x40 0x00 0x00 0x00 0x00 0x00 0x00",
            cpu.inspect_addr(0x0100).unwrap()
        );
    }

    #[test]
    fn test_pop_and_push() {
        // should loop 3 times
        let mut mem = MemoryBuilder::new(Memory::new(256 * 255));
        mem.push(OpCode::MovLitReg.into());
        mem.push(0x51);
        mem.push(0x51);
        mem.push(Register::R1.to_index() as u8);
        mem.push(OpCode::MovLitReg.into());
        mem.push(0x42);
        mem.push(0x42);
        mem.push(Register::R2.to_index() as u8);
        mem.push(OpCode::PshReg.into());
        mem.push(Register::R1.to_index() as u8);
        mem.push(OpCode::PshReg.into());
        mem.push(Register::R2.to_index() as u8);
        mem.push(OpCode::Pop.into());
        mem.push(Register::R1.to_index() as u8);
        mem.push(OpCode::Pop.into());
        mem.push(Register::R2.to_index() as u8);

        let cpu = CPU::new(mem.build());
        cpu.step().unwrap();
        assert_eq!("IP: 0x4, ACC: 0x0, R1: 0x5151, R2: 0x0, R3: 0x0, R4: 0x0, R5: 0x0, R6: 0x0, R7: 0x0, R8: 0x0, SP: 0xFEFE, BP: 0xFEFE", cpu.to_string());
        cpu.step().unwrap();
        assert_eq!("IP: 0x8, ACC: 0x0, R1: 0x5151, R2: 0x4242, R3: 0x0, R4: 0x0, R5: 0x0, R6: 0x0, R7: 0x0, R8: 0x0, SP: 0xFEFE, BP: 0xFEFE", cpu.to_string());
        assert_eq!(
            "0xFEFE: 0x00 0x00",
            cpu.inspect_addr(cpu.get_register(&Register::SP)).unwrap()
        );
        cpu.step().unwrap();
        assert_eq!("IP: 0xA, ACC: 0x0, R1: 0x5151, R2: 0x4242, R3: 0x0, R4: 0x0, R5: 0x0, R6: 0x0, R7: 0x0, R8: 0x0, SP: 0xFEFC, BP: 0xFEFE", cpu.to_string());
        assert_eq!(
            "0xFEFC: 0x00 0x00 0x51 0x51",
            cpu.inspect_addr(cpu.get_register(&Register::SP)).unwrap()
        );
    }

    #[test]
    fn test_calls_and_ret() {
        let mut mem = MemoryBuilder::new(Memory::new(256 * 255));
        // this should simulate a function application:  add(x, y) = x + y; add(2, 1) -> 3
        mem.push(OpCode::PshLit.into());
        mem.push(0x00);
        mem.push(0x02);
        // calling convention: push args
        mem.push(OpCode::PshLit.into());
        mem.push(0x00);
        mem.push(0x01);
        mem.push(OpCode::CalLit.into());
        mem.push(0x04); // label at 0x0400
        mem.push(0x00);
        mem.push(OpCode::MovLitReg.into());
        mem.push(0x00);
        mem.push(0x03);
        mem.push(Register::R1.to_index() as u8);
        mem.push(OpCode::AddRegReg.into());
        mem.push(Register::ACC.to_index() as u8);
        mem.push(Register::R1.to_index() as u8);
        // at the end, ACC should be 6

        // lets build the function
        mem.set_counter(0x0400);
        // calling convention: push base pointer, set base pointer to stack pointer
        mem.push(OpCode::PshReg.into());
        mem.push(Register::BP.to_index() as u8);
        // sp, bp -> bp = sp
        mem.push(OpCode::MovRegReg.into());
        mem.push(Register::SP.to_index() as u8);
        mem.push(Register::BP.to_index() as u8);
        // set sp to sp+2 and pop args
        mem.push(OpCode::MovLitReg.into());
        mem.push(0x00);
        mem.push(0x04);
        mem.push(Register::R1.to_index() as u8);
        mem.push(OpCode::AddRegReg.into());
        mem.push(Register::SP.to_index() as u8);
        mem.push(Register::R1.to_index() as u8);
        mem.push(OpCode::MovRegReg.into());
        mem.push(Register::ACC.to_index() as u8);
        mem.push(Register::SP.to_index() as u8);
        mem.push(OpCode::Pop.into());
        mem.push(Register::R1.to_index() as u8);
        mem.push(OpCode::Pop.into());
        mem.push(Register::R2.to_index() as u8);
        mem.push(OpCode::AddRegReg.into());
        mem.push(Register::R1.to_index() as u8);
        mem.push(Register::R2.to_index() as u8);
        // calling convention: leave, set stack pointer to base pointer, then pop base pointer
        mem.push(OpCode::MovRegReg.into());
        mem.push(Register::BP.to_index() as u8);
        mem.push(Register::SP.to_index() as u8);
        mem.push(OpCode::Pop.into());
        mem.push(Register::BP.to_index() as u8);
        mem.push(OpCode::Ret.into());

        let cpu = CPU::new(mem.build());
        cpu.step().unwrap();
        assert_eq!("IP: 0x3, ACC: 0x0, R1: 0x0, R2: 0x0, R3: 0x0, R4: 0x0, R5: 0x0, R6: 0x0, R7: 0x0, R8: 0x0, SP: 0xFEFC, BP: 0xFEFE", cpu.to_string());
        cpu.step().unwrap();
        assert_eq!("IP: 0x6, ACC: 0x0, R1: 0x0, R2: 0x0, R3: 0x0, R4: 0x0, R5: 0x0, R6: 0x0, R7: 0x0, R8: 0x0, SP: 0xFEFA, BP: 0xFEFE", cpu.to_string());
        cpu.step().unwrap();
        assert_eq!("IP: 0x400, ACC: 0x0, R1: 0x0, R2: 0x0, R3: 0x0, R4: 0x0, R5: 0x0, R6: 0x0, R7: 0x0, R8: 0x0, SP: 0xFEF8, BP: 0xFEFE", cpu.to_string());
        cpu.step().unwrap();
        assert_eq!("IP: 0x402, ACC: 0x0, R1: 0x0, R2: 0x0, R3: 0x0, R4: 0x0, R5: 0x0, R6: 0x0, R7: 0x0, R8: 0x0, SP: 0xFEF6, BP: 0xFEFE", cpu.to_string());
        cpu.step().unwrap();
        assert_eq!("IP: 0x405, ACC: 0x0, R1: 0x0, R2: 0x0, R3: 0x0, R4: 0x0, R5: 0x0, R6: 0x0, R7: 0x0, R8: 0x0, SP: 0xFEF6, BP: 0xFEF6", cpu.to_string());
        cpu.step().unwrap();
        assert_eq!("IP: 0x409, ACC: 0x0, R1: 0x4, R2: 0x0, R3: 0x0, R4: 0x0, R5: 0x0, R6: 0x0, R7: 0x0, R8: 0x0, SP: 0xFEF6, BP: 0xFEF6", cpu.to_string());
        cpu.step().unwrap();
        assert_eq!("IP: 0x40C, ACC: 0xFEFA, R1: 0x4, R2: 0x0, R3: 0x0, R4: 0x0, R5: 0x0, R6: 0x0, R7: 0x0, R8: 0x0, SP: 0xFEF6, BP: 0xFEF6", cpu.to_string());
        assert_eq!(
            "0xFEFA: 0x00 0x09 0x00 0x01 0x00 0x02",
            cpu.inspect_addr(0xfefa).unwrap()
        );
        cpu.step().unwrap();
        assert_eq!("IP: 0x40F, ACC: 0xFEFA, R1: 0x4, R2: 0x0, R3: 0x0, R4: 0x0, R5: 0x0, R6: 0x0, R7: 0x0, R8: 0x0, SP: 0xFEFA, BP: 0xFEF6", cpu.to_string());
        cpu.step().unwrap();
        assert_eq!("IP: 0x411, ACC: 0xFEFA, R1: 0x1, R2: 0x0, R3: 0x0, R4: 0x0, R5: 0x0, R6: 0x0, R7: 0x0, R8: 0x0, SP: 0xFEFC, BP: 0xFEF6", cpu.to_string());
        cpu.step().unwrap();
        assert_eq!("IP: 0x413, ACC: 0xFEFA, R1: 0x1, R2: 0x2, R3: 0x0, R4: 0x0, R5: 0x0, R6: 0x0, R7: 0x0, R8: 0x0, SP: 0xFEFE, BP: 0xFEF6", cpu.to_string());
        cpu.step().unwrap();
        assert_eq!("IP: 0x416, ACC: 0x3, R1: 0x1, R2: 0x2, R3: 0x0, R4: 0x0, R5: 0x0, R6: 0x0, R7: 0x0, R8: 0x0, SP: 0xFEFE, BP: 0xFEF6", cpu.to_string());
        cpu.step().unwrap();
        assert_eq!("IP: 0x419, ACC: 0x3, R1: 0x1, R2: 0x2, R3: 0x0, R4: 0x0, R5: 0x0, R6: 0x0, R7: 0x0, R8: 0x0, SP: 0xFEF6, BP: 0xFEF6", cpu.to_string());
        cpu.step().unwrap();
        assert_eq!("IP: 0x41B, ACC: 0x3, R1: 0x1, R2: 0x2, R3: 0x0, R4: 0x0, R5: 0x0, R6: 0x0, R7: 0x0, R8: 0x0, SP: 0xFEF8, BP: 0xFEFE", cpu.to_string());
        cpu.step().unwrap();
        assert_eq!("IP: 0x9, ACC: 0x3, R1: 0x1, R2: 0x2, R3: 0x0, R4: 0x0, R5: 0x0, R6: 0x0, R7: 0x0, R8: 0x0, SP: 0xFEFA, BP: 0xFEFE", cpu.to_string());
        cpu.step().unwrap();
        assert_eq!("IP: 0xD, ACC: 0x3, R1: 0x3, R2: 0x2, R3: 0x0, R4: 0x0, R5: 0x0, R6: 0x0, R7: 0x0, R8: 0x0, SP: 0xFEFA, BP: 0xFEFE", cpu.to_string());
        cpu.step().unwrap();
        assert_eq!("IP: 0x10, ACC: 0x6, R1: 0x3, R2: 0x2, R3: 0x0, R4: 0x0, R5: 0x0, R6: 0x0, R7: 0x0, R8: 0x0, SP: 0xFEFA, BP: 0xFEFE", cpu.to_string());
    }
}
