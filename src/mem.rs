/*
    Memory map:
    0x000 - 0x1FF - Interpreter for Chip 8
    0x000 - 0x0A0 - 4x4 built in font set
    0x200 - 0xFFF - Program ROM and RAM

    http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#0.0
*/
const MEMORY_SIZE: usize = 0x1000;
const STACK_SIZE: usize = 0x10;
const PROGRAM_MEMORY_START: usize = 0x200;
const GRAPHICS_WIDTH: usize = 64;
const GRAPHICS_HEIGHT: usize = 64;
const DIGITS_MEMORY_START: usize = 0x1AF;

const DIGITS: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

pub struct Mem {
    memory: [u8; MEMORY_SIZE],
    stack: [u16; STACK_SIZE],
    pub graphics: [u32; GRAPHICS_HEIGHT * GRAPHICS_WIDTH],
    stack_pointer: usize,
}

impl Mem {
    pub fn new() -> Mem {
        let mut mem = Mem {
            memory: [0; MEMORY_SIZE],
            stack: [0; STACK_SIZE],
            graphics: [0; GRAPHICS_HEIGHT * GRAPHICS_WIDTH],
            stack_pointer: 0,
        };

        mem.memory[DIGITS_MEMORY_START..0x1FF].copy_from_slice(&DIGITS);
        mem
    }

    pub fn load_program(&mut self, program: &[u8]) -> Result<(), &'static str> {
        if PROGRAM_MEMORY_START + program.len() > MEMORY_SIZE {
            return Err("Program is too large to fit in memory");
        }

        self.memory[PROGRAM_MEMORY_START..PROGRAM_MEMORY_START + program.len()]
            .clone_from_slice(program);
        Ok(())
    }

    pub fn fetch_opcode(&self, index: usize) -> u16 {
        (self.memory[index] as u16) << 8 | (self.memory[index + 1]) as u16
    }

    pub fn fetch(&self, index: usize) -> u8 {
        self.memory[index]
    }

    pub fn store(&mut self, index: usize, value: u8) {
        if index < PROGRAM_MEMORY_START {
            panic!("Segmentation fault");
        }

        self.memory[index] = value;
    }

    pub fn fetch_graphics(&self, x: usize, y: usize) -> u8 {
        (self.graphics[x + (y << 6)] & 0xFF) as u8
    }

    pub fn store_graphics(&mut self, x: usize, y: usize, val: u8) {
        self.graphics[x + (y << 6)] = val as u32 | 0xFF00;
    }

    pub fn clear_graphics(&mut self) {
        for n in self.graphics.iter_mut() {
            *n = 0;
        }
    }

    pub fn push(&mut self, addr: u16) {
        self.stack_pointer += 1;
        self.stack[self.stack_pointer] = addr;
    }

    pub fn peek(&self) -> u16 {
        self.stack[self.stack_pointer]
    }

    pub fn pop(&mut self) -> u16 {
        self.stack_pointer -= 1;
        self.stack[self.stack_pointer + 1]
    }

    pub fn get_address_for_digit(&self, digit: u8) -> u16 {
        (DIGITS_MEMORY_START + 5 * digit as usize) as u16
    }
}

impl Default for Mem {
    fn default() -> Self {
        Self::new()
    }
}

#[test]
fn test_load_program() {
    let program: [u8; 4] = [1, 2, 3, 4];
    let mut mem = Mem::new();

    mem.load_program(&program).unwrap();
    assert_eq!(1, mem.memory[PROGRAM_MEMORY_START]);
    assert_eq!(2, mem.memory[PROGRAM_MEMORY_START + 1]);
    assert_eq!(3, mem.memory[PROGRAM_MEMORY_START + 2]);
    assert_eq!(4, mem.memory[PROGRAM_MEMORY_START + 3]);
}

#[test]
fn test_stack() {
    let mut mem = Mem::new();

    mem.push(1);
    assert_eq!(1, mem.peek());
    let val = mem.pop();
    assert_eq!(1, val);
}

#[test]
fn test_fetch_digit_address() {
    let mem = Mem::new();

    assert_eq!(DIGITS_MEMORY_START as u16, mem.get_address_for_digit(0));

    assert_eq!(
        (DIGITS_MEMORY_START + 5) as u16,
        mem.get_address_for_digit(1)
    );
}

#[test]
fn test_store_and_fetch() {
    let mut mem = Mem::new();

    mem.store(0x200, 0xFF);
    assert_eq!(0xFF, mem.fetch(0x200));
}

#[test]
fn test_fetch_graphics() {
    let mut mem = Mem::new();
    mem.graphics[67] = 255;

    assert_eq!(255, mem.fetch_graphics(3, 1));
}
