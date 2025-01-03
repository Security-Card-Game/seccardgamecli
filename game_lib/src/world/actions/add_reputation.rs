use crate::world::board::Board;
use crate::world::reputation::Reputation;

pub fn add_reputation(board: Board, reputation: &Reputation) -> Board {
    Board {
        current_reputation: &board.current_reputation + reputation,
        ..board
    }
}

#[cfg(test)]
mod tests {
    use crate::world::actions::add_reputation::add_reputation;
    use crate::world::board::Board;
    use crate::world::reputation::Reputation;

    #[test]
    fn add_reputation_success() {
        let board = Board {
            current_reputation: Reputation::new(10),
            turns_remaining: 0,
            ..Board::empty()
        };

        let expected_board = Board {
            current_reputation: Reputation::new(11),
            ..board.clone()
        };

        let result = add_reputation(board, &Reputation::new(1));

        assert_eq!(result, expected_board);
    }
}
