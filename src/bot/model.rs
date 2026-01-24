use core::f32;

use rand::Rng;
use crate::core::board::{Board, Move, Stone};
use super::eval::Eval;

pub trait Model: Send + Sync {
    /// if None, the bot resigns (?)
    fn next_move(&self, board: &Board, mv: Move) -> Option<Move>;
}

pub struct RandomBaboModel;

impl Model for RandomBaboModel {
    fn next_move(&self, board: &Board, mv: Move) -> Option<Move> {        
        let dir = rand::thread_rng().gen_range(0..8) as usize;
        for i in 0..8 {
            let shifted = match (dir + i) % 8 {
                0 => mv.shift(1, 1),
                1 => mv.shift(1, 0),
                2 => mv.shift(1, -1),
                3 => mv.shift(0, -1),
                4 => mv.shift(-1, -1),
                5 => mv.shift(-1, 0),
                6 => mv.shift(-1, 1),
                7 => mv.shift(0, 1),
                _ => unreachable!()
            };
            if let Some(mv) = shifted {
                if board.get(mv) == Stone::None {
                    return Some(mv);
                }
            }
        }

        for _ in 0..100 {
            let rand_x = rand::thread_rng().gen_range(0..15) as usize;
            let rand_y = rand::thread_rng().gen_range(0..15) as usize;

            let mv = Move::new(rand_x, rand_y).unwrap();
            if board.get(mv) == Stone::None {
                return Some(Move::new(rand_x, rand_y).unwrap());

            }
        }

        None
    }
}

pub struct NegamaxModel {
    pub depth: u32,
    pub eval: Box<dyn Eval>,
}

impl NegamaxModel {
    fn possible_moves(&self, board: &Board, mv: Move) -> Vec<Move> {
        let _ = mv; // unused
        let mut v = Vec::new();
        for i in 0..15 {
            for j in 0..15 {
                let mv = Move::new(j, i).unwrap();
                if board.get(mv) == Stone::None {
                    v.push(mv);
                }
            }
        }
        v
    }

    fn negamax(&self, board: &Board, mv: Move, d: u32) -> f32 {
        if d == 0 {
            let eval = self.eval.eval(board, mv, board.turn());
            return eval;
        }
        let possible = self.possible_moves(board, mv);

        if possible.is_empty() {
            let eval = self.eval.eval(board, mv, board.turn());
            return eval;
        }

        let mut max = f32::NEG_INFINITY;

        for mv in possible {
            let stone = board.turn().next().to_stone();
            let temp_board = board.with_move(mv, stone).unwrap();   // trust self.possible_moves
            
            let eval = -self.negamax(&temp_board, mv, d - 1);
            
            if max < eval {
                max = eval;
            }
        }

        max
    }
}

impl Model for NegamaxModel {
    fn next_move(&self, board: &Board, mv: Move) -> Option<Move> {
        let mut best = f32::NEG_INFINITY;
        let mut best_mv = None;

        for mv in self.possible_moves(board, mv) {
            let stone = board.turn().next().to_stone();
            // println!("{:?}, {:?}", mv, stone);
            let next_board = board.with_move(mv, stone).unwrap();   // possible_moves guarantee not None

            let eval = -self.negamax(&next_board, mv, self.depth - 1);
            if eval > best {
                best = eval;
                best_mv = Some(mv);
            }
        }

        best_mv
    }
}