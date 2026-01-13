pub use byte_core::*;

pub struct TestBus {
    pub data: [u8; 0x10000],
}

impl Default for TestBus {
    fn default() -> Self {
        Self {
            data: [0u8; 0x10000],
        }
    }
}

impl bus::Bus for TestBus {
    fn read(&self, addr: u16) -> u8 {
        self.data[addr as usize]
    }

    fn write(&mut self, addr: u16, byte: u8) {
        self.data[addr as usize] = byte;
    }
}

pub fn execute_with_bus<B: bus::Bus + Default>(
    config: impl FnOnce(&mut cpu::CPU<B>),
    program: &[u8],
    addr: u16,
    steps: usize,
) -> cpu::CPU<B> {
    let mut cpu = cpu::CPU::<B>::default();
    config(&mut cpu);

    cpu.reg.pc = addr;
    cpu.load(program, addr);

    (0..steps).for_each(|_| cpu.step().unwrap());
    cpu
}

#[allow(dead_code)]
pub fn execute(
    config: impl FnOnce(&mut cpu::CPU<TestBus>),
    program: &[u8],
    addr: u16,
    n: usize,
) -> cpu::CPU<TestBus> {
    execute_with_bus::<TestBus>(config, program, addr, n)
}
