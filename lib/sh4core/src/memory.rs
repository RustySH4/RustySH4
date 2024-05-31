pub struct MemoryBus {
    memory: [u8; 65536],
}

impl Default for MemoryBus {
    fn default() -> Self {
        Self { memory: [0; 65536] }
    }
}

#[allow(unused)]
impl MemoryBus {
    pub fn read8(&self, addr: usize) -> Option<u8> {
        Some(self.memory[addr])
    }

    pub fn read16(&self, addr: usize) -> Option<[u8; 2]> {
        if addr + 2 > self.memory.len() {
            return None;
        }

        Some(self.memory[addr..(addr + 2)].try_into().unwrap())
    }

    pub fn read32(&self, addr: usize) -> Option<[u8; 4]> {
        if addr + 4 > self.memory.len() {
            return None;
        }

        Some(self.memory[addr..(addr + 4)].try_into().unwrap())
    }

    pub fn write8(&mut self, addr: usize, data: u8) {
        self.memory[addr] = data;
    }

    pub fn write_bin(&mut self, addr: usize, data: Vec<u8>) {
        self.memory[addr..(addr + data.len())].copy_from_slice(&data);
    }
}
