use crate::cpu;
use crate::mem;

pub struct Chip8 {
    pub cpu: cpu::Cpu,
    pub mem: mem::Mem,
}

impl Chip8 {
    pub fn new() -> Chip8 {
        Chip8 {
            mem: mem::Mem::new(),
            cpu: cpu::Cpu::new(),
        }
    }
}

impl Default for Chip8 {
    fn default() -> Self {
        Self::new()
    }
}

impl Chip8 {
    pub fn load_program(&mut self, program: &[u8]) -> Result<(), &'static str> {
        self.mem.load_program(program)
    }
}
