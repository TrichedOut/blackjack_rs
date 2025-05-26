use prompted::input;

use crate::{cards::{deck::Deck, hand::Hand}, util::util::format_vec_string};

use super::settings::GameSettings;

pub struct Game {
    deck: Deck,
    hands: Vec<Hand>,
    dealer: Hand,
}

impl Game {
    pub fn new(settings: &GameSettings) -> Game {
        let mut g = Game {
            deck: Deck::new(settings.deck_count),
            hands: Vec::with_capacity(settings.hand_count),
            dealer: Hand::new(),
        };

        for _ in 0..settings.hand_count {
            g.hands.push(Hand::new());
        }

        g
    }

    pub fn update_settings(&mut self, settings: &GameSettings) {
        if self.deck.size() != settings.deck_count {
            self.deck = Deck::new(settings.deck_count);
        }

        if self.hands.len() != settings.hand_count {
            self.hands.clear();
            for _ in 0..settings.hand_count {
                self.hands.push(Hand::new());
            }
        }
    }

    pub fn play(&mut self) -> f32 {
        for _ in 0..2 {
            for hand in self.hands.iter_mut() {
                hand.draw_from(&mut self.deck);
            }
            self.dealer.draw_from(&mut self.deck);
        }

        let wins;
        match self.dealer_blackjack() {
            (true, amt) => {
                wins = amt as f32;
            },
            _ => {
                self.run_player_turns(0);
                self.run_dealer_turn();
                wins = self.check_wins() as f32;
            }
        }

        for hand in self.hands.iter_mut() {
            self.deck.discard_hand(hand);
        }
        self.deck.discard_hand(&mut self.dealer);

        wins
    }

    fn dealer_blackjack(&self) -> (bool, usize) {
        let mut blackjacks = Vec::new();

        if Game::is_blackjack(&self.dealer) {
            blackjacks.push(0);
        }

        for (i, hand) in self.hands.iter().enumerate() {
            if Game::is_blackjack(&hand) {
                blackjacks.push(i + 1);
            }
        }

        println!("[2JDealer: {}, {}", self.dealer, format_vec_string(&self.dealer.filter_value()));
        for (i, hand) in self.hands.iter().enumerate() {
            println!("{}: {}, {}", i + 1, hand, format_vec_string(&hand.filter_value()));
        }

        let len = blackjacks.len();
        let dealer_jack = blackjacks.contains(&0);

        if dealer_jack && len == 1 {    // only dealer blackjack

            println!("Dealer got blackjack. Player loses all hands.");

        } else if dealer_jack {         // both dealer and player blackjack

            blackjacks.remove(0);
            if len == 2 {
                println!("Both Player and Dealer got blackjack. Player regains bet for hand {}.", format_vec_string(&blackjacks));
            } else {
                println!("Both Player and Dealer got blackjack. Player regains bet for hands {}.", format_vec_string(&blackjacks));
            }

        } else {
            return (false, 0);
        }

        return (true, len - 1);
    }

    fn run_player_turns(&mut self, from: usize) {
        let mut last_input = String::from("x");

        let mut iter = self.hands.iter_mut().enumerate();
        for _ in 0..from {
            iter.next();
        }

        for (i, hand) in iter {
            loop {
                println!("[2JHand {}: {} ; ({})", i + 1, hand, format_vec_string(&hand.filter_value()));

                if Game::is_blackjack(&hand) {
                    input!("Hand is blackjack. Standing.\nEnter to continue...");
                    break;
                }

                let mut input;
                let splittable = hand.is_splittable();
                match splittable {
                    true => input = input!("[H]it\n[S]tand\nsp[L]it\n:: "),
                    false => input = input!("[H]it\n[S]tand\n:: "),
                }

                match input {
                    _ if input == "" => input = last_input.clone(),
                    _ => last_input = input.clone(),
                }

                match input {
                    _ if input == "H" || input == "h" => hand.draw_from(&mut self.deck),
                    _ if input == "S" || input == "s" => break,
                    _ if input == "L" || input == "l" && splittable => return self.split_hand(i),
                    _ => {}
                }

                if hand.filter_value().is_empty() {
                    println!("[2JHand {}: {} ; ({})", i + 1, hand, format_vec_string(&hand.value()));
                    input!("Hand has busted. Enter to continue...");
                    break;
                }
            }
        }
    }

    fn split_hand(&mut self, ndx: usize) {
        let prev_hand = self.hands.get_mut(ndx).unwrap();
        let mut new_hand = Hand::new();
        new_hand.give_card(prev_hand.take_card());

        prev_hand.draw_from(&mut self.deck);
        new_hand.draw_from(&mut self.deck);

        self.hands.insert(ndx + 1, new_hand);
        self.run_player_turns(ndx);
    }

    fn run_dealer_turn(&mut self) {
        while self.dealer.filter_value().iter().max().unwrap_or(&22) < &17 {
            self.dealer.draw_from(&mut self.deck);
        }
    }

    fn check_wins(&self) -> f32 {
        print!("[2J");

        let dealer_val = self.dealer.filter_value();
        let dealer_max = dealer_val.iter().max().unwrap_or(&0);
        let winning: Vec<(usize, &Hand)> = 
            self.hands.iter()
            .enumerate()
            .filter(|hand| !hand.1.filter_value().is_empty())
            .filter(|hand| hand.1.filter_value().iter().max().unwrap_or(&0) > dealer_max)
            .collect();

        if dealer_val.is_empty() {
            println!("Dealer busted. All non-busted hands win:");
        } else if winning.is_empty() {
            println!("Dealer scored {}, you lost on all hands.", dealer_max);
        } else {
            println!("Dealer scored {}, you won on {} hands:", dealer_max, winning.len());
        }

        let payouts: Vec<f32> = winning.iter().map(|hand| { if Game::is_blackjack(hand.1) { 1.5 } else { 2. }} ).collect();
        for (i, hand) in winning {
            println!("Hand {}: {} ; ({})", i + 1, hand, format_vec_string(&hand.filter_value()));
        }

        payouts.iter().sum()
    }

    fn is_blackjack(hand: &Hand) -> bool {
        if hand.size() != 2 {
            return false;
        }

        return hand.filter_value().contains(&21);
    }
}
