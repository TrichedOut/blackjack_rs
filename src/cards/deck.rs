use super::{card::{Card, CardFace}, hand::Hand, suit::Suit};
use rand::prelude::SliceRandom;

pub struct Deck {
    size: usize,
    pub draw_pile: Pile,
    pub discard_pile: Pile,
}

pub struct Pile {
    cards: Vec<Card>,
}

impl Pile {
    // create an empty pile with assumed size of 1 deck (52 cards)
    pub fn new_empty(decks: usize) -> Pile {
        Pile {
            cards: Vec::with_capacity(52 * decks),
        }
    }

    // create a new pile with n decks worth of cards
    pub fn new_full(decks: usize) -> Pile {
        let mut p = Pile::new_empty(decks);

        for i in 0..p.cards.capacity() {
            let s = i / 13;
            let suit = Suit::from_val(s);

            let val = ((i % 13) + 1).try_into().unwrap();

            let face = match val {
                01 => Some(CardFace::A),
                11 => Some(CardFace::J),
                12 => Some(CardFace::Q),
                13 => Some(CardFace::K),
                __  => None,
            };

            p.cards.push(Card::new(val, face, suit));
        }

        p
    }

    pub fn draw(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    pub fn place(&mut self, card: Card) {
        self.cards.push(card);
    }

    pub fn shuffle(&mut self) {
        self.cards.shuffle(&mut rand::rng());
    }
}

impl Deck {
    pub fn new(decks: usize) -> Deck {
        let mut deck = Deck {
            size: decks,
            draw_pile: Pile::new_full(decks),
            discard_pile: Pile::new_empty(decks),
        };

        deck.draw_pile.shuffle();
        deck
    }

    pub fn discard_hand(&mut self, hand: &mut Hand) {
        loop {
            match hand.take_card() {
                Some(c) => self.discard_pile.place(c),
                None    => break,
            }
        }
    }

    pub fn reshuffle(&mut self) {
        self.draw_pile.cards.extend(self.discard_pile.cards.clone());
        self.discard_pile.cards.clear();
        self.draw_pile.shuffle();
    }

    pub fn size(&self) -> usize {
        self.size
    }
}
