use std::time::{Instant, Duration};

use rand::Rng;
use raylib::prelude::*;

const GRID_WIDTH: i32 = 10;
const GRID_HEIGHT: i32 = 20;
const CELL_SIZE: i32 = 30;
const GRID_OFFSET: Vector2 = Vector2 { x: 100.0, y: 50.0 };
const COLOR_BG: Color = Color::BLACK;
const COLOR_GRID: Color = Color::WHITE;
const COLOR_PIECE: Color = Color::RED;


struct TetrisGame {
    grid: [[bool; GRID_HEIGHT as usize]; GRID_WIDTH as usize],
    current_piece: Tetrimino,
    current_piece_position: (i32, i32),
    current_piece_rotation: i32,
    last_fall_time: Instant,
    fall_interval: Duration,
    score: u32,
    last_key_pressed: Option<KeyboardKey>,
}


impl Tetrimino {
    fn get_shape(&self) -> &'static [[bool; 4]; 4] {
        match self {
            Tetrimino::I => &[
                [false, true, false, false],
                [false, true, false, false],
                [false, true, false, false],
                [false, true, false, false],
            ],
            Tetrimino::O => &[
                [false, false, false, false],
                [false, false, false, false],
                [true, true, false, false],
                [true, true, false, false],
            ],
            Tetrimino::T => &[
                [false, false, false, false],
                [false, false, false, false],
                [false, true, false, false],
                [true, true, true, false],
            ],
            Tetrimino::J => &[
                [false, false, false, false],
                [false, true, false, false],
                [false, true, false, false],
                [true, true, false, false],
            ],
            Tetrimino::L => &[
                [false, false, false, false],
                [false, true, false, false],
                [false, true, false, false],
                [false, true, true, false],
            ],
            Tetrimino::S => &[
                [false, false, false, false],
                [false, false, false, false],
                [false, true, true, false],
                [true, true, false, false],
            ],
            Tetrimino::Z => &[
                [false, false, false, false],
                [false, false, false, false],
                [true, true, false, false],
                [false, true, true, false],
            ],
            // Add other tetrimino shapes here
        }
    }

    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..7) {
            0 => Tetrimino::I,
            1 => Tetrimino::O,
            2 => Tetrimino::T,
            3 => Tetrimino::J,
            4 => Tetrimino::L,
            5 => Tetrimino::S,
            6 => Tetrimino::Z,
            _ => unreachable!(),
        }
    }

    fn get_rotated_shape(&self, rotation: i32) -> [[bool; 4]; 4] {
        let mut shape = self.get_shape().clone();
        for _ in 0..(rotation % 4) {
            shape = rotate_shape_right(shape);
        }
        shape
    }
}

fn rotate_shape_right(shape: [[bool; 4]; 4]) -> [[bool; 4]; 4] {
    let mut new_shape = shape.clone();
    for i in 0..4 {
        for j in 0..4 {
            new_shape[j][3 - i] = shape[i][j];
        }
    }
    new_shape
}



impl TetrisGame {
    pub fn new() -> Self {
        TetrisGame {
            grid: [[false; GRID_HEIGHT as usize]; GRID_WIDTH as usize],
            current_piece: Tetrimino::random(),
            current_piece_position: (GRID_WIDTH / 2 - 2, 0),
            current_piece_rotation: 0,
            last_fall_time: Instant::now(),
            fall_interval: Duration::from_secs(1),
            score: 0,
            last_key_pressed: None,
            }
    }

    pub fn update(&mut self) {
        let (mut dx, mut dy, mut dr) = (0, 0, 0);
    
        if let Some(key) = self.last_key_pressed {
            match key {
                KeyboardKey::KEY_LEFT => dx -= 1,
                KeyboardKey::KEY_RIGHT => dx += 1,
                KeyboardKey::KEY_DOWN => dy += 1,
                KeyboardKey::KEY_UP => dr += 1,
                _ => (),
            }
        }

        if dx != 0 || dr != 0 || dy != 0 || self.last_fall_time.elapsed() >= self.fall_interval {
            let previous_position = self.current_piece_position;
            let previous_rotation = self.current_piece_rotation;
            self.current_piece_position.0 += dx;
            self.current_piece_position.1 += dy;
            self.current_piece_rotation = (self.current_piece_rotation + dr) % 4;

            if self.is_collision() {
                self.current_piece_position = previous_position;
                self.current_piece_rotation = previous_rotation;
            }
            if self.last_fall_time.elapsed() >= self.fall_interval {
                self.current_piece_position.1 += 1;
                if self.is_collision() {
                    self.current_piece_position.1 -= 1;
                    self.lock_piece();
                    self.clear_lines();
                    self.spawn_new_piece();
                    if self.is_collision() {
                        self.game_over();
                    }
                }
                self.last_fall_time = Instant::now();
            }
            self.last_fall_time = Instant::now();
        }
        
    }

    

