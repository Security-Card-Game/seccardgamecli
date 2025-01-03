use crate::world::board::Board;
use crate::world::reputation::Reputation;

pub fn subtract_reputation(board: Board, reputation: &Reputation) -> Board {
    Board {
        current_reputation: &board.current_reputation - reputation,
        ..board
    }
}

#[cfg(test)]
mod tests {
    use crate::world::actions::subtract_reputation::subtract_reputation;
    use crate::world::board::Board;
    use crate::world::reputation::Reputation;

    #[test]
    fn subtract_reputation_success() {
        let board = Board {
            current_reputation: Reputation::new(10),
            turns_remaining: 0,
            ..Board::empty()
        };

        let expected_board = Board {
            current_reputation: Reputation::new(9),
            ..board.clone()
        };

        let result = subtract_reputation(board, &Reputation::new(1));

        assert_eq!(result, expected_board);
    }
}
