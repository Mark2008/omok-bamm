use crate::core::board::{Board, Move, Stone};

pub trait Prune: Send + Sync {
    fn possible(&self, board: &Board, mv: Move) -> Vec<Move>;
}


pub struct NoPrune;

pub struct NeighborPrune;

impl Prune for NoPrune {
    fn possible(&self, board: &Board, mv: Move) -> Vec<Move> {
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
}