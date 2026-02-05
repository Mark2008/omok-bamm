use crate::core::{
    board::{Board, Move, Player},
    rule::Rule,
};
use std::fmt::Debug;
use std::sync::Arc;

pub trait Eval: Debug + Send + Sync {
    fn eval(&self, board: &Board, mv: Move, player: Player) -> f32;
}

#[derive(Debug)]
pub struct BaboEval {
    pub rule: Arc<dyn Rule>,
}

impl Eval for BaboEval {
    fn eval(&self, board: &Board, mv: Move, player: Player) -> f32 {
        let winning_player = self.rule.is_winning(board, mv, player);
        if winning_player {
            return 1000.0;
        }

        let winning_opponent = self.rule.is_winning(board, mv, player.next());
        if winning_opponent {
            return -1000.0;
        }

        0.0
    }
}
