pub struct CPU {
    pub memory: [u8; 4096],
    pub v: [u8; 16],
    pub pc: usize,
    pub i: u16,
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub graphics: [[u8; 64]; 32],
    pub stack: [u16; 16],
    pub sp: usize,
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
            (7,_,_,_) => {},
            //LD Vx, Vy (Set Vx = Vy)
            (8,_,_,0) => {},
            //OR Vx, Vy (Set Vx = Vx OR Vy)
            (8,_,_,1) => {},
            //AND Vx, Vy (Set Vx = Vx AND Vy)
            (8,_,_,2) => {},
            //XOR Vx, Vy (Set Vx = Vx XOR Vy)
            (8,_,_,3) => {},
            //ADD Vx, Vy (Set Vx = Vx + Vy, set VF = carry)
            (8,_,_,4) => {},
            //SUB Vx, Vy (Set Vx = Vx - Vy, set VF = NOT borrow)
            (8,_,_,5) => {},
            //SHR Vx {, Vy} (Set Vx = Vx SHR 1)
            (8,_,_,6) => {},
            //SUBN Vx, Vy (Set Vx = Vy - Vx, set VF = NOT borrow)
            (8,_,_,7) => {},
            //SHL Vx {, Vy} (Set Vx = Vx SHL 1)
            (8,_,_,0xE) => {},
            //SNE Vx, Vy (Skip next instruction if Vx != Vy)
            (9,_,_,0) => {},
            //LD I, addr (Set I = nnn)
            (0xA,_,_,_) => {},
            //JP V0, addr (Jump to location nnn + V0)
            (0xB,_,_,_) => {},
            //RND Vx, byte (Set Vx = random byte AND kk.)
            (0xC,_,_,_) => {},
            //DRW Vx, Vy, n (Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision)
            (0xD,_,_,_) => {},
            //SKP Vx (Skip next instruction if key with the value of Vx is pressed.)
            (0xE,_,9,0xE) => {},
            //SKNP Vx (Skip next instruction if key with the value of Vx is not pressed)
            (0xE,_,0xA,1) => {},
            //LD Vx, delay_timer (Set Vx = delay timer value)
            (0xF,_,0,7) => {},
            //LD Vx, key_press (Wait for a key press, store the value of the key in Vx)
            (0xF,_,0,0xA) => {},
            //LD delay_timer, Vx (Set delay timer = Vx)
            (0xF,_,1,5) => {},
            //LD sound_timer, Vx (ST is set equal to the value of Vx)
            (0xF,_,1,8) => {},
            //Add I, Vx (Set I = I + Vx)
            (0xF,_,1,0xE) => {},
            //LD F, Vx (Set I = location of sprite for digit Vx)
            (0xF,_,2,9) => {},
            //LD B, Vx (Store BCD representation of Vx in memory locations I, I+1, and I+2)
            (0xF,_,3,3) => {},
            //LD [I], Vx (Store registers V0 through Vx in memory starting at location I)
            (0xF,_,5,5) => {},
            //LD Vx, [I] (Read registers V0 through Vx from memory starting at location I)
            (0xF,_,6,5) => {},
            _ => ()
        }
    }
}