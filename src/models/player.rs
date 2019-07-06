use super::card::CardValue;

#[derive(Debug)]
pub struct Player {
    /// id is unique to the game only. There can be multiple ppl with the same id in diff games
    pub id: u32,
    /// username is unique to the game only. There can be multiple ppl with the same username in diff games
    pub username: String,
    /// cards player has on board, max is 3
    pub cards: Vec<CardValue>,
}

impl Player {
    pub fn new(id: u32, username: &str) -> Self {
        Player { id, username: username.to_string(), cards: Vec::with_capacity(3) }
    }
}
