mod cpu;

use cpu::CPU;
use std::fs::File;
use std::io::Read;

struct EmulationState {
    cpu: CPU,
    rom: String,
}

impl EmulationState {
    fn new() -> Self {
        EmulationState {
            cpu: CPU::new(),
            rom: String::new(),
        }
    }

    fn load_rom(&mut self, location: &str) {
        let mut game = File::open(location).expect("Game was not found");
        let mut buffer = [0u8; 3584];
        game.read(&mut buffer[..]).expect("Error when reading file");
        self.cpu.load(&buffer);
    }
}

fn main() {
    println!("Hello, world!");
}
