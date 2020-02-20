use std::fs::File;
use std::io::Read;

pub struct CPU {
    pub memory: [u8; 4096],
    pub v: [u8; 16],
    pub pc: u16;
    pub i: u16,
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub graphics: [[u8; 64]; 32],
    pub stack: [u16; 16];
    pub sp: u16,
    pub keypad: [u8; 16],
}

impl CPU {
    pub fn new() -> Self {
        let mut memory: [u8; 4096] = [0; 4096];
        let font_set: [u8; 80] = [
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
            0xF0, 0x80, 0xF0, 0x80, 0x80  // F
        ];

        for i in 0..font_set.len() {
            memory[i] = font_set[i];
        }

        CPU {
            memory: memory,
            v: [0; 16],
            pc: 0x200,
            i: 0,
            delay_timer: 0,
            sound_timer: 0,
            graphics: [[0; 64]; 32],
            stack: [0; 16],
            sp: 0,
            keypad: [0; 16],
        }
    }

    pub fn load(&mut self, location: &str) {
        let mut game = File::open(location).expect("Game was not found");
        let mut buffer = [0; 3584];
        let buffer_size = game.read(&mut buffer[..]).expect("Error when reading file");
        
        for i in 0..buffer_size {
            self.memory[0x200 + i] = buffer[i];
        }
    }
}