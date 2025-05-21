mod cell;
mod grid;

use ggez::{
    Context, ContextBuilder, GameResult,
    event::{self, EventHandler},
    graphics::{self, Color, DrawMode, DrawParam, FillOptions, Mesh, Rect},
};
use std::time::{Duration, Instant};

use cell::CellType;
use grid::Grid;

const GRID_SIZE: (i16, i16) = (30, 30);
const GRID_CELL_SIZE: (i16, i16) = (32, 32);

const SCREEN_SIZE: (f32, f32) = (
    GRID_SIZE.0 as f32 * GRID_CELL_SIZE.0 as f32,
    GRID_SIZE.1 as f32 * GRID_CELL_SIZE.1 as f32,
);

const FPS: f32 = 8.0;

struct GameState {
    board: Grid,
    time_step: i64,
}

impl GameState {
    pub fn new() -> GameState {
        let mut board = Grid::new(GRID_SIZE.0 as u32, GRID_SIZE.1 as u32);
        board.init();

        GameState {
            board,
            time_step: 0,
        }
    }
}

impl EventHandler for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.board = self.board.get_next_state();
        self.time_step += 1;

        println!("Stepping.. current time step: {}", self.time_step);

        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context) -> GameResult {
        let frame_duration = Duration::from_secs_f32(1.0 / FPS);
        let start_time = Instant::now();

        let mut canvas = graphics::Canvas::from_frame(_ctx, Color::WHITE);

        for i in 0..self.board.cells.len() {
            let cur_cell = self.board.cells[i];

            if let CellType::LiveCell = cur_cell.cell_type {
                let (x, y) = cur_cell.get_coordinates();
                let rect = Rect::new(
                    x as f32 * GRID_CELL_SIZE.0 as f32,
                    y as f32 * GRID_CELL_SIZE.1 as f32,
                    GRID_CELL_SIZE.0 as f32,
                    GRID_CELL_SIZE.1 as f32,
                );

                let square = Mesh::new_rectangle(
                    _ctx,
                    DrawMode::Fill(FillOptions::default()),
                    rect,
                    Color::BLACK,
                )?;
                canvas.draw(&square, DrawParam::default());
            }
        }

        canvas.finish(_ctx)?;

        // Calculate the time taken to render the frame
        let elapsed = start_time.elapsed();

        // println!("Finished drawing");

        // Sleep the remaining time to achieve the target frame rate
        if elapsed < frame_duration {
            ggez::timer::sleep(frame_duration - elapsed);
        }

        Ok(())
    }
}

fn main() -> GameResult {
    let (ctx, event_loop) = ContextBuilder::new("game_of_life", "!Sure")
        .window_setup(ggez::conf::WindowSetup::default().title("Game of Life"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1))
        .build()?;

    let state = GameState::new();
    event::run(ctx, event_loop, state)
    // println!("Hello, world!");
}
