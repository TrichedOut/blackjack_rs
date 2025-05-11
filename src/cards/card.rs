use std::fmt::Display;
use crate::cards::suit::Suit;

#[derive(Debug, Copy, Clone)]
pub enum CardFace {
    A, J, Q, K
}

#[derive(Copy, Clone)]
pub struct Card {
    pub val: u32,
    pub face: Option<CardFace>,
    pub suit: Suit,
}

impl Card {
    pub fn new(val: u32, face: Option<CardFace>, suit: Suit) -> Card {
        Card {
            val, face, suit,
        }
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let color = match &self.suit {
            Suit::Spades => 51,
            Suit::Hearts => 206,
            Suit::Diamonds => 171,
            Suit::Clubs => 159,
        };

        write!(f, "{}", 
            match &self.face {
                Some(face) => format!("[38;5;{color}m{}{:?}[0m", self.suit, face),
                None => format!("[38;5;{color}m{}{}[0m", self.suit, self.val),
            }
        )
    }
}
