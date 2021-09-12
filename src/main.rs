pub mod chip8;
pub mod cpu;
pub mod mem;

extern crate minifb;

use std::{fs::File, io::Read};
use std::collections::HashMap;
use minifb::{Key, Window, WindowOptions, Scale};
use simple_logger::SimpleLogger;
use log::LevelFilter;

const WIDTH: usize = 64;
const HEIGHT: usize = 64;

fn update_keymap(mapping: &HashMap<Key, usize>, window: &Window, key_map: &mut [bool; 16]) {
    for (key, keymap_index) in mapping.iter() {
        key_map[*keymap_index] = if window.is_key_down(*key) {
            true
        } else {
            false
        }
    }
}

fn main() {
    SimpleLogger::new().with_level(LevelFilter::Error).init().unwrap();

    let mut key_map: HashMap<Key, usize> = HashMap::new();
    key_map.insert(Key::W, 5);
    key_map.insert(Key::A, 7);
    key_map.insert(Key::S, 8);
    key_map.insert(Key::D, 9);

    let mut chip8 = chip8::Chip8::new();
    let mut keypad = [false; 16];

    let mut file = File::open("snake.ch8").expect("Could not open file");

    let mut buffer:Vec<u8> = Vec::new();
    file.read_to_end(&mut buffer).expect("Could not read file");

    chip8.load_program(&buffer).expect("Could not load program");
    
    let mut window_options = WindowOptions::default();
    window_options.scale = Scale::X8;

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        window_options,
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        update_keymap(&key_map, &window, &mut keypad);

        chip8.run_cycle(&keypad);
        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&chip8.mem.graphics, WIDTH, HEIGHT)
            .unwrap();
    }
}
