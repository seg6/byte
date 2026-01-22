pub mod system {
    use eframe::egui::Color32;

    pub const DEFAULT_BINARY: &[u8; 1 << 16] = include_bytes!("../assets/demo.bin");
    pub const DEFAULT_SOURCE: &str = include_str!("../assets/demo.s");

    pub const W: usize = 64;
    pub const H: usize = 64;
    pub const COLOR_PALETTE: [Color32; 16] = [
        Color32::from_rgb(0x00, 0x00, 0x00),
        Color32::from_rgb(0xFF, 0xFF, 0xFF),
        Color32::from_rgb(0x88, 0x00, 0x00),
        Color32::from_rgb(0xAA, 0xFF, 0xEE),
        Color32::from_rgb(0xCC, 0x44, 0xCC),
        Color32::from_rgb(0x00, 0xCC, 0x55),
        Color32::from_rgb(0x00, 0x00, 0xAA),
        Color32::from_rgb(0xEE, 0xEE, 0x77),
        Color32::from_rgb(0x66, 0x44, 0x00),
        Color32::from_rgb(0xFF, 0x77, 0x77),
        Color32::from_rgb(0x33, 0x33, 0x33),
        Color32::from_rgb(0x77, 0x77, 0x77),
        Color32::from_rgb(0xAA, 0xFF, 0x66),
        Color32::from_rgb(0x00, 0x88, 0xFF),
        Color32::from_rgb(0x00, 0x88, 0xFF),
        Color32::from_rgb(0xBB, 0xBB, 0xBB),
    ];
    pub const FRAMEBUFFER_SIZE: usize = W * H;

    pub const INSTRUCTIONS_PER_FRAME: usize = 6400000 / 60;
}

pub mod mmio {
    pub const VID: u16 = 0xfd;
    pub const INP: u16 = 0xff;
    pub const RNG: u16 = 0xfe;
}
