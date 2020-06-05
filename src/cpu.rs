#![allow(arithmetic_overflow)]

use crate::mem;

const REGISTER_COUNT: usize = 16;

pub struct Cpu {
    registers: [u8; REGISTER_COUNT],
    index: u16,
    program_counter: u16,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            registers: [0; REGISTER_COUNT],
            index: 0,
            program_counter: 0x200,
        }
    }

    pub fn increase_program_counter(&mut self, count: u16) {
        self.program_counter += count;
    }

    pub fn set_register_value(&mut self, register_index: usize, value: u8) {
        if register_index > REGISTER_COUNT {
            panic!(
                "Cannot set register value on index above {}",
                REGISTER_COUNT - 1
            );
        }
        self.registers[register_index] = value;
    }

    pub fn execute_cycle(&mut self, mem: &mut mem::Mem) {
        let opcode: u16 = mem.fetch_opcode(self.program_counter as usize);

        match opcode & 0xF000 {
            0x0000 => {
                match opcode & 0x000f {
                    0x000 => println!("clear screen"), // clear screen,
                    0x00e => println!("return"),       // exit subroutine
                    x => println!("Unrecognized opcode {}", x),
                }
            }

            0x1000 => {
                // jump to
                self.program_counter = opcode & 0x0FFF;
            }
            0x2000 => {
                // call subroutine at
                mem.push(self.program_counter);
                self.program_counter = opcode & 0x0FFF;
            }
            0x3000 => {
                // skip next if reg index by 2nd byte is eq to 3rd and 4th byte
                let register_value = self.registers[(opcode & 0x0F00) as usize];
                let value = (opcode & 0x00FF) as u8;

                if value == register_value {
                    self.increase_program_counter(4);
                } else {
                    self.increase_program_counter(2);
                }
            }
            0x4000 => {
                // skip next if reg index by 2nd byte is neq to 3rd and 4th byte
                let register_value = self.registers[(opcode & 0x0F00) as usize];
                let value = (opcode & 0x00FF) as u8;
                if value != register_value {
                    self.increase_program_counter(4);
                } else {
                    self.increase_program_counter(2);
                }
            }
            0x5000 => {
                // skip next if reg at index 2nd byte is eq 3rd byte
                let register_value1 = self.registers[(opcode >> 8 & 0x0F) as usize];
                let register_value2 = self.registers[(opcode >> 4 & 0x0F) as usize];
                if register_value1 == register_value2 {
                    self.increase_program_counter(4);
                } else {
                    self.increase_program_counter(2)
                }
            }
            0x6000 => {
                // set register at byte 2 to the value of 3 and 4.
                let register_index = (opcode >> 8 & 0x0F) as usize;
                let value = (opcode & 0x00FF) as u8;
                self.set_register_value(register_index, value);
                self.increase_program_counter(2)
            }
            0x7000 => {
                // vx = vx + byte 3 and, carry not modified
                let register_index = (opcode >> 8 & 0x0F) as usize;
                let register_value = self.registers[register_index];
                let value = (opcode & 0x00FF) as u8;
                self.set_register_value(register_index, register_value + value);
                self.increase_program_counter(2);
            }
            0x8000 => {
                let register_index_x = (opcode >> 8 & 0x0F) as usize;
                let register_index_y = (opcode >> 4 & 0x0F) as usize;
                match opcode & 0x000F {
                    0x0000 => {
                        // vx = vy
                        self.set_register_value(register_index_x, self.registers[register_index_y]);
                    }
                    0x0001 => self.set_register_value(
                        // vx = vx OR vy
                        register_index_x,
                        self.registers[register_index_x] | self.registers[register_index_y],
                    ),
                    0x0002 => self.set_register_value(
                        // vx = vx AND vy
                        register_index_x,
                        self.registers[register_index_x] & self.registers[register_index_y],
                    ),
                    0x0003 => self.set_register_value(
                        // vx = vx XOR vy
                        register_index_x,
                        self.registers[register_index_x] ^ self.registers[register_index_y],
                    ),
                    0x0004 => {
                        // vx = vx - vy, carry is set if overflow
                        let result: u32 = self.registers[register_index_x] as u32
                            + self.registers[register_index_y] as u32;
                        let carry = result > 0xFF;
                        self.set_register_value(register_index_x, (result & 0xFF) as u8);
                        self.set_register_value(0xF, carry as u8);
                    }
                    0x0005 => {
                        // vx = vx - vy, carry is 1 if borrow
                        let result: i32 = self.registers[register_index_x] as i32
                            - self.registers[register_index_y] as i32;
                        let carry = result < 0;
                        self.set_register_value(register_index_x, (result & 0xFF) as u8);

                        self.set_register_value(0xF, carry as u8);
                    }
                    0x0006 => {
                        // rsf vx, set carry if lsb is 1
                        self.set_register_value(15, self.registers[register_index_x] & 0x01);
                        self.set_register_value(
                            register_index_x,
                            self.registers[register_index_x] >> 1,
                        );
                    }
                    0x0007 => {
                        // vx = vy - vx, set carry if borrow
                        let result: i32 = self.registers[register_index_y] as i32
                            - self.registers[register_index_x] as i32;
                        let carry = result < 0;
                        self.set_register_value(register_index_x, (result & 0xFF) as u8);
                        self.set_register_value(0xF, carry as u8);
                    }
                    0x000E => {
                        // lsf vx, set carry if msb is 1
                        self.set_register_value(15, (self.registers[register_index_x] & 0x80) >> 7);
                        self.set_register_value(
                            register_index_x,
                            self.registers[register_index_x] << 1,
                        );
                    }
                    x => {
                        println!("Unknown subcode {}", x);
                    }
                }
                self.increase_program_counter(2);
            }
            0x9000 => {
                // skip next instruction if vx == vy
                let register_index_x = (opcode >> 8 & 0x0F) as usize;
                let register_index_y = (opcode >> 4 & 0x0F) as usize;
                if self.registers[register_index_x] == self.registers[register_index_y] {
                    self.increase_program_counter(4);
                } else {
                    self.increase_program_counter(2);
                }
            }
            0xA000 => {
                // set index to byte 2,3,4
                self.index = opcode & 0x0FFF;
                self.increase_program_counter(2);
            }
            0xB000 => {
                // jump to byte 2,3,4
                let address = opcode & 0x0FFF;
                self.program_counter = address + self.registers[0] as u16;
            }
            0xC000 => {
                // set vx to rand with AND from byte 3,4
                let rand = rand::random::<u8>();
                let register_index_x = (opcode >> 8 & 0x0F) as usize;
                self.registers[register_index_x] = rand & (opcode & 0x00FF) as u8
            }
            0xD000 => { 
                let register_index_x = (opcode >> 8 & 0x0F) as usize;
                let register_index_y = (opcode >> 4 & 0x0F) as usize;
                let x = self.registers[register_index_x];
                let y = self.registers[register_index_y];
                let height: u8 = (opcode & 0x0F) as u8;
                let mut pixel: u8;
                
                self.registers[15] = 0;
                for yline in 0..height {
                    pixel = mem.fetch((self.index + yline as u16) as usize);
                    for xline in 0..8 {
                        let x_coord_index = (x as u16 + xline) as usize;
                        let y_coord_index = (y as u16 + yline as u16) as usize;
                        if (pixel & (0x80 >> xline)) != 0 {
                            let current_pixel = mem.fetch_graphics(x_coord_index, y_coord_index);
                            if current_pixel == 1 {
                                self.registers[0xF] = 1;
                            }
                            mem.store_graphics(x_coord_index, y_coord_index, current_pixel  ^ 0x01)
                        }
                    }
                }
                self.increase_program_counter(2);
            }
            x => println!("Unrecognized opcode {}", x),
        }
    }
}

