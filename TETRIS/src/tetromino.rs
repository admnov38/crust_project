use raylib::prelude::*;
use rand::Rng;

#[derive(Clone, Copy)]
pub enum TetrominoShape {
    I,
    O,
    T,
    L,
    J,
    S,
    Z
}

#[derive(Clone, Copy)]
pub struct Tetromino {
    pub tetormino_type: TetrominoShape,
    pub shapes: [[[bool; 4]; 4]; 4],
    pub pos: Vector2,
    pub orientation: usize, 
    pub color: Color
}


impl Tetromino {
    pub fn new(tetromino_size: f32, spawn_point: Vector2, shape: TetrominoShape) -> Tetromino {
        Self::generate_tetromino(shape, tetromino_size, spawn_point)
    }

    pub fn random(spawn_point: Vector2) -> Tetromino {
        let mut rng = rand::thread_rng();
        let random_enum_value: TetrominoShape = match rng.gen_range(0..7) {
            0 => TetrominoShape::I,
            1 => TetrominoShape::O,
            2 => TetrominoShape::T,
            3 => TetrominoShape::L,
            4 => TetrominoShape::J,
            5 => TetrominoShape::S,
            6 => TetrominoShape::Z,
            _ => panic!("Unexpected random value generated!"),
        };
        Self::generate_tetromino(random_enum_value, 30.0, spawn_point)
    } 

    fn generate_tetromino(tetromino_type: TetrominoShape, size: f32, pos: Vector2) -> Tetromino {

        match tetromino_type {
            TetrominoShape::I => {               
                let top_b = [
                    [false, true, false, false],
                    [false, true, false, false],
                    [false, true, false, false],
                    [false, true, false, false],
                ];
                let right_b = [
                    [false, false, false, false],
                    [true,  true,  true,  true],
                    [false, false, false, false],
                    [false, false, false, false],
                ];
                let left_b = [
                    [false, false, true, false],
                    [false, false, true, false],
                    [false, false, true, false],
                    [false, false, true, false],
                ];
                
                return Tetromino {
                    tetormino_type: tetromino_type,
                    shapes: [top_b, right_b, top_b, right_b],
                    orientation: 0,
                    color: Color::FIREBRICK,
                    pos: pos,
                }
            },
            TetrominoShape::O => {
                let top_b = [
                    [false, true,  true,  false],
                    [false, true,  true,  false],
                    [false, false, false, false],
                    [false, false, false, false],
                ];
            
                return Tetromino {
                    tetormino_type: tetromino_type,
                    shapes: [top_b, top_b, top_b, top_b],
                    orientation: 0,
                    color: Color::GREEN,
                    pos: pos,
                }
            },
            TetrominoShape::T => {
                let top_b = [
                    [false, true, false, false],
                    [true, true, true, false],
                    [false, false, false, false],
                    [false, false, false, false],
                ];
                let right_b = [
                    [false, true, false, false],
                    [false, true, true, false],
                    [false, true, false, false],
                    [false, false, false, false],
                ];
                let bottom_b = [
                    [false, false, false, false],
                    [true,  true,  true,  false],
                    [false, true,  false, false],
                    [false, false, false, false],
                ];
                let left_b = [
                    [false, true,  false, false],
                    [true,  true,  false, false],
                    [false, true,  false, false],
                    [false, false, false, false],
                ];


                return Tetromino {
                    tetormino_type: tetromino_type,
                    shapes: [top_b, right_b, bottom_b, left_b],
                    orientation: 0,
                    color: Color::SKYBLUE,
                    pos: pos,
                } 
            },
            TetrominoShape::L => { 
                let top_b = [
                    [false, true,  false, false],
                    [false, true,  false, false],
                    [false, true,  true,  false],
                    [false, false, false, false],
                ];
                let right_b = [
                    [false, false, false, false],
                    [true,  true,  true,  false],
                    [true,  false, false, false],
                    [false, false, false, false],
                ];
                let bottom_b = [
                    [true,  true,  false, false],
                    [false, true,  false,  false],
                    [false, true,  false, false],
                    [false, false, false, false],
                ];
                let left_b = [
                    [false, false, true,  false],
                    [true,  true,  true,  false],
                    [false, false, false, false],
                    [false, false, false, false],
                ];
                
                return Tetromino {
                    tetormino_type: tetromino_type,
                    shapes: [top_b, right_b, bottom_b, left_b],
                    orientation: 0,
                    pos: pos,
                    color: Color::GOLD
                } 
            },
            TetrominoShape::J => { 
                let top_b = [
                    [false, true,  false, false],
                    [false, true,  false, false],
                    [true,  true,  false, false],
                    [false, false, false, false],
                ];
                let right_b = [
                    [true,  false, false, false],
                    [true,  true,  true,  false],
                    [false, false, false, false],
                    [false, false, false, false],
                ];
                let bottom_b = [
                    [false, true,  true,  false],
                    [false, true,  false, false],
                    [false, true,  false, false],
                    [false, false, false, false],
                ];
                let left_b = [
                    [false, false, false, false],
                    [true,  true,  true,  false],
                    [false, false, true,  false],
                    [false, false, false, false],
                ];
            
                return Tetromino {
                    tetormino_type: tetromino_type,
                    shapes: [top_b, right_b, bottom_b, left_b],
                    orientation: 0,
                    pos: pos,
                    color: Color::ORANGE
                } 
            },
            TetrominoShape::S => {
                let top_b = [
                    [false, true,  true,  false],
                    [true,  true,  false, false],
                    [false, false, false, false],
                    [false, false, false, false],
                ];
                let right_b = [
                    [true,  false, false, false],
                    [true,  true,  false, false],
                    [false, true,  false, false],
                    [false, false, false, false],
                ];

                return Tetromino {
                    tetormino_type: tetromino_type,
                    shapes: [top_b, right_b, top_b, right_b],
                    orientation: 0,
                    pos: pos,
                    color: Color::PINK
                }
            },
            TetrominoShape::Z => {
                let top_b = [
                    [true,  true,  false, false],
                    [false, true,  true,  false],
                    [false, false, false, false],
                    [false, false, false, false],
                ];
                let right_b = [
                    [false, true,  false, false],
                    [true,  true,  false, false],
                    [true,  false, false, false],
                    [false, false, false, false],
                ];

                return Tetromino {
                    tetormino_type: tetromino_type,
                    shapes: [top_b, right_b, top_b, right_b],
                    orientation: 0,
                    pos,
                    color: Color::PURPLE
                }     
            }       
        };
    }

