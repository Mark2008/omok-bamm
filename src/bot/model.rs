use rand::Rng;
use crate::core::board::{Board, Move, Stone};

pub trait Model {
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

struct NegamaxModel {
    depth: u32,
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

    fn negamax(&self, board: &Board, mv: Move, d: u32) -> (f32, Move) {
        if d == 0 {
            // return heuristic value
        }
        let possible = self.possible_moves(board, mv);

        if possible.is_empty() {
            // terminal node.
            // return somethingdgsadaggdsgsda i dont know
        }

        let mut max_mv = possible.first().unwrap().clone();
        let mut max = f32::NEG_INFINITY;

        for mv in possible {
            let stone = board.turn.next().to_stone();
            let temp_board = board.with_move(mv, stone).unwrap();   // trust self.possible_moves
            
            let (eval, _) = self.negamax(&temp_board, mv, d - 1);
            let eval = -eval;
            
            if max < eval {
                max = eval;
                max_mv = mv;
            }
        }

        (max, max_mv)
    }
}

impl Model for NegamaxModel {
    fn next_move(&self, board: &Board, mv: Move) -> Option<Move> {
        let (eval, result_mv) = self.negamax(board, mv, self.depth);
        Some(result_mv)
    }
}