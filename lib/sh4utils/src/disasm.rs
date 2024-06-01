use sh4core::{cpu::CPU, opcodes::OpCode};
use std::io::Write;

pub fn disasemble(program: Vec<u8>) -> Vec<u8> {
    let mut result: Vec<u8> = vec![];
    let mut cpu = CPU::new();

    cpu.registers.pc = 0;
    cpu.bus.write_bin(0, program.clone());

    for _ in 0..(program.len() / 2) {
        let bytes = cpu.bus.read16(cpu.registers.pc as usize).unwrap();
        let ins = ((bytes[0] as u16) << 8) | bytes[1] as u16;

        if let Some(opcode) = OpCode::decode_instruction(ins) {
            writeln!(&mut result, "{}", opcode).unwrap();
        }

        cpu.registers.pc += 2;
    }

    result
}
