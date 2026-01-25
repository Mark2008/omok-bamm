use rand::Rng;
use crate::core::board::{Board, Move, Stone};
use crate::core::rule::{Rule};
use super::eval::Eval;
use super::prune::Prune;

pub trait Model: Send + Sync {
    /// if None, the bot resigns (?)
    fn next_move(&self, board: &Board, mv: Move) -> Option<Move>;
}

#[derive(Debug)]
pub struct RandomBaboModel;

#[derive(Debug)]
pub struct NegamaxModel {
    pub depth: u32,
    pub eval: Box<dyn Eval>,
    pub prune: Box<dyn Prune>,
    pub rule: Box<dyn Rule>,
}

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

impl NegamaxModel {
    // #[tracing::instrument(skip(self, board), ret)]
    fn negamax(&self, board: &Board, mv: Move, d: u32) -> f32 {
        if d == 0 {
            // terminal node
            let eval = self.eval.eval(board, mv, board.turn().next());
            return eval;
        }
        let possible = self.prune.possible(board, mv);

        if possible.is_empty() {
            // terminal node
            let eval = self.eval.eval(board, mv, board.turn().next());
            return eval;
        }
        let winning_self = self.rule.is_winning(board, mv, board.turn());
        if winning_self {
            return -10000.0;
        }
        let winning_opponent = self.rule.is_winning(board, mv, board.turn().next());
        if winning_opponent {
            return 10000.0;
        }

        let mut max = core::f32::NEG_INFINITY;

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

        for mv in self.prune.possible(board, mv) {
            let stone = board.turn().next().to_stone();
            // tracing::debug!("{:?}, {:?}", mv, stone);
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