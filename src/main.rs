mod cpu;

use cpu::CPU;
use std::fs::File;
use std::io::Read;
use ggez::{graphics, Context, ContextBuilder, GameResult};
use ggez::event::{self, EventHandler, KeyCode};
use ggez::input::keyboard;

const GRID_SIZE: (u16, u16) = (64,32);
const GRID_CELL_SIZE: u16 = 20;
const SCREEN_SIZE: (f32, f32) = (
    GRID_CELL_SIZE as f32 * GRID_SIZE.0 as f32,
    GRID_CELL_SIZE as f32 * GRID_SIZE.1 as f32
);

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

impl EventHandler for EmulationState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        if keyboard::is_key_pressed(ctx, KeyCode::Key1) {
            //key 1 is pressed
            self.cpu.keypad[1] = true;
        } else if keyboard::is_key_pressed(ctx, KeyCode::Key2) {
            //Key 2 is pressed
            self.cpu.keypad[2] = true;
        } else if keyboard::is_key_pressed(ctx, KeyCode::Key3) {
            //key 3 is pressed
            self.cpu.keypad[3] = true;
        } else if keyboard::is_key_pressed(ctx, KeyCode::Key4) {
            //key C is pressed
            self.cpu.keypad[12] = true;
        } else if keyboard::is_key_pressed(ctx, KeyCode::Q) {
            //key 4 is pressed
            self.cpu.keypad[4] = true;
        } else if keyboard::is_key_pressed(ctx, KeyCode::W) {
            //key 5 is pressed
            self.cpu.keypad[5] = true;
        } else if keyboard::is_key_pressed(ctx, KeyCode::E) {
            //key 6 is pressed
            self.cpu.keypad[6] = true;
        } else if keyboard::is_key_pressed(ctx, KeyCode::R) {
            //Key D is pressed
            self.cpu.keypad[13] = true;
        } else if keyboard::is_key_pressed(ctx, KeyCode::A) {
            //key 7 is pressed
            self.cpu.keypad[7] = true;
        } else if keyboard::is_key_pressed(ctx, KeyCode::S) {
            //key 8 is pressed
            self.cpu.keypad[8] = true;
        } else if keyboard::is_key_pressed(ctx, KeyCode::D) {
            //key 9 is pressed
            self.cpu.keypad[9] = true;
        } else if keyboard::is_key_pressed(ctx, KeyCode::F) {
            //key E is pressed
            self.cpu.keypad[14] = true;
        } else if keyboard::is_key_pressed(ctx, KeyCode::Z) {
            //key A is pressed
            self.cpu.keypad[10] = true;
        } else if keyboard::is_key_pressed(ctx, KeyCode::X) {
            //key 0 is pressed
            self.cpu.keypad[0] = true;
        } else if keyboard::is_key_pressed(ctx, KeyCode::C) {
            //key B is pressed
            self.cpu.keypad[11] = true;
        } else if keyboard::is_key_pressed(ctx, KeyCode::V) {
            //key F is pressed
            self.cpu.keypad[15] = true;
        } else if keyboard::is_key_pressed(ctx, KeyCode::Escape){
            event::quit(ctx);
        } else {
            self.cpu.keypad = [false; 16];
        }
        self.cpu.emulate_cycle();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let set_color = |y: u8| if y == 0 { graphics::BLACK } else { graphics::WHITE };
        for y in 0..32 {
            for x in 0..64 {
                let rectangle = graphics::Mesh::new_rectangle(
                    ctx, 
                    graphics::DrawMode::fill(),
                    graphics::Rect::new((x * GRID_CELL_SIZE) as f32, (y * GRID_CELL_SIZE) as f32, GRID_CELL_SIZE as f32, GRID_CELL_SIZE as f32),
                    set_color(self.cpu.graphics[y as usize][x as usize])
                )?;
                graphics::draw(ctx, &rectangle,(ggez::mint::Point2 { x: 0.0, y: 0.0 },))?;
            }
        }
        graphics::present(ctx)?;
        Ok(())
    }
}

fn main() {
    let (ctx, events_loop) = &mut ggez::ContextBuilder::new("Chip-8", "Sammy")
        .window_setup(ggez::conf::WindowSetup::default().title("Chip-8"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1))
        .build().unwrap();

    let state = &mut EmulationState::new();
    event::run(ctx, events_loop, state).unwrap();

    
}
