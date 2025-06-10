use super::{card::{Card, CardFace}, hand::Hand, suit::Suit};
use rand::prelude::SliceRandom;

/**
 * A Deck has a draw pile and discard pile.
 * Also stores the number of 52 card decks are in the final Deck
 */
pub struct Deck {
    size: usize,
    pub draw_pile: Pile,
    pub discard_pile: Pile,
}

/**
 * Standard pile of cards
 */
pub struct Pile {
    cards: Vec<Card>,
}

impl Pile {
    /**
     * Create a pile of cards with space for `decks` decks of cards
     */
    pub fn new_empty(decks: usize) -> Pile {
        Pile {
            cards: Vec::with_capacity(52 * decks),
        }
    }

    /**
     * Create a pile of cards with cards for `decks` decks of cards
     */
    pub fn new_full(decks: usize) -> Pile {
        // start with an empty pile
        let mut p = Pile::new_empty(decks);

        // iterate on size measured by capacity
        for i in 0..p.cards.capacity() {
            // suit index to suit
            let s = i / 13;
            let suit = Suit::from_val(s);

            // val is 1-14, so %13+1
            let val = ((i % 13) + 1) as u32;

            // get the display face of the card
            let face = match val {
                01 => Some(CardFace::A),
                11 => Some(CardFace::J),
                12 => Some(CardFace::Q),
                13 => Some(CardFace::K),
                __ => None,
            };

            p.cards.push(Card::new(val, face, suit));
        }

        p
    }

    /**
     * Draw the top card from the pile.
     * Returns Some(Card) if there are cards in the pile.
     * Returns None if there are no cards left in the pile
     */
    pub fn draw(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    /**
     * Place a card on the top of the pile
     */
    pub fn place(&mut self, card: Card) {
        self.cards.push(card);
    }

    /**
     * Shuffle the pile
     */
    pub fn shuffle(&mut self) {
        self.cards.shuffle(&mut rand::rng());
    }
}

impl Deck {
    /**
     * Create a new single `Deck` with the cards of `decks` decks.
     * Comes with both a draw pile and discard pile
     */
    pub fn new(decks: usize) -> Deck {
        // start with a full draw pile and empty discard pile
        let mut deck = Deck {
            size: decks,
            draw_pile: Pile::new_full(decks),
            discard_pile: Pile::new_empty(decks),
        };

        // shuffle the draw pile then return
        deck.draw_pile.shuffle();
        deck
    }

    /**
     * Given a `Hand`, take all cards from the `Hand` and place them on top of
     * the discard pile
     */
    pub fn discard_hand(&mut self, hand: &mut Hand) {
        // while cards are in hand, take the card and place in discard
        loop {
            match hand.take_card() {
                Some(c) => self.discard_pile.place(c),
                None    => break,
            }
        }
    }

    /**
     * Take all cards in the discard pile, place them on top of the draw pile,
     * then shuffle all cards.
     * Empties the discard pile
     */
    pub fn reshuffle(&mut self) {
        self.draw_pile.cards.extend(self.discard_pile.cards.clone());
        self.discard_pile.cards.clear();
        self.draw_pile.shuffle();
    }

    /**
     * Returns the number of decks contained in this `Deck`
     */
    pub fn size(&self) -> usize {
        self.size
    }
}
