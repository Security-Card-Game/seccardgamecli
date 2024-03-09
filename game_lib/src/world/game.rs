use crate::world::current_turn::CurrentBoard;

#[derive(Debug, Clone)]
pub enum Game {
    Start(CurrentBoard),
    InProgress(CurrentBoard),
    Finished,
}
