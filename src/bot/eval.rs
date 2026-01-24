use crate::core::{
    board::{Board, Player, Move},
    rule::Rule,
};

pub trait Eval {
    fn eval(&self, board: &Board, mv: Move, player: Player) -> f32;
}

struct BaboEval {
    rule: Box<dyn Rule>,
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