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
            CheckResult::LooksGood => {
                board.put_force(mv, player.to_stone());
                Ok(PutOutcome::Continue)
            },
            CheckResult::Win(p) => {
                board.put_force(mv, player.to_stone());
                Ok(PutOutcome::Win(p))
            },
            CheckResult::Draw => {
                board.put_force(mv, player.to_stone());
                Ok(PutOutcome::Draw)
            },
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