use std::fmt::Debug;
use crate::core::board::{Board, Move, Stone};

pub trait Prune: Debug + Send + Sync {
    fn possible(&self, board: &Board, mv: Move) -> Vec<Move>;
}

#[derive(Debug)]
pub struct NoPrune;

#[derive(Debug)]
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

const SHIFT_ARRAY: [(i32, i32); 8] = [(1,1),(1,0),(1,-1),(0,-1),(-1,-1),(-1,0),(-1,1),(0,1)];
impl Prune for NeighborPrune {
    fn possible(&self, board: &Board, mv: Move) -> Vec<Move> {
        let _ = mv; // unused
        let mut candid = [[false; 15]; 15];
        for i in 0..15 {
            for j in 0..15 {
                let mv = Move::new(j, i).unwrap();
                if board.get(mv) == Stone::None {
                    continue;
                }
                for (dx, dy) in SHIFT_ARRAY {
                    let x = j as i32 - dx;
                    let y = i as i32 - dy;

                    if x < 0 || y < 0 || x >= 15 || y >= 15 {
                        continue;
                    }
                    candid[y as usize][x as usize] = true;
                }
            }
        }

        let mut v = Vec::new();
        for i in 0..15 {
            for j in 0..15 {
                if !candid[i][j] {
                    continue;
                }
                let mv = Move::new(j, i).unwrap();
                if board.get(mv) == Stone::None {
                    v.push(mv);
                }
            }
        }
        v
    }
}