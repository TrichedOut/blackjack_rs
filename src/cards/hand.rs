use std::fmt::Display;

use crate::util::util::format_vec_string;

use super::{card::{Card, CardFace}, deck::Deck};

#[derive(Clone)]
pub struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    pub fn new() -> Hand {
        Hand {
            cards: vec![],
        }
    }

    pub fn draw_from(&mut self, deck: &mut Deck) {
        let card = deck.draw_pile.draw();
        match card {
            Some(c) => self.cards.push(c),
            None => {
                deck.reshuffle();

                let card = deck.draw_pile.draw();
                match card {
                    Some(c) => self.cards.push(c),
                    None => panic!("Deck has no cards after reshuffle. No cards to draw."),
                }
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

    pub fn is_splittable(&self) -> bool {
        if self.cards.len() != 2 {
            return false;
        }

        match (self.cards.get(0), self.cards.get(1)) {
            (Some(a), Some(b)) => return a.val == b.val,
            _ => return false,
        }
    }

    pub fn size(&self) -> usize {
        self.cards.len()
    }
}

impl Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format_vec_string(&self.cards))
    }
}
