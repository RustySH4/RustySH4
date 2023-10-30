#![allow(dead_code)]

use crate::memory::SH4Memory;
use crate::registers::SH4Registers;
use crate::traits_empty_generated::SH4ISA;

#[derive(Default, Debug)]
pub struct SH4Core {
    ebreak: bool,
    debug: bool,
    pc: u32,
    mem: SH4Memory,
    regs: SH4Registers,
    delay_slot_flag: bool,
}

impl SH4ISA for SH4Core {
    fn access_memory(&mut self) -> &mut SH4Memory {
        &mut self.mem
    }

    fn access_registers(&mut self) -> &mut SH4Registers {
        &mut self.regs
    }
}
