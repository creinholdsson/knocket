use crate::cpu;
use crate::mem;
use simple_logger::SimpleLogger;

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
        let result = self.mem.load_program(program);

        for index in (0x200..0x200+program.len()).step_by(2) {
            log::trace!("{}", format!("opcode {:04x}", self.mem.fetch_opcode(index)));
        }

        result
    }

    pub fn run_cycle(&mut self, keypad: &[bool; 16]) {
        self.cpu.execute_cycle(&mut self.mem, &keypad);
    }
}
