use std::fmt::Display;

use crate::util::util::format_vec_string;

use super::{card::{Card, CardFace}, deck::Deck};

#[derive(Clone)]
pub struct Hand {
    cards: Vec<Card>,
    doubled: bool,
}

impl Hand {
    pub fn new() -> Hand {
        Hand {
            cards: vec![],
            doubled: false,
        }
    }

    pub fn draw_from(&mut self, deck: &mut Deck) -> Card {
        let card = deck.draw_pile.draw();
        match card {
            Some(c) => {
                self.cards.push(c);
                return c;
            },
            None => {
                deck.reshuffle();
                return self.draw_from(deck);
            },
        }
    }

    pub fn take_card(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    pub fn give_card(&mut self, card: Option<Card>) {
        match card {
            Some(c) => self.cards.push(c),
            _ => (),
        }
    }

    pub fn value(&self) -> Vec<u32> {
        let mut vals = Vec::new();
        vals.push(0);

        for card in self.cards.iter() {
            match card.face {
                Some(CardFace::A) => {
                    vals.iter_mut().for_each(|v| *v += 1);
                    vals.push(*vals.last().unwrap() + 10);
                }
                Some(_) => vals.iter_mut().for_each(|v| *v += 10),
                None => vals.iter_mut().for_each(|v| *v += card.val),
            }
        }

        vals
    }

    pub fn filter_value(&self) -> Vec<u32> {
        self.value().iter().copied().filter(|v| *v <= 21).collect()
    }

    pub fn true_value(&self) -> u32 {
        *self.filter_value().iter().max().unwrap_or(&0)
    }

    pub fn is_blackjack(&self) -> bool {
        self.true_value() == 21 && self.cards.len() == 2
    }

    pub fn is_busted(&self) -> bool {
        self.value().iter().min().unwrap_or(&22) > &21
    }

    pub fn is_splittable(&self) -> bool {
        if self.cards.len() != 2 {
            return false;
        }

        match (self.cards.get(0), self.cards.get(1)) {
            (Some(a), Some(b)) => return a.val == b.val,
            _ => return false,
        }
    }

    pub fn set_doubled(&mut self, doubled: bool) {
        self.doubled = doubled;
    }

    pub fn is_doubled(&self) -> bool {
        self.doubled
    }
}

impl Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format_vec_string(&self.cards))
    }
}
