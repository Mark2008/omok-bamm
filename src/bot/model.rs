use rand::Rng;
use crate::core::board::{Board, Move, Stone};

pub trait Model {
    /// if None, the bot resigns (?)
    fn next_move(&self, board: &Board, mv: Move) -> Option<Move>;
}

pub struct RandomBaboModel;

impl Model for RandomBaboModel {
    fn next_move(&self, board: &Board, mv: Move) -> Option<Move> {
        let _ = mv;
        loop {
            let rand_x = rand::thread_rng().gen_range(0..15) as usize;
            let rand_y = rand::thread_rng().gen_range(0..15) as usize;

            let mv = Move::new(rand_x, rand_y).unwrap();
            if board.get(mv) == Stone::None {
                return Some(Move::new(rand_x, rand_y).unwrap());

            }
        }
    }
}

// struct MinimaxModel {
//     depth: u32,
// }

// impl Model for MinimaxModel {
//     fn next_move(&self, board: &Board, mv: Move) -> Move {
        
//     }
// }