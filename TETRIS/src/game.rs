use std::{time::{Instant, Duration}};

use raylib::{prelude::*, ffi::ColorAlpha};

use crate::{tetromino::Tetromino, scoreboard::{self, ScoreBoard}};

pub enum Mode {
    Classic,
    Modern
}

pub struct Game {
    pub board: Rectangle, 
    pub mode: Mode,
    pub spawn_point: Vector2,
    pub curr_piece: Tetromino, 
    pub next_piece: Tetromino, 
    pub swap_piece: Tetromino, 
    game_state: Vec<Vec<bool>>,
    colour: Color,
    last_fall_time: Instant,
    pub score: u32,
    pub level: u32,
    pub username: String,
    lines: u32,
    is_running: bool
}

impl Game {
    pub fn new(handle: &RaylibHandle, mode: Mode, level: u32, block_size: i32, username: &str) -> Game {
        let board_dim: Vector2 = match mode {
            Mode::Classic => { Vector2::new(10 as f32 , 20 as f32) },
            Mode::Modern => { Vector2::new(15 as f32, 20 as f32) }
        };

        let game_board = Rectangle::new(
                            (handle.get_screen_width() as f32 * 0.75) / 2.0 - board_dim.x * block_size as f32 / 2.0 as f32,
                            50.0,
                            board_dim.x * block_size as f32,
                            board_dim.y * block_size as f32
                        );

        let game_state = vec![vec![false; board_dim.x as usize]; board_dim.y as usize];
        let spawn_point = Vector2::new((board_dim.x as i32 / 2 - 2) as f32, 0.0);
        Game { 
            board: game_board, 
            mode: mode,
            spawn_point: spawn_point, 
            game_state: game_state,
            colour: Color::LIGHTGRAY,
            curr_piece: Tetromino::random(spawn_point),
            next_piece: Tetromino::random(spawn_point),
            swap_piece: Tetromino::random(spawn_point),
            last_fall_time: Instant::now(),
            score: 0,
            level: level,
            lines: 0,
            is_running: false,
            username: username.to_owned()
        }
    }   

    pub fn update(&mut self, input: Option<KeyboardKey>) {    

        if let Some(key) = input {
            match key {
                KeyboardKey::KEY_A => {
                    if !self.is_running {
                        return
                    }
                    let shape = self.curr_piece.get_shape_left();
                    if !self.is_collision(shape, self.curr_piece.pos) {
                        self.curr_piece.rotate_left();
                    }
                },
                KeyboardKey::KEY_D => {
                    if !self.is_running {
                        return
                    }
                    let shape = self.curr_piece.get_shape_right();
                    if !self.is_collision(shape, self.curr_piece.pos) {
                        self.curr_piece.rotate_right();
                    }
                },
                KeyboardKey::KEY_LEFT => {
                    if !self.is_running {
                        return
                    }
                    let t = self.curr_piece.try_move_left();
                    let shape = t.get_shape();
                    if !self.is_collision(shape, Vector2::new(t.pos.x, t.pos.y)) {
                        self.curr_piece = t;
                    }
                },
                KeyboardKey::KEY_RIGHT => {
                    if !self.is_running {
                        return
                    }
                    let t = self.curr_piece.try_move_right();
                    let shape = t.get_shape();
                    if !self.is_collision(shape, Vector2::new(t.pos.x, t.pos.y)) {
                        self.curr_piece = t;
                    }
                },
                KeyboardKey::KEY_SPACE => {
                    if !self.is_running {
                        return
                    }
                    let shape = self.curr_piece.get_shape();
                    while !self.is_collision(shape, self.curr_piece.pos) {
                        self.curr_piece.pos.y += 1.0;
                    }
                    self.curr_piece.pos.y -= 1.0;
                    self.lock_piece();
                    self.clear_lines();
                    
                    self.curr_piece = self.next_piece;
                    self.next_piece = Tetromino::random(self.spawn_point);

                    if self.is_collision(shape, self.curr_piece.pos) {
                        self.game_over();
                    }
                },
                KeyboardKey::KEY_DOWN => {
                    if !self.is_running {
                        return
                    }
                    self.curr_piece.pos.y += 1.0;
                    let shape = self.curr_piece.get_shape();
                    if self.is_collision(shape, self.curr_piece.pos) {
                        self.curr_piece.pos.y -= 1.0;
                        self.lock_piece();
                        self.clear_lines();
    
                        self.curr_piece = self.next_piece;
                        self.next_piece = Tetromino::random(self.spawn_point);

                        if self.is_collision(shape, self.curr_piece.pos) {
                            self.game_over();
                        }
                    }
                },
                KeyboardKey::KEY_T => {
                    if !self.is_running {
                        return
                    }
                    match self.mode {
                        Mode::Classic => return,
                        Mode::Modern => {
                            let tmp_piece = self.curr_piece;
                            self.curr_piece = self.swap_piece;
                            self.curr_piece.pos = tmp_piece.pos;  
                            self.swap_piece = tmp_piece;
                        }
                    }

                },
                KeyboardKey::KEY_P => {
                    self.is_running = !self.is_running;
                }
                _ => (),
            }
        }
        
        if !self.is_running {
            return
        }
         
        let fall_interval =  Duration::from_millis((1000 - self.level * 50).into());

        if Instant::now() - self.last_fall_time > fall_interval {
            self.curr_piece.pos.y += 1.0;
            let shape = self.curr_piece.get_shape();
            if self.is_collision(shape, self.curr_piece.pos) {
                self.curr_piece.pos.y -= 1.0;
                self.lock_piece();
                self.clear_lines();
                self.curr_piece = self.next_piece;
                self.next_piece = Tetromino::random(self.spawn_point);
                if self.is_collision(shape, self.curr_piece.pos) {
                    self.game_over();
                }
            }
            self.last_fall_time = Instant::now();
        }
    }

