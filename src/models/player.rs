use super::card::CardValue;

#[derive(Debug)]
pub struct Player {
    pub id: u32,
    pub username: String,
    pub cards: Vec<CardValue>,
}
