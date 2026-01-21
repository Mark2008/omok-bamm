use super::board::{Board, Stone, Move, Player};

pub trait Rule {
    fn is_valid(board: &Board, mv: Move, player: Player) -> bool;

    fn is_winning(board: &Board, mv: Move, player: Player) -> bool;

    fn check(board: &Board, mv: Move, player: Player) -> CheckResult {
        let valid: bool = Self::is_valid(board, mv, player);
        if !valid {
            return CheckResult::Invalid;
        }

        let winning = Self::is_winning(board, mv, player);
        if winning {
            return CheckResult::Win(player)
        }

        // todo: check draw

        CheckResult::LooksGood
    }

    fn put(
        board: &mut Board, mv: Move, player: Player
    ) -> Result<PutOutcome, PutError> {
        if board.get(mv) != Stone::None {
            return Err(PutError::Occupied);
        }

        let check = Self::check(board, mv, player);
        match check {
            CheckResult::Invalid => Err(PutError::Occupied),
            result => {
                board.put_force(mv, player.to_stone());
                Ok(match result {
                    CheckResult::LooksGood => PutOutcome::Continue,
                    CheckResult::Win(p) => PutOutcome::Win(p),
                    CheckResult::Draw => PutOutcome::Draw,
                    _ => unreachable!()
                })
            }
        }
    }
}

pub enum PutOutcome {
    Continue,
    Win(Player),
    Draw,
}

pub enum PutError {
    Occupied
}

pub enum CheckResult {
    LooksGood,
    Invalid,
    Win(Player),
    Draw
}

const DIRECTION: [Direction; 4] = [
    Direction::Horizontal, Direction::Vertical, 
    Direction::DiagDown, Direction::DiagUp,
];

enum Direction {
    Horizontal, Vertical, DiagDown, DiagUp,
}

impl Direction {
    pub fn delta(&self) -> (i32, i32) {
        match self {
            Direction::Horizontal => (1, 0),
            Direction::Vertical => (0, 1),
            Direction::DiagDown => (1, 1),
            Direction::DiagUp => (1, -1),
        }
    }
}

#[derive(PartialEq)]
enum OpenType {
    Open, HalfOpen, Closed
}

/// (Omok Rule)
/// disallawed 3-3, allowed 4-4
/// jangmok is not winning
struct OmokRule;

impl OmokRule {
    fn count_one_side(  // helper function
        board: &Board, mv: Move, stone: Stone,
        dx: i32, dy: i32
    ) -> (u32, bool) {
        let mut point = mv.clone();
        let mut cnt = 0;
        let mut open = false;
        loop {
            let shifted = point.shift(dx, dy);
            match shifted {
                Some(mv) => point = mv,
                None => break   // breaks when out of board
            }
            let item = board.get(point);
            match item {
                s if s == stone => cnt += 1,
                Stone::None => {
                    open = true;
                    break;
                }
                _ => {
                    break;
                }
            }
        }
        (cnt, open)
    }

    fn line_count(
        board: &Board, mv: Move, player: Player, 
        dx: i32, dy: i32
    ) -> (u32, OpenType) {
        let stone = player.to_stone();

        let (cnt1, open1) = Self::count_one_side(board, mv, stone, dx, dy);
        let (cnt2, open2) = Self::count_one_side(board, mv, stone, -dx, -dy);
        
        let open_type =
            if open1 && open2 { OpenType::Open }
            else if open1 ^ open2 { OpenType::HalfOpen }
            else { OpenType::Closed };
        
        (cnt1 + cnt2 + 1, open_type)
    }
}

impl Rule for OmokRule {
    fn is_valid(board: &Board, mv: Move, player: Player) -> bool {
        // 3-3 deteciton
        let mut already_sam = false;
        for (dx, dy) in DIRECTION.map(|x| x.delta()) {
            let (cnt, open) = Self::line_count(board, mv, player, dx, dy);
            if cnt == 3 && open == OpenType::Open {
                if !already_sam {
                    already_sam = true;
                }
                else {
                    return false;
                }
            }
        }
        true
    }

    fn is_winning(board: &Board, mv: Move, player: Player) -> bool {
        for (dx, dy) in DIRECTION.map(|x| x.delta()) {
            let (cnt, _) = Self::line_count(board, mv, player, dx, dy);
            if cnt == 5 {
                return true;
            }
        }
        false
    }
}