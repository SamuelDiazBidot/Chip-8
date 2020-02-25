mod cpu;

use cpu::CPU;
use std::fs::File;
use std::io::Read;
use ggez::{graphics, Context, ContextBuilder, GameResult};
use ggez::event::{self, EventHandler};

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
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        // self.cpu.emulate_cycle();
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
