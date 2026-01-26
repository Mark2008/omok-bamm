use rand::Rng;
use crate::core::board::{Board, Move, Stone};

pub struct Zobrist {
    h: [[u64; 3]; 255],
}

impl Zobrist {
    pub fn init() -> Self {
        let mut rng = rand::thread_rng();
        let mut h = [[0; 3]; 255];

        for i in 0..255 {
            for j in 0..3 {
                h[i][j] = rng.r#gen();
            }
        }

        Self { h }
    }

    pub fn hash(&self, board: &Board) -> u64 {
        let mut h = 0;
        for x in 0..15 {
            for y in 0..15 {
                let mv = Move{x, y};
                let num = match board.get(mv) {
                    Stone::None => 0,
                    Stone::Black => 1,
                    Stone::White => 2
                };
                h ^= self.h[y * 15 + x][num];
            }
        }
        h
    }

    pub fn update(&self, hash: u64, mv: Move, stone: Stone) -> u64 {
        let num = match stone {
            Stone::None => 0,
            Stone::Black => 1,
            Stone::White => 2,
        };
        hash ^ self.h[mv.y * 15 + mv.x][num]
    }
}