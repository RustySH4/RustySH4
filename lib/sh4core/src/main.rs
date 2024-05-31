use cpu::CPU;

#[macro_use]
extern crate lazy_static;

mod cpu;
mod cpu_impl;
mod memory;
mod opcodes;

fn main() {
    let mut cpu = CPU::new();
    cpu.registers.pc = 0;
    cpu.bus.write_bin(0, vec![0x7F, 0xF8, 0x64, 0xF3]);

    cpu.step();
    cpu.step();

    println!("{}", cpu.registers)
}
