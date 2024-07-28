use crate::world::actions::action_error::ActionError::NotEnoughResources;
use crate::world::actions::action_error::ActionResult;
use crate::world::board::Board;
use crate::world::resources::Resources;

pub fn remove_resources(board: Board, resources: &Resources) -> ActionResult<Board> {
    let current_resources = board.current_resources.clone();
    if current_resources.value() >= resources.value() {
        Ok(Board {
            current_resources: &current_resources - resources,
            ..board
        })
    } else {
        Err(NotEnoughResources(
            Board {
                current_resources: Resources::new(0),
                ..board
            },
            resources.clone(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::world::actions::action_error::ActionError;
    use crate::world::actions::remove_resources::remove_resources;
    use crate::world::board::Board;
    use crate::world::resources::Resources;

    #[test]
    fn remove_resources_success() {
        let board = Board {
            current_resources: Resources::new(10),
            turns_remaining: 0,
            ..Board::empty()
        };

        let expected_board = Board {
            current_resources: Resources::new(0),
            ..board.clone()
        };

        let result = remove_resources(board, &Resources::new(10)).unwrap();

        assert_eq!(result, expected_board);
    }

    #[test]
    fn remove_resources_failure() {
        let board = Board {
            current_resources: Resources::new(10),
            turns_remaining: 0,
            ..Board::empty()
        };

        let expected_board = Board {
            current_resources: Resources::new(0),
            ..board.clone()
        };

        let result = remove_resources(board, &Resources::new(11)).unwrap_err();

        assert_eq!(
            result,
            ActionError::NotEnoughResources(expected_board, Resources::new(11))
        );
    }
}
