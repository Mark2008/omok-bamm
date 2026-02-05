//! This is a test that should expose the bug described in the original issue
//! The test demonstrates where data is written but then read as None

#[cfg(test)]
mod board_bug_test {
    use crate::core::board::{Board, Move, Stone};

    #[test]
    fn test_put_get_inconsistency() {
        // This test simulates the exact bug scenario:
        // 1. put() is called and data appears to be written
        // 2. get() is called on same move and returns Stone::None instead of the written stone

        let mut board = Board::blank();

        // Create a move
        let mv = Move::new(7, 7).unwrap();

        // Verify initial state
        assert_eq!(board.get(mv), Stone::None);

        // Put a stone (this should work correctly)
        let result = board.put(mv, Stone::Black);
        assert_eq!(result, true);

        // This assertion would fail if the bug exists:
        // The stone should be readable after being written
        let retrieved = board.get(mv);

        // If this fails, then the bug exists - data was written but not read back
        assert_eq!(
            retrieved,
            Stone::Black,
            "Bug detected: put() wrote data but get() returned None instead of Black stone"
        );
    }

    #[test]
    fn test_put_then_get_consistency() {
        // Comprehensive test for consistency
        let mut board = Board::blank();

        // Test multiple positions
        let test_positions = [
            Move::new(0, 0).unwrap(),
            Move::new(7, 7).unwrap(),
            Move::new(14, 14).unwrap(),
            Move::new(5, 9).unwrap(),
        ];

        for (i, mv) in test_positions.iter().enumerate() {
            // Verify initial state
            assert_eq!(
                board.get(*mv),
                Stone::None,
                "Position {} should start as None",
                i
            );

            // Put a stone
            board.put(*mv, Stone::White);

            // Immediately check it was written
            let retrieved = board.get(*mv);
            assert_eq!(
                retrieved,
                Stone::White,
                "Position {} should contain White stone after put_unchecked",
                i
            );
        }

        // Verify all positions have correct values
        for (i, mv) in test_positions.iter().enumerate() {
            let retrieved = board.get(*mv);
            assert_eq!(
                retrieved,
                Stone::White,
                "Position {} still contains correct stone",
                i
            );
        }
    }
}
