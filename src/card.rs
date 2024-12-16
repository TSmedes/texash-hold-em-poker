/*
 * This is an enumeration type (Sebesta, 6.4)
 * It enumerates the four suits of a deck of cards
 */
#[derive(Debug, Clone)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

/*
 * This is an enumeration type (Sebesta, 6.4)
 * It enumerates the ranks of a deck of cards
 */
#[derive(Debug, Clone)]
pub enum Rank {
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
    King,
    Ace,
}

/*
 * This is a structure type (Sebesta, 6.4)
 * It represents a card in a deck of cards
 */
#[derive(Clone, Debug)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}