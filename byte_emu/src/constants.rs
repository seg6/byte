pub mod system {
    pub const DEFAULT_BINARY: &[u8; 1 << 16] = include_bytes!("../assets/demo.bin");
    pub const DEFAULT_SOURCE: &str = include_str!("../assets/demo.s");

    pub const W: usize = 64;
    pub const H: usize = 64;
    pub const COLOR_PALETTE: [u32; 16] = [
        0x000000FF, 0xFFFFFFFF, 0x880000FF, 0xAAFFEEFF, 0xCC44CCFF, 0x00CC55FF, 0x0000AAFF,
        0xEEEE77FF, 0x664400FF, 0xFF7777FF, 0x333333FF, 0x777777FF, 0xAAFF66FF, 0x0088FFFF,
        0x0088FFFF, 0xBBBBBBFF,
    ];
    pub const FRAMEBUFFER_SIZE: usize = W * H;

    pub const INSTRUCTIONS_PER_FRAME: usize = 6400000 / 60;
}

pub mod mmio {
    pub const VID: u16 = 0xfd;
    pub const INP: u16 = 0xff;
    pub const RNG: u16 = 0xfe;
}
