use std::fmt::Display;

use crate::util::util::format_vec_string;

use super::{card::{Card, CardFace}, deck::Deck};

#[derive(Clone)]
/**
 * A `Hand` consists of a list of `Card`s and a status of if doubled down
 */
pub struct Hand {
    cards: Vec<Card>,
    doubled: bool,
}

impl Hand {
    /**
     * Create a new hand with no cards
     */
    pub fn new() -> Hand {
        Hand {
            cards: vec![],
            doubled: false,
        }
    }

    /**
     * Given a `Deck`, take a card from the deck.
     * If the `Deck` was empty, reshuffle the deck then draw.
     * Returns a copy of the card drawn
     */
    pub fn draw_from(&mut self, deck: &mut Deck) -> Card {
        // draw a card
        match deck.draw_pile.draw() {
            Some(c) => {
                // add the card to hand and return
                self.cards.push(c);
                return c;
            },
            None => {
                // reshuffle the deck and draw
                deck.reshuffle();
                return self.draw_from(deck);
            },
        }
    }

    /**
     * Take a card from this `Hand`.
     * Returns Some(Card) if there were cards in hand.
     * Returns None if there were no cards left in hand
     */
    pub fn take_card(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    /**
     * Give a card to this `Hand`.
     * Takes an optional to allow for soft None handling
     */
    pub fn give_card(&mut self, card: Option<Card>) {
        match card {
            Some(c) => self.cards.push(c),
            _ => (),
        }
    }

    /**
     * Creates a vector of the possible values of the `Hand`.
     * Will contain one value unless there are Aces present, in which case there
     * will be 1 + Aces values.
     * Does not filter for valid values, for this functionality, see `filter_value()`
     */
    pub fn value(&self) -> Vec<u32> {
        // create a vector of values, set initial value to 0
        let mut vals = Vec::new();
        vals.push(0);

        // look at each card,
        for card in self.cards.iter() {
            match card.face {
                // if the card is an ace, add 1 to all current values, then
                // add another value equal to 10 + max
                Some(CardFace::A) => {
                    vals.iter_mut().for_each(|v| *v += 1);
                    vals.push(*vals.last().unwrap() + 10);
                }
                // if the card is a non-ace face, add 10 to each value
                Some(_) => vals.iter_mut().for_each(|v| *v += 10),
                // otherwise, just add the cards value
                None => vals.iter_mut().for_each(|v| *v += card.val),
            }
        }

        // return accumulated values
        vals
    }

    /**
     * Creates a vector of the possible values of the `Hand`.
     * Will contain one value unless there are Aces present, in which case there
     * will be 1 + Aces values.
     * Returns a vector of only valid blackjack scores, hands with values of up
     * to 21
     */
    pub fn filter_value(&self) -> Vec<u32> {
        // filter the standard value
        self.value().iter().copied().filter(|v| *v <= 21).collect()
    }

    /**
     * Gets the true (scored) value of the hand. This is the biggest value not
     * greater than 21, or 0 if no such value exists
     */
    pub fn true_value(&self) -> u32 {
        *self.filter_value().iter().max().unwrap_or(&0)
    }

    /**
     * Returns whether the hand is a blackjack.
     * Is blackjack if and only if there are 2 cards that add to a true value of
     * 21
     */
    pub fn is_blackjack(&self) -> bool {
        self.true_value() == 21 && self.cards.len() == 2
    }

    /**
     * Returns whether the hand is busted.
     * Is busted if and only if there all values the hand can take are greater
     * than 21
     */
    pub fn is_busted(&self) -> bool {
        self.value().iter().min().unwrap_or(&22) > &21
    }

    /**
     * Returns whether the hand can be split during a game of Blackjack.
     * Is able to be split if and only if the hand has two cards and both cards
     * have the same value
     */
    pub fn is_splittable(&self) -> bool {
        // ensure length of 2
        if self.cards.len() != 2 {
            return false;
        }

        // check that the two values are the same
        match (self.cards.get(0), self.cards.get(1)) {
            (Some(a), Some(b)) => return a.val == b.val,
            _ => return false,
        }
    }

    /**
     * Set whether or not the hand has been doubled
     */
    pub fn set_doubled(&mut self, doubled: bool) {
        self.doubled = doubled;
    }

    /**
     * Check whether or not the hand has been doubled
     */
    pub fn is_doubled(&self) -> bool {
        self.doubled
    }
}

// make a hand printable
impl Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // show the hand as a comma separated list of cards
        write!(f, "{}", format_vec_string(&self.cards))
    }
}
