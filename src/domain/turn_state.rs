#[derive(Copy, Clone, PartialEq)]
pub enum TurnState {
    AwaitingInput,
    PlayerTurn,
    EnemyTurn,
}
