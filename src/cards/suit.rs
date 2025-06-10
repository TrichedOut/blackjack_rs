use std::fmt::Display;

#[derive(Copy, Clone)]
/**
 * Each suit option
 */
pub enum Suit {
    Spades, Hearts, Diamonds, Clubs
}

// make a suit printable
impl Display for Suit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // simple match of various symbols
        write!(f, "{}", 
            String::from(match self {
                Suit::Spades => "@",
                Suit::Hearts => "#",
                Suit::Diamonds => "$",
                Suit::Clubs => "%",
            }
        ))
    }
}

impl Suit {
    /**
     * Get a suit based off of a size.
     * Operates in the (mod 4) number space
     */
    pub fn from_val(val: usize) -> Suit {
        match val % 4 {
            0 => Suit::Spades,
            1 => Suit::Hearts,
            2 => Suit::Diamonds,
            _ => Suit::Clubs,
        }
    }
}
