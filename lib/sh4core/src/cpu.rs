use std::fmt;

use crate::{memory::MemoryBus, opcodes::OpCode};

#[allow(unused)]
pub struct Registers {
    // TODO: Add BANK0 and BANK1 for privleage modes
    pub r: [u32; 16],
    pub pr: u32,
    pub pc: u32,
    fpscr: u32,
}

impl Default for Registers {
    fn default() -> Self {
        Self {
            r: [0; 16],
            pr: 0,
            pc: 0xA0000000,
            fpscr: 0x00040001,
        }
    }
}

impl fmt::Display for Registers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..16 {
            write!(f, "R_{:0>2}: {:08X}    ", i, self.r[i])?;
            if (i + 1) % 4 == 0 {
                writeln!(f)?;
            }
        }

        Ok(())
    }
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Default)]
pub struct CPU {
    pub registers: Registers,
    pub bus: MemoryBus,
}

impl CPU {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn step(&mut self) {
        let bytes = self.bus.read16(self.registers.pc as usize).unwrap();
        let ins = ((bytes[0] as u16) << 8) | bytes[1] as u16;

        if let Some(opcode) = OpCode::decode_instruction(ins) {
            self.execute(opcode)
        } else {
            panic!("Unkown instruction found for: 0x{:x}", ins);
        };
    }

    pub fn delay_slot(&mut self, addr: usize) {
        let pc = self.registers.pc;
        self.registers.pc = addr as u32;
        self.step();
        self.registers.pc = pc;
    }
}