impl Default for Cpu {
    fn default() -> Self {
        Self::new()
    }
}

#[test]
fn test_increase_program_counter() {
    let mut cpu = Cpu::new();
    let current_pc = cpu.program_counter;

    cpu.increase_program_counter(1);
    assert_eq!(current_pc + 1, cpu.program_counter);
}

#[test]
fn test_execute_cycle_0xa0ff() {
    let mut cpu = Cpu::new();
    let mut mem = mem::Mem::new();

    mem.load_program(&[0xA0, 0xFF]);
    cpu.execute_cycle(&mut mem);

    assert_eq!(0x00FF, cpu.index);
}

#[test]
fn test_execute_cycle_0x2xxx() {
    let mut cpu = Cpu::new();
    let mut mem = mem::Mem::new();

    mem.load_program(&[0x20, 0x01]);
    cpu.execute_cycle(&mut mem);

    assert_eq!(0x0001, cpu.program_counter);
}

#[test]
fn test_execute_cycle_0x3xxx_equal() {
    let mut cpu = Cpu::new();
    let mut mem = mem::Mem::new();

    mem.load_program(&[0x30, 0x00]);

    cpu.execute_cycle(&mut mem);

    assert_eq!(0x204, cpu.program_counter);
}

#[test]
fn test_execute_cycle_0x3xxx_not_equal() {
    let mut cpu = Cpu::new();
    let mut mem = mem::Mem::new();

    mem.load_program(&[0x30, 0x01]);

    cpu.execute_cycle(&mut mem);

    assert_eq!(0x202, cpu.program_counter);
}

#[test]
fn test_execute_cycle_0x4xxx_equal() {
    let mut cpu = Cpu::new();
    let mut mem = mem::Mem::new();

    mem.load_program(&[0x40, 0x00]);

    cpu.execute_cycle(&mut mem);

    assert_eq!(0x202, cpu.program_counter);
}

