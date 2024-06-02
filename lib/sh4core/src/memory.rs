#[derive(Default)]
pub enum Endianness {
    Big,
    #[default]
    Little,
}

pub struct MemoryBus {
    pub endianness: Endianness,
    memory: [u8; 65536],
}

impl Default for MemoryBus {
    fn default() -> Self {
        Self {
            memory: [0; 65536],
            endianness: Endianness::Little,
        }
    }
}

#[allow(unused)]
impl MemoryBus {
    pub fn read8(&self, addr: usize) -> Option<u8> {
        Some(self.memory[addr])
    }

    pub fn read16(&self, addr: usize) -> Option<u16> {
        if addr + 2 > self.memory.len() {
            return None;
        }

        let bytes: [u8; 2] = self.memory[addr..(addr + 2)].try_into().unwrap();
        Some(match self.endianness {
            Endianness::Big => u16::from_be_bytes(bytes),
            Endianness::Little => u16::from_le_bytes(bytes),
        })
    }

    pub fn read32(&self, addr: usize) -> Option<u32> {
        if addr + 4 > self.memory.len() {
            return None;
        }

        let bytes: [u8; 4] = self.memory[addr..(addr + 4)].try_into().unwrap();
        Some(match self.endianness {
            Endianness::Big => u32::from_be_bytes(bytes),
            Endianness::Little => u32::from_le_bytes(bytes),
        })
    }

    pub fn write8(&mut self, addr: usize, data: u8) {
        self.memory[addr] = data;
    }

    pub fn write16(&mut self, addr: usize, data: u16) {
        let data: &[u8] = match self.endianness {
            Endianness::Big => &data.to_be_bytes(),
            Endianness::Little => &data.to_le_bytes(),
        };

        self.memory[addr..(addr + 2)].copy_from_slice(data);
    }

    pub fn write32(&mut self, addr: usize, data: u32) {
        let data: &[u8] = match self.endianness {
            Endianness::Big => &data.to_be_bytes(),
            Endianness::Little => &data.to_le_bytes(),
        };

        self.memory[addr..(addr + 4)].copy_from_slice(data);
    }

    pub fn write_bin(&mut self, addr: usize, data: Vec<u8>) {
        self.memory[addr..(addr + data.len())].copy_from_slice(&data);
    }
}
