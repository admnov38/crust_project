use std::time::{Instant, Duration};

use raylib::prelude::*;

use crate::tetromino::Tetromino;

pub enum Mode {
    Classic,
    Modern
}

#[derive(Clone)]
enum BoardRec {
    Empty { pos: Vector2 },
    Taken{ pos: Vector2, colour: Color }
}

pub struct Game {
    pub board: Rectangle, 
    pub spawn_point: Vector2,
    curr_piece: Tetromino, 
    game_state: Vec<Vec<bool>>,
    colour: Color,
    last_fall_time: Instant,
    fall_interval: Duration,
    score: u32,
}

impl Game {
    pub fn new(handle: &RaylibHandle, mode: Mode, block_size: i32) -> Game {
        let board_dim: Vector2 = match mode {
            Mode::Classic => { Vector2::new(10 as f32 , 20 as f32) },
            Mode::Modern => { Vector2::new(20 as f32, 20 as f32) }
        };

        let game_board = Rectangle::new(
                            (handle.get_screen_width() as f32 * 0.75) / 2.0 - board_dim.x * block_size as f32 / 2.0 as f32,
                            board_dim.y,
                            board_dim.x * block_size as f32,
                            board_dim.y * block_size as f32
                        );


        let game_state = vec![vec![false; board_dim.x as usize]; board_dim.y as usize];
        Game { 
            board: game_board, 
            spawn_point: Vector2::new(0.0, 0.0), 
            game_state: game_state,
            colour: Color::RED,
            curr_piece: Tetromino::random( Vector2::new(0.0, 0.0)),
            last_fall_time: Instant::now(),
            fall_interval: Duration::from_millis(500),
            score: 0,
        }
    }   

    pub fn update(&mut self, input: Option<KeyboardKey>) {    
        if let Some(key) = input {
            match key {
                KeyboardKey::KEY_A => {
                    let shape = self.curr_piece.get_shape_left();
                    if !self.is_collision(shape, self.curr_piece.pos) {
                        self.curr_piece.rotate_left();
                    }
                },
                KeyboardKey::KEY_D => {
                    let shape = self.curr_piece.get_shape_right();
                    if !self.is_collision(shape, self.curr_piece.pos) {
                        self.curr_piece.rotate_right();
                    }
                },
                KeyboardKey::KEY_LEFT => {
                    let t = self.curr_piece.try_move_left();
                    let shape = t.get_shape();
                    if !self.is_collision(shape, Vector2::new(t.pos.x, t.pos.y)) {
                        self.curr_piece = t;
                    }
                },
                KeyboardKey::KEY_RIGHT => {
                    let t = self.curr_piece.try_move_right();
                    let shape = t.get_shape();
                    if !self.is_collision(shape, Vector2::new(t.pos.x, t.pos.y)) {
                        self.curr_piece = t;
                    }
                },
                _ => (),
            }
        }
        
        if Instant::now() - self.last_fall_time > self.fall_interval {
            self.curr_piece.pos.y += 1.0;
            let shape = self.curr_piece.get_shape();
            if self.is_collision(shape, self.curr_piece.pos) {
                self.curr_piece.pos.y -= 1.0;
                self.lock_piece();
                self.clear_lines();
                self.curr_piece = Tetromino::random(self.spawn_point);
                if self.is_collision(shape, self.curr_piece.pos) {
                    todo!();
                }
            }
            self.last_fall_time = Instant::now();
        }
    }

    fn is_collision(&self, shape: [[bool;4];4], pos: Vector2) -> bool {
        for i in 0..4 {
            for j in 0..4 {
                let x = pos.x as i32 + j;
                let y = pos.y as i32 + i;
                if shape[i as usize][j as usize] {
                    if x < 0 || x >= self.game_state[0].len() as i32 || y < 0 || y >= self.game_state.len() as i32 {
                        return true;
                    }
                    if self.game_state[y as usize][x as usize] {
                        return true;
                    }
                }
            }
        }
        false
    }

    fn clear_lines(&mut self) {
        let prev_height = self.game_state.len();
        self.game_state.retain(|row| !row.iter().all(|&b| b));
        let removed = prev_height - self.game_state.len();

        let empty_line = [false; 10];

        for i in 0..removed {
            self.game_state.insert(0, empty_line.to_vec())
        }

        self.score += match removed {
            1 => 40,
            2 => 100,
            3 => 300,
            4 => 1200,
            _ => 0,
        };
    }

    fn lock_piece(&mut self) {
        let shape = self.curr_piece.get_shape();
        for i in 0..4 {
            for j in 0..4 {
                if shape[i as usize][j as usize] {
                    self.game_state[(self.curr_piece.pos.y as i32 + i) as usize][(self.curr_piece.pos.x as i32 + j) as usize] = true;
                }
            }
        }
    }

    pub fn draw(&self, handle: &mut RaylibDrawHandle) {
        let score_text = format!("Score: {}", self.score);
        handle.draw_text(&score_text, 100, 100, 28, Color::RED);
        handle.draw_rectangle_lines_ex(self.board, 2, self.colour);
        let mut curr_pos = Vector2::new(self.board.x, self.board.y); 
        for row in &self.game_state {
            for val in row {
                if *val {
                    handle.draw_rectangle(curr_pos.x as i32, curr_pos.y as i32, 32, 32, Color::GRAY);
                }
                curr_pos.x += 32.0;
            }
            curr_pos.y += 32.0;
            curr_pos.x = self.board.x;
        }
        let kkt = &self.curr_piece;
        let ref_pos = Vector2::new(self.board.x, self.board.y);
        curr_pos = Vector2::new(ref_pos.x + kkt.pos.x * 32.0, ref_pos.y + kkt.pos.y * 32.0);
        for row in kkt.get_shape() {
            for val in row {
                if val {
                    handle.draw_rectangle(curr_pos.x as i32, curr_pos.y as i32, 32, 32, kkt.color);
                }
                curr_pos.x += 32.0;
            }
            curr_pos.y += 32.0;
            curr_pos.x = ref_pos.x + kkt.pos.x * 32.0;
        }
    }
}