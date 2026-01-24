use crate::core::{
    board::{Board, Player, Move},
    rule::{OmokRule, Rule},
};

pub trait Eval {
    fn eval(&self, board: &Board, mv: Move, player: Player) -> f32;
}

struct BaboEval;

impl Eval for BaboEval {
    fn eval(&self, board: &Board, mv: Move, player: Player) -> f32 {
        let rule = OmokRule{};

        let winning_player = rule.is_winning(board, mv, player);
        if winning_player {
            return 1000.0;
        }

        let winning_opponent = rule.is_winning(board, mv, player.next());
        if winning_opponent {
            return -1000.0;
        }

        0.0
    }
}