#[test]
fn test_execute_cycle_0x4xxx_not_equal() {
    let mut cpu = Cpu::new();
    let mut mem = mem::Mem::new();

    mem.load_program(&[0x40, 0x01]);

    cpu.execute_cycle(&mut mem);

    assert_eq!(0x204, cpu.program_counter);
}

#[test]
fn test_execute_cycle_0x5xxx_equal() {
    let mut cpu = Cpu::new();
    let mut mem = mem::Mem::new();

    mem.load_program(&[0x51, 0x20]);
    cpu.set_register_value(1, 0x01);
    cpu.set_register_value(2, 0x01);

    cpu.execute_cycle(&mut mem);

    assert_eq!(0x204, cpu.program_counter);
}

#[test]
fn test_execute_cycle_0x5xxx_not_equal() {
    let mut cpu = Cpu::new();
    let mut mem = mem::Mem::new();

    mem.load_program(&[0x51, 0x20]);
    cpu.set_register_value(1, 0x01);
    cpu.set_register_value(2, 0x02);

    cpu.execute_cycle(&mut mem);

    assert_eq!(0x202, cpu.program_counter);
}

#[test]
fn test_execute_cycle_0x6xxx() {
    let mut cpu = Cpu::new();
    let mut mem = mem::Mem::new();

    mem.load_program(&[0x61, 0x01]);

    cpu.execute_cycle(&mut mem);

    assert_eq!(0x01, cpu.registers[1]);
}

#[test]
fn test_execute_cycle_0x7xxx() {
    let mut cpu = Cpu::new();
    let mut mem = mem::Mem::new();

    cpu.set_register_value(1, 0x10);

    mem.load_program(&[0x71, 0x01]);

    cpu.execute_cycle(&mut mem);

    assert_eq!(0x11, cpu.registers[1]);
}

#[test]
fn test_execute_cycle_0x8000() {
    let mut cpu = Cpu::new();
    let mut mem = mem::Mem::new();

    cpu.set_register_value(2, 0x10);

    mem.load_program(&[0x81, 0x20]);

    cpu.execute_cycle(&mut mem);

    assert_eq!(0x10, cpu.registers[1]);
}

#[test]
fn test_execute_cycle_0x8001() {
    let mut cpu = Cpu::new();
    let mut mem = mem::Mem::new();

    cpu.set_register_value(1, 0x01);
    cpu.set_register_value(2, 0x10);
    mem.load_program(&[0x81, 0x21]);

    cpu.execute_cycle(&mut mem);
    assert_eq!(0x11, cpu.registers[1]);
}

#[test]
fn test_execute_cycle_0x8002() {
    let mut cpu = Cpu::new();
    let mut mem = mem::Mem::new();

    cpu.set_register_value(1, 0x01);
    cpu.set_register_value(2, 0x10);
    mem.load_program(&[0x81, 0x22]);

    cpu.execute_cycle(&mut mem);
    assert_eq!(0x00, cpu.registers[1]);
}

#[test]
fn test_execute_cycle_0x8003() {
    let mut cpu = Cpu::new();
    let mut mem = mem::Mem::new();

    cpu.set_register_value(1, 0x01);
    cpu.set_register_value(2, 0x11);
    mem.load_program(&[0x81, 0x23]);

    cpu.execute_cycle(&mut mem);
    assert_eq!(0x10, cpu.registers[1]);
}

#[test]
fn test_execute_cycle_0x8004_overflow() {
    let mut cpu = Cpu::new();
    let mut mem = mem::Mem::new();

    cpu.set_register_value(1, 0xFF);
    cpu.set_register_value(2, 0xFF);
    mem.load_program(&[0x81, 0x24]);

    cpu.execute_cycle(&mut mem);
    assert_eq!(0xFE, cpu.registers[1]);
    assert_eq!(0x01, cpu.registers[15]);
}

#[test]
fn test_execute_cycle_0x8004_no_overflow() {
    let mut cpu = Cpu::new();
    let mut mem = mem::Mem::new();

    cpu.set_register_value(1, 0x01);
    cpu.set_register_value(2, 0x01);
    mem.load_program(&[0x81, 0x24]);

    cpu.execute_cycle(&mut mem);
    assert_eq!(0x02, cpu.registers[1]);
    assert_eq!(0x00, cpu.registers[15]);
}

#[test]
fn test_execute_cycle_0x8005_borrow() {
    let mut cpu = Cpu::new();
    let mut mem = mem::Mem::new();

    cpu.set_register_value(1, 0x00);
    cpu.set_register_value(2, 0xFF);
    mem.load_program(&[0x81, 0x25]);

    cpu.execute_cycle(&mut mem);
    assert_eq!(0x01, cpu.registers[1]);
    assert_eq!(0x01, cpu.registers[15]);
}

