use byte_core::bus::Bus;

pub struct ByteBus {
    data: [u8; 0x10000],
}

impl Default for ByteBus {
    fn default() -> Self {
        Self {
            data: [0u8; 0x10000],
        }
    }
}

impl Bus for ByteBus {
    fn read(&self, addr: u16) -> u8 {
        self.data[addr as usize]
    }

    fn write(&mut self, addr: u16, byte: u8) {
        self.data[addr as usize] = byte;
    }
}

impl ByteBus {
    pub fn get_memory_region(&self, start: u16, size: usize) -> &[u8] {
        let start = start as usize;
        let end = (start + size).min(self.data.len());
        &self.data[start..end]
    }
}