    fn spawn_new_piece(&mut self) {
        self.current_piece = Tetrimino::random();
        self.current_piece_position = (GRID_WIDTH / 2 - 2, 0);
        self.current_piece_rotation = 0;
    }
    
    fn game_over(&mut self) {
        println!("Game Over!\nYour score: {}", self.score);

        self.grid = [[false; GRID_HEIGHT as usize]; GRID_WIDTH as usize];
        self.score = 0;
    }

    fn clear_lines(&mut self) {
        let mut lines_cleared = 0;
    
        for y in (0..GRID_HEIGHT).rev() {
            let mut full_line = true;
            for x in 0..GRID_WIDTH {
                if !self.grid[x as usize][y as usize] {
                    full_line = false;
                    break;
                }
            }
            if full_line {
                for y2 in (1..=y).rev() {
                    for x in 0..GRID_WIDTH {
                        self.grid[x as usize][y2 as usize] = self.grid[x as usize][(y2 - 1) as usize];
                    }
                }
                lines_cleared += 1;
            }
        }
    
        self.score += match lines_cleared {
            1 => 40,
            2 => 100,
            3 => 300,
            4 => 1200,
            _ => 0,
        };
    }

    fn lock_piece(&mut self) {
        let shape = self.current_piece.get_rotated_shape(self.current_piece_rotation);
        for i in 0..4 {
            for j in 0..4 {
                if shape[i as usize][j as usize] {
                    let x = self.current_piece_position.0 + i;
                    let y = self.current_piece_position.1 + j;
                    self.grid[x as usize][y as usize] = true;
                }
            }
        }
    }

    fn is_collision(&self) -> bool {
        let shape = self.current_piece.get_rotated_shape(self.current_piece_rotation);
        for i in 0..4 {
            for j in 0..4 {
                let x = self.current_piece_position.0 + i;
                let y = self.current_piece_position.1 + j;
                if shape[i as usize][j as usize] {
                    if x < 0 || x >= GRID_WIDTH || y < 0 || y >= GRID_HEIGHT {
                        return true;
                    }
                    if self.grid[x as usize][y as usize] {
                        return true;
                    }
                }
            }
        }
        false
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        d.clear_background(COLOR_BG);

        // Draw grid
        for x in 0..GRID_WIDTH {
            for y in 0..GRID_HEIGHT {
                let position = Vector2 {
                    x: GRID_OFFSET.x + (x as i32 * CELL_SIZE) as f32,
                    y: GRID_OFFSET.y + (y as i32 * CELL_SIZE) as f32,
                };
                if self.grid[x as usize][y as usize] {
                    d.draw_rectangle_v(position, Vector2::new(CELL_SIZE as f32, CELL_SIZE as f32), Color::GREEN);
                } else {
                    d.draw_rectangle_lines_ex(
                        Rectangle::new(position.x, position.y, CELL_SIZE as f32, CELL_SIZE as f32),
                        1,
                        COLOR_GRID,
                    );
                }
            }
        }

        // Draw current piece
        let shape = self.current_piece.get_rotated_shape(self.current_piece_rotation);
        for i in 0..4 {
            for j in 0..4 {
                if shape[i as usize][j as usize] {
                    let x = self.current_piece_position.0 + i;
                    let y = self.current_piece_position.1 + j;
                    let position = Vector2 {
                        x: GRID_OFFSET.x + (x as i32 * CELL_SIZE) as f32,
                        y: GRID_OFFSET.y + (y as i32 * CELL_SIZE) as f32,
                    };
                    d.draw_rectangle_v(position, Vector2::new(CELL_SIZE as f32, CELL_SIZE as f32), COLOR_PIECE);
                }
            }
        }
    }
}


fn main() {
    let (mut rl, thread) = raylib::init()
        .size(800, 1000)
        .title("Tetris")
        .build();

    let mut game = TetrisGame::new();

    while !rl.window_should_close() {
        game.last_key_pressed = handle_input(&mut rl);

        let mut d = rl.begin_drawing(&thread);

        game.update();
        game.draw(&mut d);
    }
}

#[derive(Clone, Copy)]
enum Tetrimino {
    I,
    O,
    T,
    J,
    L,
    S,
    Z,
}

fn handle_input(rl: &mut RaylibHandle) -> Option<KeyboardKey> {
    rl.get_key_pressed()
}

