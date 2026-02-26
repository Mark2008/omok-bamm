use std::fmt::Debug;
use std::sync::Arc;
use crate::core::{
    board::{Board, Move, Stone, Player},
    rule::Rule,
};

pub trait Eval: Debug + Send + Sync {
    fn eval(&self, board: &Board, mv: Move) -> f32;
}

#[derive(Debug)]
pub struct BaboEval {
    pub rule: Arc<dyn Rule>,
}


// todo: remove rule field
/// slow but solid evaluation
#[derive(Debug)]
pub struct PatternEval<R: Rule> {
    pub rule: R,
}

impl Eval for BaboEval {
    fn eval(&self, board: &Board, mv: Move) -> f32 {
        let player = board.turn();
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


struct PatternCount {
    open_cnt_black: [u8; 5],
    half_cnt_black: [u8; 5],
    open_cnt_white: [u8; 5],
    half_cnt_white: [u8; 5],
    five_black: u8,
    five_white: u8,
}

fn pattern_count(board: &Board) -> PatternCount {
    // initialize result
    let mut result = PatternCount {
        open_cnt_black: [0; 5],
        half_cnt_black: [0; 5],
        open_cnt_white: [0; 5],
        half_cnt_white: [0; 5],
        five_black: 0,
        five_white: 0,
    };
    
    // horizontal count
    for x in 0..15 {
        scan_line(board, x, 0, 0, 1, &mut result);
    }

    // vertical count
    for y in 0..15 {
        scan_line(board, 0, y, 1, 0, &mut result);
    }

    // diagonal down (top left --> bottom right)
    for x in 0..15 {
        scan_line(board, x, 0, 1, 1, &mut result);
    }
    for y in 1..15 {
        scan_line(board, 0, y, 1, 1, &mut result);
    }

    // diagonal up (bottom left --> top right)
    for x in 0..15 {
        scan_line(board, x, 14, 1, -1, &mut result);
    }
    for y in 0..14 {
        scan_line(board, 0, y, 1, -1, &mut result);
    }

    result
}

fn scan_line(
    board: &Board,
    start_x: usize,
    start_y: usize,
    dx: isize,
    dy: isize,
    result: &mut PatternCount,
) {
    let mut x = start_x as isize;
    let mut y = start_y as isize;

    let mut open = false;
    let mut cnt: usize = 0;
    let mut last = Stone::None;
    
    while 0 <= x && x < 15 && 0 <= y && y < 15 {
        let stone = board.get(Move { x: x as usize, y: y as usize });

        if stone == last {
            cnt += 1;
        } else {
            if cnt >= 5 {
                if cnt == 5 {
                    match last {
                        Stone::None => (),
                        Stone::Black => result.five_black += 1,
                        Stone::White => result.five_white += 1,
                    }
                }
                cnt = 1;
                last = stone;
                continue;
            }

            match last {
                Stone::None => open = true,
                Stone::Black => {
                    if (stone == Stone::None) ^ open {
                        result.half_cnt_black[cnt] += 1;
                    } else if (stone == Stone::None) && open {
                        result.open_cnt_black[cnt] += 1;
                    }
                },
                Stone::White => {
                    if (stone == Stone::None) ^ open {
                        result.half_cnt_white[cnt] += 1;
                    } else if (stone == Stone::None) && open {
                        result.open_cnt_white[cnt] += 1;
                    }
                }
            }

            cnt = 1;
            last = stone;
        }

        x += dx;
        y += dy;
    }

    if cnt >= 5 {
        if cnt == 5 {
            match last {
                Stone::None => (),
                Stone::Black => result.five_black += 1,
                Stone::White => result.five_white += 1,
            }
        }
        return;
    }

    match last {
        Stone::None => (),
        Stone::Black => {
            if open {
                result.half_cnt_black[cnt] += 1;
            }
        },
        Stone::White => {
            if open {
                result.half_cnt_white[cnt] += 1;
            }
        }
    }
}

const FIVE: f32 = 1000.0;
const OPEN_FOUR: f32 = 600.0;
const HALF_FOUR: f32 = 200.0;
const OPEN_THREE: f32 = 100.0;
const HALF_THREE: f32 = 50.0;
const OPEN_TWO: f32 = 10.0;

fn multiply_weight_value(open: [u8; 5], half: [u8; 5], five: u8) -> f32 {
      open[2] as f32 * OPEN_TWO
    + open[3] as f32 * OPEN_THREE
    + open[4] as f32 * OPEN_FOUR
    + half[3] as f32 * HALF_THREE
    + half[4] as f32 * HALF_FOUR
    + five as f32 * FIVE
}

impl<R: Rule> Eval for PatternEval<R> {
    fn eval(&self, board: &Board, _mv: Move) -> f32 {
        let pattern_count = pattern_count(board);

        let value_black = multiply_weight_value(
            pattern_count.open_cnt_black,
            pattern_count.half_cnt_black,
            pattern_count.five_black,
        );
        let value_white = multiply_weight_value(
            pattern_count.open_cnt_white,
            pattern_count.half_cnt_white,
            pattern_count.five_white,
        );

        let player = board.turn();
        match player {
            Player::Black => value_black - value_white,
            Player::White => value_white - value_black,
        }
    }
}