    fn game_over(&mut self) {
        self.is_running = false;
    }

    fn is_collision(&self, shape: [[bool; 4]; 4], pos: Vector2) -> bool {
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

        for _i in 0..removed {
            self.game_state.insert(0, empty_line.to_vec())
        }

        self.score += match removed {
            1 => 40 * (self.level + 1),
            2 => 100 * (self.level + 1),
            3 => 300 * (self.level + 1),
            4 => 1200 * (self.level + 1),
            _ => 0,
        };
        self.lines += removed as u32;
        if self.lines % 10 == 0 && removed > 0 && self.level < 15 {
            self.level += 1;
        }
    }

    fn lock_piece(&mut self) {
        let shape = self.curr_piece.get_shape();
        for i in 0..4 {
            for j in 0..4 {
                if shape[i as usize][j as usize] {
                    self.game_state[std::cmp::max(self.curr_piece.pos.y as i32 + i, 0) as usize][(self.curr_piece.pos.x as i32 + j) as usize] = true;
                }
            }
        }
    }

    pub fn draw(&self, handle: &mut RaylibDrawHandle) {

        if !self.is_running {
            handle.draw_text("P - play", 10, handle.get_screen_height() - 30, 20, Color::LIGHTGRAY)
        }
        else {
            handle.draw_text("P - pause", 10, handle.get_screen_height() - 30, 20, Color::LIGHTGRAY)
        }

        handle.draw_rectangle_lines_ex(self.board, 2, self.colour);

        let cell_size = 32;
        //vertical
        for i in 0..(self.board.width as i32 / cell_size) {
            handle.draw_line(
                (self.board.x as i32 + i * cell_size) as i32, 
                self.board.y as i32, 
                (self.board.x as i32 + i * cell_size) as i32,
                (self.board.y + self.board.height) as i32,
                self.colour
            );
        }
        for i in 0..(self.board.height as i32 / cell_size) {
            handle.draw_line(
                self.board.x as i32,
                (self.board.y as i32 + i * cell_size) as i32,
                (self.board.x + self.board.width) as i32, 
                (self.board.y as i32 + i * cell_size) as i32, 
                self.colour);
        }

        let mut curr_pos = Vector2::new(self.board.x, self.board.y); 
        for row in &self.game_state {
            for val in row {
                if *val {
                    handle.draw_rectangle(curr_pos.x as i32, curr_pos.y as i32, 32, 32, self.colour);
                }
                curr_pos.x += 32.0;
            }
            curr_pos.y += 32.0;
            curr_pos.x = self.board.x;
        }

        let ref_pos = Vector2::new(self.board.x, self.board.y);
        curr_pos = Vector2::new(ref_pos.x + self.curr_piece.pos.x * 32.0, ref_pos.y + self.curr_piece.pos.y * 32.0);
        for row in self.curr_piece.get_shape() {
            for val in row {
                if val {
                    handle.draw_rectangle(curr_pos.x as i32, curr_pos.y as i32, 32, 32, self.curr_piece.color);
                }
                curr_pos.x += 32.0;
            }
            curr_pos.y += 32.0;
            curr_pos.x = ref_pos.x + self.curr_piece.pos.x * 32.0;
        }
    }
}