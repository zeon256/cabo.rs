use num_derive;
use num_traits::FromPrimitive;
use std::fmt;

#[derive(Debug)]
pub struct CardValue {
    pub value: Value,
    pub suit: Suit
}

impl CardValue {
    pub fn new(value: Value, suit: Suit) -> CardValue {
        CardValue { value, suit }
    }

    pub fn from_primitive(value: i32, suit: i32) -> CardValue {
        CardValue {
            value: FromPrimitive::from_i32(value).unwrap(),
            suit: FromPrimitive::from_i32(suit).unwrap()
        }
    }
}

impl fmt::Display for CardValue {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(format!("{:?} {:?}", self.value, self.suit).as_str());
        Ok(())
    }
}

#[derive(Debug)]
pub struct CardPosition {
    pub player_id: u32,
    pub position: Position
}

#[derive(Debug)]
pub struct Card {
    pub value: Option<CardValue>,
    pub position: CardPosition
}

impl Card {
    pub fn new(value: Option<CardValue>, position: CardPosition) -> Card {
        Card { value, position}
    }
}

#[derive(FromPrimitive, ToPrimitive, Debug)]
pub enum Value {
    Ace = 1,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King
}

#[derive(FromPrimitive, ToPrimitive, Debug)]
pub enum Suit {
    Diamond = 1,
    Club,
    Heart,
    Spade
}

#[derive(FromPrimitive, ToPrimitive, Debug)]
pub enum Position {
    Left,
    Center,
    Right
}