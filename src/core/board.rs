
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Stone {
    None, Black, White,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Turn {
    Black, White
}

pub type Player = Turn;

pub struct Board {
    v: [[Stone; 15]; 15],
    pub turn: Turn,
    pub ply: u32,
}

#[derive(Debug, Copy, Clone)]
pub struct Move {
    pub x: usize, pub y: usize
}

impl Turn {
    pub fn next(&self) -> Self {
        match self {
            Turn::Black => Turn::White,
            Turn::White => Turn::Black,
        }
    }

    pub fn to_stone(&self) -> Stone {
        match self {
            Turn::Black => Stone::Black,
            Turn::White => Stone::White,
        }
    }
}

impl Board {
    pub fn blank() -> Self {
        Self {
            v: [[Stone::None; 15]; 15],
            turn: Turn::Black,
            ply: 0,
        }
    }

    pub fn get(&self, mv: Move) -> Stone {
        self.v[mv.y][mv.x]
    }

    pub fn put_force(&mut self, mv: Move, stone: Stone) {
        self.turn = self.turn.next();
        self.ply += 1;
        self.v[mv.y][mv.x] = stone;
    }
}

impl Move {
    pub fn new(x: usize, y: usize) -> Option<Self> {
        if x >= 15 || y >= 15 {
            return None;
        }
        Some(Self { x, y })
    }

    pub fn shift(&self, dx: i32, dy: i32) -> Option<Self> {
        let x = self.x as i32 - dx;
        let y = self.y as i32 - dy;
        if x < 0 || y < 0 || x >= 15 || y >= 15 {
            return None;
        }
        Some(
            Self {
                x: x as usize, 
                y: y as usize
            }
        )
    }
}