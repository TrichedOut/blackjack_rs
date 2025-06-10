use std::fmt::Display;
use crate::cards::suit::Suit;

#[derive(Debug, Copy, Clone)]
/**
 * Face / Ace card identifier
 */
pub enum CardFace {
    A, J, Q, K
}

#[derive(Copy, Clone)]
/**
 * A standard playing card
 */
pub struct Card {
    pub val: u32,               // The "index" value of the card (1-14)
    pub face: Option<CardFace>, // None for numbers. Face/Ace gets a CardFace
    pub suit: Suit,             // Card suit
}

impl Card {
    /**
     * Fully qualified constructor
     */
    pub fn new(val: u32, face: Option<CardFace>, suit: Suit) -> Card {
        Card {
            val, face, suit,
        }
    }
}

// make a card printable
impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // each suit has a unique color. color codes can be found
        // here: https://gist.github.com/fnky/458719343aabd01cfb17a3a4f7296797
        let color = match &self.suit {
            Suit::Spades => 51,     // Cyan
            Suit::Hearts => 206,    // Mid pink
            Suit::Diamonds => 171,  // Magenta
            Suit::Clubs => 159,     // Light blue
        };

        // Show a card as its suit symbol and its either 1) value for number 
        // cards or 2) face for face cards.
        // Each card is also colored based on suit, see above.
        write!(f, "{}", 
            match &self.face {
                Some(face) => format!("[38;5;{}m{}{:?}[0m", color, self.suit, face),
                None => format!("[38;5;{}m{}{}[0m", color, self.suit, self.val),
            }
        )
    }
}
