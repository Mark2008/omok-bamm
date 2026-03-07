use std::sync::Mutex;
use rand::Rng;
use crate::core::board::{Board, Move, Stone};
use crate::core::rule::{PutOutcome, Rule};
use super::eval::Eval;
use super::prune::Prune;
use super::hash::Zobrist;
use super::tt::{TT, TTEntry};

pub trait Model: Send + Sync {
    /// if None, the bot resigns (?)
    fn next_move(&mut self, board: &Board, mv: Move) -> Option<Move>;
}

use std::time::Instant;
// static variables for checking performance
use std::sync::atomic::{AtomicU64, Ordering};
pub static NODE_COUNT: AtomicU64 = AtomicU64::new(0);
pub static ABP_CUTOFF: AtomicU64 = AtomicU64::new(0);
pub static TT_HIT: AtomicU64 = AtomicU64::new(0);

#[derive(Debug)]
pub struct NegamaxModel<E: Eval, P: Prune, R: Rule> {
    pub depth: u32,
    pub eval: E,
    pub prune: P,
    pub rule: R,
    pub zobrist: Zobrist,
    // pub tt: Mutex<TT>,
    pub tt: TT,
}

impl<E: Eval, P: Prune, R: Rule> NegamaxModel<E, P, R> {
    pub fn new(depth: u32, eval: E, prune: P, rule: R) -> Self {
        Self {
            depth,
            eval,
            prune,
            rule,
            zobrist: Zobrist::init(),
            // tt: Mutex::new(TT::new(65536)),
            tt: TT::new(65536),
        }
    }

    fn negamax(
        &mut self, 
        board: &mut Board, 
        d: u32,
        alpha: f32,
        beta: f32,
        mv: Move, 
        hash: u64,
    ) -> f32 {
        if d == 0 {
            return self.eval.eval(&board, mv);
        }

        let possible = self.prune.possible(&board, mv);
        if possible.is_empty() {
            // terminal node
            return self.eval.eval(&board, mv);
        }

        let mut max = core::f32::NEG_INFINITY;
        let mut alpha = alpha;
        for mv in possible {
            let eval = self.eval_after_move(board, d, alpha, beta, mv, hash);
            max = max.max(eval);
            alpha = alpha.max(eval);
            if alpha >= beta {
                ABP_CUTOFF.fetch_add(1, Ordering::Relaxed);
                break;
            }
        }

        max
    }

    // helper function (common logic)
    fn eval_after_move(
        &mut self, 
        board: &mut Board, 
        d: u32,
        alpha: f32,
        beta: f32,
        mv: Move, 
        hash: u64,
    ) -> f32 {
        NODE_COUNT.fetch_add(1, Ordering::Relaxed);

        let turn = board.turn();

        // update hash value
        let hash = self.zobrist.update(hash, mv, turn.to_stone());
        let depth = board.ply() + d;
        if let Some(entry) = self.tt.get(hash) {
            if entry.depth >= depth {
                TT_HIT.fetch_add(1, Ordering::Relaxed);
                return entry.value;
            }

            // todo: 
            // add best_move to ttentry and search that move first
        }

        let result = self.rule.put(board, mv, turn);
        let eval = match result {
            Ok(outcome) => {
                let value = match outcome {
                    PutOutcome::Continue => -self.negamax(
                        board, d - 1, -beta, -alpha, mv, hash
                    ),
                    PutOutcome::Win => 100000.0 - d as f32,
                    PutOutcome::Draw => 0.0,
                };

                // revert to previous state
                board.undo_unchecked(mv);

                value
            },
            Err(error_type) => {
                // tracing::debug!("{:?}", error_type);
                // invalid moves (e.g., forbidden like 3-3) are treated as worst possible
                core::f32::NEG_INFINITY
            }
        };

        let entry = TTEntry {
            hash,
            value: eval,
            depth,
        };
        self.tt.put(entry);

        eval
    }
}

impl<E: Eval, P: Prune, R: Rule> Model for NegamaxModel<E, P, R> {
    fn next_move(&mut self, board: &Board, mv: Move) -> Option<Move> {
        // reset performance counter
        NODE_COUNT.store(0, Ordering::Relaxed);
        ABP_CUTOFF.store(0, Ordering::Relaxed);
        TT_HIT.store(0, Ordering::Relaxed);

        // start timer
        let start = Instant::now();

        let mut best = f32::NEG_INFINITY;
        let mut best_mv = None;

        // start point of simulation
        let mut clone_board = board.clone();

        // calculate hash
        let hash = self.zobrist.hash(board);

        let possible = self.prune.possible(board, mv);
        for mv in possible {
            let eval = self.eval_after_move(
                &mut clone_board, self.depth, 
                core::f32::NEG_INFINITY, core::f32::INFINITY, mv, hash,
            );
            
            if eval > best {
                best = eval;
                best_mv = Some(mv);
            }
        }

        // record result
        tracing::debug!(
            "\nNODE_COUNT: {}\nABP_CUTOFF: {}\nTT_HIT: {}",
            NODE_COUNT.load(Ordering::Relaxed),
            ABP_CUTOFF.load(Ordering::Relaxed),
            TT_HIT.load(Ordering::Relaxed),
        );
        tracing::debug!("elapsed: {:?}", start.elapsed());

        best_mv
    }
}