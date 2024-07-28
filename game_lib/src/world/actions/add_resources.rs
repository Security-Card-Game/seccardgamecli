use crate::world::board::Board;
use crate::world::resources::Resources;

pub fn add_resources(board: Board, resources: &Resources) -> Board {
    Board {
        current_resources: &board.current_resources + resources,
        ..board
    }
}

#[cfg(test)]
mod tests {
    use crate::world::actions::add_resources::add_resources;
    use crate::world::board::Board;
    use crate::world::resources::Resources;

    #[test]
    fn add_resources_success() {
        let board = Board {
            current_resources: Resources::new(10),
            turns_remaining: 0,
            ..Board::empty()
        };

        let expected_board = Board {
            current_resources: Resources::new(11),
            ..board.clone()
        };

        let result = add_resources(board, &Resources::new(1));

        assert_eq!(result, expected_board);
    }
}
