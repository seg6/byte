use eframe::egui;
use bitflags::bitflags;
use byte_core::{bus::Bus, cpu};
use std::collections::HashSet;

use crate::emu::rand;
use crate::emu::bus::ByteBus;
use crate::constants::{system, mmio};

pub struct ByteEmu {
    cpu: cpu::CPU<ByteBus>,
    rand: Box<dyn Iterator<Item = u32>>,
}

bitflags! {
    pub struct ByteInputState: u8 {
        const RIGHT  = 0b00000001;
        const LEFT   = 0b00000010;
        const DOWN   = 0b00000100;
        const UP     = 0b00001000;
        const START  = 0b00010000;
        const SELECT = 0b00100000;
        const B      = 0b01000000;
        const A      = 0b10000000;
    }
}

impl From<HashSet<egui::Key>> for ByteInputState {
    fn from(val: HashSet<egui::Key>) -> ByteInputState {
        use egui::Key::*;
        let mut state = ByteInputState::empty();

        #[rustfmt::skip]
        val.iter().for_each(|key| match key {
            A          => state.insert(ByteInputState::SELECT),
            S          => state.insert(ByteInputState::START),
            D          => state.insert(ByteInputState::A),
            F          => state.insert(ByteInputState::B),
            ArrowUp    => state.insert(ByteInputState::UP),
            ArrowDown  => state.insert(ByteInputState::DOWN),
            ArrowLeft  => state.insert(ByteInputState::LEFT),
            ArrowRight => state.insert(ByteInputState::RIGHT),
            _          => ()
        });

        state
    }
}

impl Default for ByteEmu {
    fn default() -> Self {
        Self {
            cpu: cpu::CPU::<ByteBus>::default(),
            rand: Box::new(rand::random_numbers(rand::random_seed() as u32)),
        }
    }
}

impl ByteEmu {
    pub fn load_program(&mut self, program: &[u8], start: u16) {
        self.cpu.load(program, start);
        self.cpu.interrupt(cpu::Interrupt::RST);
    }

    pub fn framebuffer(&self) -> [u32; system::FRAMEBUFFER_SIZE] {
        let mut frame = [0u32; system::FRAMEBUFFER_SIZE];
        let video_ptr = (self.cpu.bus.read(mmio::VID) as u16 & 0xf) << 0xc;
        let video_mem = self.get_memory_region(video_ptr, system::FRAMEBUFFER_SIZE);

        frame.iter_mut().zip(video_mem).for_each(|(pixel, color)| {
            *pixel = system::COLOR_PALETTE[(color & 0xf) as usize];
        });
        frame
    }

    pub fn step(&mut self, input_state: ByteInputState) {
        self.cpu.bus.write(mmio::INP, input_state.bits());

        for _ in 0..system::INSTRUCTIONS_PER_FRAME {
            if let Some(n) = self.rand.next() {
                self.cpu.bus.write(mmio::RNG, n as u8);
            }
            if let Err(err) = self.cpu.step() {
                log::error!("{err}");
            };
        }

        self.cpu.interrupt(cpu::Interrupt::IRQ);
    }

    pub fn get_memory_region(&self, start: u16, size: usize) -> &[u8] {
        self.cpu.bus.get_memory_region(start, size)
    }
}
