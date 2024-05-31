use std::fmt;

use crate::{
    memory::MemoryBus,
    opcodes_generated::{OpCode, OpCodeArgs},
};

pub struct Registers {
    // TODO: Add BANK0 and BANK1 for privleage modes
    pub r: [u32; 16],
    pub pc: u32,
    fpscr: u32,
}

impl Default for Registers {
    fn default() -> Self {
        Self {
            r: [0; 16],
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

#[derive(Default)]
pub struct CPU {
    pub registers: Registers,
    pub bus: MemoryBus,
}

impl CPU {
    pub fn new() -> Self {
        Self::default()
    }

    fn sign_extend(x: u16) -> i32 {
        if (x & 0x80) == 0 {
            x as i32 & 0x000_000FF
        } else {
            x as i32 | 0xFFFF_FF00u32 as i32
        }
    }

    fn usign_extend(x: u16) -> u32 {
        if (x & 0x80) == 0 {
            x as u32 & 0x000_000FF
        } else {
            x as u32 | 0xFFFF_FF00
        }
    }

    fn MOV(&mut self, args: OpCodeArgs) {
        self.registers.r[args.n as usize] = self.registers.r[args.m as usize];
        self.registers.pc += 2;
    }

    fn ADD(&mut self, args: OpCodeArgs) {
        self.registers.r[args.n as usize] += self.registers.r[args.m as usize];
        self.registers.pc += 2
    }

    fn ADDI(&mut self, args: OpCodeArgs) {
        self.registers.r[args.n as usize] += CPU::usign_extend(args.i);
        self.registers.pc += 2;
    }

    pub fn execute(&mut self, opcode: OpCode) {
        match opcode.id {
            0 => self.MOV(opcode.args),
            79 => self.ADD(opcode.args),
            80 => self.ADDI(opcode.args),
            _ => {
                println!("UNIMPLEMENTED OPCODE: {:#?}", opcode);
                todo!()
            }
        }
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
}
