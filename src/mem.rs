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

pub struct Mem {
    memory: [u8; MEMORY_SIZE],
    stack: [u16; STACK_SIZE],
    graphics: [[u8; GRAPHICS_HEIGHT]; GRAPHICS_WIDTH],
    stack_pointer: usize,
}
//unsigned char gfx[64 * 32];
impl Mem {
    pub fn new() -> Mem {
        Mem {
            memory: [0; MEMORY_SIZE],
            stack: [0; STACK_SIZE],
            graphics: [[0; GRAPHICS_HEIGHT]; GRAPHICS_WIDTH],
            stack_pointer: 0,
        }
    }

    pub fn load_program(&mut self, program: &[u8]) {
        if PROGRAM_MEMORY_START + program.len() > MEMORY_SIZE {
            panic!("Program is too large to fit in memory");
        }

        self.memory[PROGRAM_MEMORY_START..PROGRAM_MEMORY_START + program.len()]
            .clone_from_slice(&program);
    }

    pub fn fetch_opcode(&self, index: usize) -> u16 {
        (self.memory[index] as u16) << 8 | (self.memory[index + 1]) as u16
    }

    pub fn fetch(&self, index: usize) -> u8 {
        self.memory[index]
    }

    pub fn fetch_graphics(&self, x: usize, y: usize) -> u8 {
        self.graphics[x][y]
    }

    pub fn store_graphics(&mut self, x: usize, y: usize, val: u8) {
        self.graphics[x][y] = val;
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

    mem.load_program(&program);
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
