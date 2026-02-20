use rand::Rng;
use crate::core::board::{Board, Move, Stone};
use crate::core::rule::{PutOutcome, Rule};
use super::eval::Eval;
use super::prune::Prune;

pub trait Model: Send + Sync {
    /// if None, the bot resigns (?)
    fn next_move(&self, board: &Board, mv: Move) -> Option<Move>;
}

#[derive(Debug)]
pub struct RandomBaboModel;

#[derive(Debug)]
pub struct NegamaxModel<E: Eval, P: Prune, R: Rule> {
    pub depth: u32,
    pub eval: E,
    pub prune: P,
    pub rule: R,
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

impl<E: Eval, P: Prune, R: Rule> NegamaxModel<E, P, R> {
    pub fn new(depth: u32, eval: E, prune: P, rule: R) -> Self {
        Self {
            depth,
            eval,
            prune,
            rule,
        }
    }
    // #[tracing::instrument(skip(self, board), ret)]
    fn negamax(&self, board: &Board, mv: Move, d: u32) -> f32 {
        if d == 0 {
            // terminal node
            let eval = -self.eval.eval(board, mv, board.turn().next());
            return eval;
        }
        let possible = self.prune.possible(board, mv);

        // if possible.is_empty() {
        //     // terminal node
        //     let eval = self.eval.eval(board, mv, board.turn());
        //     return eval;
        // }

        let mut max = core::f32::NEG_INFINITY;

        for mv in possible {
            let eval = self.eval_after_move(board, mv, d);
            max = max.max(eval);
        }

        max
    }

    // helper function (common logic)
    fn eval_after_move(&self, board: &Board, mv: Move, d: u32) -> f32 {
        let mut next_board = board.clone();
        let result = self.rule.put(&mut next_board, mv, board.turn());

        match result {
            Ok(PutOutcome::Continue) => {
                -self.negamax(&next_board, mv, d - 1)
            },
            Ok(PutOutcome::Win) => { 100000.0 },
            Ok(PutOutcome::Draw) => { 0.0 },
            Err(error_type) => {
                tracing::debug!("{:?}", error_type);
                // invalid moves (e.g., forbidden like 3-3) are treated as worst possible
                core::f32::NEG_INFINITY
            }
        }
    }
}

impl<E: Eval, P: Prune, R: Rule> Model for NegamaxModel<E, P, R> {
    fn next_move(&self, board: &Board, mv: Move) -> Option<Move> {
        let mut best = f32::NEG_INFINITY;
        let mut best_mv = None;

        let possible = self.prune.possible(board, mv);
        for mv in possible {
            let eval = self.eval_after_move(board, mv, self.depth);
            // let stone = board.turn().next().to_stone();
            // // tracing::debug!("{:?}, {:?}", mv, stone);
            // let next_board = board.with_move(mv, stone).unwrap();   // possible_moves guarantee not None

            // let eval = -self.negamax(&next_board, mv, self.depth - 1);
            if eval > best {
                best = eval;
                best_mv = Some(mv);
            }
        }

        best_mv
    }
}