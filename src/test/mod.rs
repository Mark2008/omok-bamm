#[allow(unused_imports)]
mod negamax {
    use std::sync::Arc;

    use crate::bot::model::Model;
    use crate::core::board::{Board, Move, Stone};
    use crate::bot::{model, eval, prune};
    use crate::core::rule;
    // use crate::bot::prune::*;

    #[test]
    fn panic_manual() {
        let mut board = Board::blank();

        board.put(Move { x: 7, y: 7 }, Stone::Black);
        board.put(Move { x: 6, y: 6 }, Stone::White);

        board.put(Move { x: 7, y: 6 }, Stone::Black);
        board.put(Move { x: 5, y: 5 }, Stone::White);

        board.put(Move { x: 6, y: 5 }, Stone::Black);

    }

    #[test]
    fn panic_bot() {
        super::super::init_trace();
        let mut board = Board::blank();

        let model = model::NegamaxModel {
            depth: 4,
            eval: eval::BaboEval { rule: Arc::new(rule::OmokRule) },
            prune: prune::NeighborPrune,
            rule: rule::OmokRule,
        };

        board.put(Move { x: 7, y: 7 }, Stone::Black);
        board.put(Move { x: 6, y: 6 }, Stone::White);

        board.put(Move { x: 7, y: 6 }, Stone::Black);
        board.put(Move { x: 5, y: 5 }, Stone::White);

        board.put(Move { x: 6, y: 5 }, Stone::Black);

        // let prune = Box::new(NeighborPrune);
        // let possible = prune.possible(&board, Move { x: 6, y: 5 });

        // println!("{:?}", possible);

        let next = model.next_move(&mut board, Move { x: 6, y: 5 });
        let _ = next;
    }
}