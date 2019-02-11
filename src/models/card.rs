pub enum CardValue {
    Ace,
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

pub enum Suit {
    Diamond,
    Club,
    Heart,
    Spade
}

pub struct Card {
    pub value: CardValue,
    pub suit: Suit
}