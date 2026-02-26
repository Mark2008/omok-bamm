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
        if v.is_empty() {
            vec![Move { x: 7, y: 7 }]
        } else {
            v
        }
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

        // heuristic: consider distance from last put position and center
        v.sort_by_key(|item| {
            let dist_last = chebyshev_dist(mv, *item);
            let dist_center = chebyshev_dist(Move { x: 7, y: 7}, *item);
            dist_last * 10 + dist_center
        });

        // if there's nowhere to put (assuming blank board)
        // or full-filled board can be the case
        if v.is_empty() {
            vec![Move { x: 7, y: 7 }]
        } else {
            v
        }
    }
}


// utils
fn chebyshev_dist(mv1: Move, mv2: Move) -> usize {
    let dx = mv1.x.abs_diff(mv2.x);
    let dy = mv1.y.abs_diff(mv2.y);
    dx.max(dy)
}