pub trait Bus {
    fn read(&self, addr: u16) -> u8;
    fn write(&mut self, addr: u16, byte: u8);

    fn read_u16(&self, addr: u16) -> u16 {
        let lo = self.read(addr);
        let hi = self.read(addr.wrapping_add(1));

        (hi as u16) << 8 | (lo as u16)
    }

    fn write_u16(&mut self, addr: u16, data: u16) {
        self.write(addr, (data & 0xff) as u8);
        self.write(addr.wrapping_add(1), (data >> 8) as u8);
    }
}
