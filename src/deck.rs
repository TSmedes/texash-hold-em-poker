use crate::card::{Card, Rank, Suit};
use rand::seq::SliceRandom;
use rand::thread_rng;
/*
 * The struct type in Rust is a record type (Sebesta, 6.7)
 * It is an aggregate data type with one element named cards.
 */
pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Deck {
        let mut cards = Vec::new();
        for suit in vec![Suit::Spades, Suit::Hearts, Suit::Diamonds, Suit::Clubs] {
            for rank in vec![Rank::Two, Rank::Three, Rank::Four, Rank::Five, Rank::Six, Rank::Seven, Rank::Eight, Rank::Nine, Rank::Ten, Rank::Jack, Rank::Queen, Rank::King, Rank::Ace] {
                cards.push(Card { suit: suit.clone(), rank: rank.clone() });
            }
        }
        Deck { cards }
    }

    pub fn shuffle(&mut self) {
        self.cards.shuffle(&mut thread_rng());
    }

    pub fn deal(&mut self) -> Option<Card> {
        self.cards.pop()
    }
}