use rand;

pub struct CPU {
    pub memory: [u8; 4096],
    pub v: [u8; 16],
    pub pc: usize,
    pub i: usize,
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub graphics: [[u8; 64]; 32],
    pub stack: [u16; 16],
    pub sp: usize,
    pub keypad: [bool; 16],
    //temporary implementation
    pub keypad_waiting: bool,
    pub keypad_register: usize,
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
            keypad: [false; 16],
            keypad_waiting: false,
            keypad_register: 0,
        }
    }

    pub fn load(&mut self, rom_data: &[u8]) {
        for (i, &byte) in rom_data.iter().enumerate() {
            self.memory[0x200 + i] = byte;
        }
    }

    pub fn emulate_cycle(&mut self) {
        if self.keypad_waiting {
            for i in 0..16 {
                self.keypad_waiting = false;
                self.v[self.keypad_register] = i as u8;
                break;
            }
        } else {
            self.update_timers();
            self.run_opcode();
        }
    }

    fn update_timers(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }
    }

    fn run_opcode(&mut self) {
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
        let nnn = (opcode & 0x0FFF) as usize;
        let kk = (opcode & 0x00FF) as u8;

        //TODO
        match nibbles {
            //CLS (Clear the display)
            (0,0,0xE,0) => {
                self.graphics = [[0; 64]; 32];
                self.pc += 2;
            },
            //RET (Return from a subroutine)
            (0,0,0xE,0xE) => {
                self.pc = self.stack[self.sp] as usize;
                self.sp -= 1;
            },
            //JP addr (Jump to location nnn)
            (1,_,_,_) => {
                self.pc = nnn;
            },
            //CALL addr (Call to subroutine nnn)
            (2,_,_,_) => {
                self.sp += 1;
                self.stack[self.sp] = (self.pc + 2) as u16;
                self.pc = nnn;
            },
            //SE Vx, byte (Skip next instruction if Vx = kk)
            (3,_,_,_) => {
                if self.v[x] == kk {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            },
            //SNE Vx, byte (Skip next instruction if Vx != kk)
            (4,_,_,_) => {
                if self.v[x] != kk {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            },
            //SE Vx, Vy (Skip next instruction if Vx = Vy)
            (5,_,_,0) => {
                if self.v[x] == self.v[y] {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            },
            //LD Vx, byte (Set Vx = kk)
            (6,_,_,_) => {
                self.v[x] = kk;
                self.pc += 2;
            },
            //ADD Vx, byte (Set Vx = Vx + kk)
            (7,_,_,_) => {
                let vx = self.v[x] as u16;
                let kk = self.v[x] as u16;
                self.v[x] = (vx + kk) as u8;
                self.pc += 2;

            },
            //LD Vx, Vy (Set Vx = Vy)
            (8,_,_,0) => {
                self.v[x] = self.v[y];
                self.pc += 2;
            },
            //OR Vx, Vy (Set Vx = Vx OR Vy)
            (8,_,_,1) => {
                self.v[x] |= self.v[y];
                self.pc += 2;
            },
            //AND Vx, Vy (Set Vx = Vx AND Vy)
            (8,_,_,2) => {
                self.v[x] &= self.v[y];
                self.pc += 2;
            },
            //XOR Vx, Vy (Set Vx = Vx XOR Vy)
            (8,_,_,3) => {
                self.v[x] ^= self.v[y];
                self.pc += 2;
            },
            //ADD Vx, Vy (Set Vx = Vx + Vy, set VF = carry)
            (8,_,_,4) => {
                let vx = self.v[x] as u16;
                let vy = self.v[x] as u16;
                let result = vx + vy;
                self.v[0x0F] = ((result & 0x0F00) >> 8) as u8;
                self.v[x] = result as u8;
                self.pc += 2;
            },
            //SUB Vx, Vy (Set Vx = Vx - Vy, set VF = NOT borrow)
            (8,_,_,5) => {
                let vx = self.v[x] as i16;
                let vy = self.v[x] as i16;
                let result = vx - vy; 
                self.v[0x0F] = if self.v[x] > self.v[y] {1} else {0};
                self.v[x] = result as u8;
                self.pc += 2;
            },
            //SHR Vx {, Vy} (Set Vx = Vx SHR 1)
            (8,_,_,6) => {
                self.v[0x0F] = self.v[x] & 0b00000001;
                self.v[x] >>= 1;
                self.pc += 2;
            },
            //SUBN Vx, Vy (Set Vx = Vy - Vx, set VF = NOT borrow)
            (8,_,_,7) => {
                let vx = self.v[x] as i16;
                let vy = self.v[x] as i16;
                let result = vx - vy; 
                self.v[0x0F] = if self.v[y] > self.v[x] {1} else {0};
                self.v[x] = result as u8;
                self.pc += 2;
            },
            //SHL Vx {, Vy} (Set Vx = Vx SHL 1)
            (8,_,_,0xE) => {
                self.v[0x0F] = (self.v[x] & 0b10000000) >> 7;
                self.v[x] <<= 1;
                self.pc += 2;
            },
            //SNE Vx, Vy (Skip next instruction if Vx != Vy)
            (9,_,_,0) => {
                if self.v[x] != self.v[y] {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            },
            //LD I, addr (Set I = nnn)
            (0xA,_,_,_) => {
                self.i = nnn;
                self.pc += 2;
            },
            //JP V0, addr (Jump to location nnn + V0)
            (0xB,_,_,_) => {
                self.pc = nnn + (self.v[0x00] as usize);
            },
            //RND Vx, byte (Set Vx = random byte AND kk.)
            (0xC,_,_,_) => {
                self.v[x] = rand::random::<u8>() & kk;
                self.pc += 2;
            },
            //DRW Vx, Vy, n (Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision)
            // Copied from https://github.com/starrhorne/chip8-rust
            // I still dont understand how to implement this
            (0xD,_,_,_) => {
                self.v[0x0F] = 0;
                for byte in 0..n {
                    let y = (self.v[y] as usize + byte) % 32;
                    for bit in 0..8 {
                        let x = (self.v[y] as usize + bit) % 64;
                        let color = (self.memory[self.i + byte] >> (7 - bit)) & 1;
                        self.v[0x0F] |= color & self.graphics[y][x];
                        self.graphics[y][x] ^= color;
                    }
                }
            },
            //SKP Vx (Skip next instruction if key with the value of Vx is pressed.)
            (0xE,_,9,0xE) => {
                let key = self.v[x] as usize;
                if self.keypad[key] {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            },
            //SKNP Vx (Skip next instruction if key with the value of Vx is not pressed)
            (0xE,_,0xA,1) => {
                let key = self.v[x] as usize;
                if !self.keypad[key] {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            },
            //LD Vx, delay_timer (Set Vx = delay timer value)
            (0xF,_,0,7) => {
                self.v[x] = self.delay_timer;
                self.pc += 2;
            },
            //LD Vx, key_press (Wait for a key press, store the value of the key in Vx)
            (0xF,_,0,0xA) => {
                // if let Some(val) = get_key_press() {
                //      self.v[x] = val;
                //      self.pc += 2;
                //
                self.keypad_waiting = true;
                self.keypad_register = x;
                self.pc += 2;
            },
            //LD delay_timer, Vx (Set delay timer = Vx)
            (0xF,_,1,5) => {
                self.delay_timer = self.v[x];
                self.pc += 2;
            },
            //LD sound_timer, Vx (ST is set equal to the value of Vx)
            (0xF,_,1,8) => {
                self.sound_timer = self.v[x];
                self.pc += 2;
            },
            //Add I, Vx (Set I = I + Vx)
            (0xF,_,1,0xE) => {
                self.i += self.v[x] as usize;
                self.pc += 2;
            },
            //LD F, Vx (Set I = location of sprite for digit Vx) Multiply by 5 because each sprite has 5 lines
            (0xF,_,2,9) => {
                self.i = (self.v[x] as usize) * 5;
                self.pc += 2;
            },
            //LD B, Vx (Store BCD representation of Vx in memory locations I, I+1, and I+2)
            (0xF,_,3,3) => {
                self.memory[self.i] = self.v[x] / 100;
                self.memory[self.i + 1] = (self.v[x] % 100) / 10;
                self.memory[self.i + 2] = self.v[x] % 10;
                self.pc += 2;
            },
            //LD [I], Vx (Store registers V0 through Vx in memory starting at location I)
            (0xF,_,5,5) => {
                for index in 0..(x+1) {
                    self.memory[self.i + index] = self.v[index];
                }
                self.pc += 2;
            },
            //LD Vx, [I] (Read registers V0 through Vx from memory starting at location I)
            (0xF,_,6,5) => {
                for index in 0..(x+1) {
                    self.v[index] = self.memory[self.i + index];
                }
                self.pc += 2;
            },
            _ => { 
                self.pc += 2; 
            },
        }
    }
}