#[test]
fn test_execute_cycle_0x8005_no_borrow() {
    let mut cpu = Cpu::new();
    let mut mem = mem::Mem::new();

    cpu.set_register_value(1, 0xFF);
    cpu.set_register_value(2, 0x01);
    mem.load_program(&[0x81, 0x25]);

    cpu.execute_cycle(&mut mem);
    assert_eq!(0xFE, cpu.registers[1]);
    assert_eq!(0x00, cpu.registers[15]);
}

#[test]
fn test_execute_cycle_0x8006_lsb_0() {
    let mut cpu = Cpu::new();
    let mut mem = mem::Mem::new();

    cpu.set_register_value(1, 0xF0);
    mem.load_program(&[0x81, 0x06]);

    cpu.execute_cycle(&mut mem);
    assert_eq!(0x78, cpu.registers[1]);
    assert_eq!(0x00, cpu.registers[15]);
}

#[test]
fn test_execute_cycle_0x8006_lsb_1() {
    let mut cpu = Cpu::new();
    let mut mem = mem::Mem::new();

    cpu.set_register_value(1, 0xF1);
    mem.load_program(&[0x81, 0x06]);

    cpu.execute_cycle(&mut mem);
    assert_eq!(0x78, cpu.registers[1]);
    assert_eq!(0x01, cpu.registers[15]);
}

#[test]
fn test_execute_cycle_0x8007_borrow() {
    let mut cpu = Cpu::new();
    let mut mem = mem::Mem::new();

    cpu.set_register_value(1, 0xFF);
    cpu.set_register_value(2, 0x00);
    mem.load_program(&[0x81, 0x27]);

    cpu.execute_cycle(&mut mem);
    assert_eq!(0x01, cpu.registers[1]);
    assert_eq!(0x01, cpu.registers[15]);
}

#[test]
fn test_execute_cycle_0x8007_no_borrow() {
    let mut cpu = Cpu::new();
    let mut mem = mem::Mem::new();

    cpu.set_register_value(1, 0x00);
    cpu.set_register_value(2, 0xFF);
    mem.load_program(&[0x81, 0x27]);

    cpu.execute_cycle(&mut mem);
    assert_eq!(0xFF, cpu.registers[1]);
    assert_eq!(0x00, cpu.registers[15]);
}

#[test]
fn test_execute_cycle_0x800e_msb_0() {
    let mut cpu = Cpu::new();
    let mut mem = mem::Mem::new();

    cpu.set_register_value(1, 0x70);
    mem.load_program(&[0x81, 0x0E]);

    cpu.execute_cycle(&mut mem);
    assert_eq!(0xE0, cpu.registers[1]);
    assert_eq!(0x00, cpu.registers[15]);
}

#[test]
fn test_execute_cycle_0x800e_msb_1() {
    let mut cpu = Cpu::new();
    let mut mem = mem::Mem::new();

    cpu.set_register_value(1, 0xFF);
    mem.load_program(&[0x81, 0x0E]);

    cpu.execute_cycle(&mut mem);
    assert_eq!(0xFE, cpu.registers[1]);
    assert_eq!(0x01, cpu.registers[15]);
}

#[test]
fn test_execute_cycle_0x9000_equal() {
    let mut cpu = Cpu::new();
    let mut mem = mem::Mem::new();

    cpu.set_register_value(1, 0xFF);
    cpu.set_register_value(2, 0xFF);
    mem.load_program(&[0x91, 0x20]);
    cpu.execute_cycle(&mut mem);

    assert_eq!(0x204, cpu.program_counter);
}

#[test]
fn test_execute_cycle_0x9000_not_equal() {
    let mut cpu = Cpu::new();
    let mut mem = mem::Mem::new();

    cpu.set_register_value(1, 0x01);
    cpu.set_register_value(2, 0xFF);
    mem.load_program(&[0x91, 0x20]);
    cpu.execute_cycle(&mut mem);

    assert_eq!(0x202, cpu.program_counter);
}

#[test]
fn test_execute_cycle_0xb000() {
    let mut cpu = Cpu::new();
    let mut mem = mem::Mem::new();

    cpu.set_register_value(0, 0xFF);
    cpu.set_register_value(2, 0xFF);
    mem.load_program(&[0xB0, 0x00]);
    cpu.execute_cycle(&mut mem);

    assert_eq!(0xFF, cpu.program_counter);
}

#[test]
fn test_execute_cycle_0xc000() {
    let mut cpu = Cpu::new();
    let mut mem = mem::Mem::new();

    cpu.set_register_value(1, 0xFF);
    cpu.set_register_value(2, 0xFF);
    mem.load_program(&[0xC1, 0x00]);
    cpu.execute_cycle(&mut mem);

    assert_eq!(0x00, cpu.registers[0]);
}
