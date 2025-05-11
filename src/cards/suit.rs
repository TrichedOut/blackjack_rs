use std::fmt::Display;

#[derive(Copy, Clone)]
pub enum Suit {
    Spades, Hearts, Diamonds, Clubs
}

impl Display for Suit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
    pub fn from_val(val: usize) -> Suit {
        match val % 4 {
            0 => Suit::Spades,
            1 => Suit::Hearts,
            2 => Suit::Diamonds,
            _ => Suit::Clubs,
        }
    }
}
