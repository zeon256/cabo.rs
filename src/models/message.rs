#[derive(Debug)]
pub enum Message {
    Init {},
    SelfDraw,
    SelfPeek,
    Place,
    Swap,
    Kill,
    OtherDraw,
    OtherPeek,
    Turn,
    End,
}

#[derive(Debug)]
pub struct WSError {
    pub code: u16,
    pub message: String
}

#[derive(Debug)]
pub enum GameError {
    NotEnoughPlayers,
    EmptyLobbyPassword,
    WrongPrevGameState
}