    pub fn get_shape(&self) -> [[bool; 4]; 4] {
        self.shapes[self.orientation]
    }

    pub fn try_move_right(&self) -> Tetromino {
        let new_pos = Vector2::new(self.pos.x + 1.0, self.pos.y);
        let orientation = self.orientation;
        let mut new_piece = Self::generate_tetromino(self.tetormino_type, 32.0, new_pos);
        new_piece.orientation = orientation;
        new_piece
    }

    pub fn try_move_left(&self) -> Tetromino {
        let new_pos = Vector2::new(self.pos.x - 1.0, self.pos.y);
        let orientation = self.orientation;
        let mut new_piece = Self::generate_tetromino(self.tetormino_type, 32.0, new_pos);
        new_piece.orientation = orientation;
        new_piece    
    }

    pub fn rotate_right(&mut self) {
        match self.orientation {
            0 => { self.orientation = 1 },
            1 => { self.orientation = 2 },
            2 => { self.orientation = 3 },
            3 => { self.orientation = 0 },
            _ => panic!()
        };
    }

    pub fn rotate_left(&mut self) {
        match self.orientation {
            0 => { self.orientation = 3 },
            3 => { self.orientation = 2 },
            2 => { self.orientation = 1 },
            1 => { self.orientation = 0 },
            _ => panic!()
        };
    }

    pub fn get_shape_right(&self) -> [[bool; 4]; 4] {
        match self.orientation {
            0 => { self.shapes[1] },
            1 => { self.shapes[2] },
            2 => { self.shapes[3] },
            3 => { self.shapes[0] },
            _ => panic!()
        }
    }

    pub fn get_shape_left(&self) -> [[bool; 4]; 4] {
        match self.orientation {
            0 => { self.shapes[3] },
            1 => { self.shapes[2] },
            2 => { self.shapes[1] },
            3 => { self.shapes[0] },
            _ => panic!()
        }
    }

}
