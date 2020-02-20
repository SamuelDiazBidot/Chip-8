use std::fs::File;
use std::io::Read;

pub struct CPU {
    pub memory: [u8; 4096],
    pub v: [u8; 16],
    pub pc: usize,
    pub i: u16,
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub graphics: [[u8; 64]; 32],
    pub stack: [u16; 16],
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

    pub fn load(&mut self, rom_data: &[u8]) {
        for (i, &byte) in rom_data.iter().enumerate() {
            self.memory[0x200 + i] = byte;
        }
    }

    pub fn emulate_cycle(&mut self) {
        let opcode: u16 = (self.memory[self.pc] as u16) << 8 | (self.memory[self.pc + 1] as u16);
        let nibbles = (
            (opcode & 0xF000) >> 12,
            (opcode & 0x0F00) >> 8, 
            (opcode & 0x00F0) >> 4,
            (opcode & 0x000F)
        );

        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;
        let n = (opcode & 0x000F) as usize;
        let nnn = ((opcode & 0x0FFF) as usize);
        let kk = ((opcode & 0x00FF) as u8);

        //TODO
        match nibbles {
            _ => ()
        }
    }
}