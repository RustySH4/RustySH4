#![allow(dead_code)]

// TODO: Implement correct memory layout
// TODO: Allow for diffrent memory sizes
// TODO: Implement conditional display, because of it's size
#[derive(Debug)]
pub struct SH4Memory {
    mem: [u8; 2048],
}

impl Default for SH4Memory {
    fn default() -> Self {
        SH4Memory { mem: [0; 2048] }
    }
}

// TODO: Use Bytes create rather than direct access
impl SH4Memory {
    fn read8(&self, addr: usize) -> u8 {
        self.mem[addr]
    }

    fn read16(&self, addr: usize) -> u16 {
        // TODO: Endianess
        ((self.mem[addr] as u16) << 8) + (self.mem[addr] as u16)
    }

    fn read32(&self, addr: usize) -> u32 {
        // TODO: Endianess
        ((self.mem[addr] as u32) << 24)
            + ((self.mem[addr] as u32) << 18)
            + ((self.mem[addr] as u32) << 16)
            + (self.mem[addr] as u32)
    }
}
