use prompted::input;

use crate::{cards::{deck::Deck, hand::Hand}, util::util::format_vec_string};

pub struct Game {
    deck: Deck,
    hands: Vec<Hand>,
    dealer: Hand,
    bet: usize,
}

impl Game {
    pub fn new(deck_count: usize, hand_count: usize, bet: usize) -> Game {
        let mut g = Game {
            deck: Deck::new(deck_count),
            hands: Vec::with_capacity(hand_count),
            dealer: Hand::new(),
            bet,
        };

        for _ in 0..hand_count {
            g.hands.push(Hand::new());
        }

        g
    }

    pub fn play(&mut self) {
        for _ in 0..2 {
            for hand in self.hands.iter_mut() {
                hand.draw_from(&mut self.deck);
            }
            self.dealer.draw_from(&mut self.deck);
        }

        if !self.blackjack_occured() {
            self.run_player_turns(0);
            self.run_dealer_turn();
            self.check_wins();
        }

        for hand in self.hands.iter_mut() {
            loop {
                match hand.take_card() {
                    Some(c) => self.deck.discard_pile.place(c),
                    None => break,
                }
            }
        }

        input!("\nEnter to continue...");

        loop {
            let again = input!("[2JPlay [A]gain\n[G]o Back\n:: ");
            match again {
                _ if again == "A" || again == "a" => return self.play(),
                _ if again == "G" || again == "g" => return,
                _ => (),
            }
        }
    }

    fn blackjack_occured(&self) -> bool {
        let mut blackjacks = Vec::new();
        for (i, hand) in self.hands.iter().enumerate() {
            if hand.filter_value().contains(&21) {
                blackjacks.push(i + 1);
            }
        }

        if self.dealer.filter_value().contains(&21) {
            blackjacks.push(0);
        }

        println!("[2JDealer: {}, {}", self.dealer, format_vec_string(&self.dealer.filter_value()));
        for (i, hand) in self.hands.iter().enumerate() {
            println!("{}: {}, {}", i + 1, hand, format_vec_string(&hand.filter_value()));
        }

        if blackjacks.len() != 0 {
            if blackjacks.contains(&0) {
                println!("Dealer got blackjack. Player loses all hands.");
            } else if blackjacks.len() == 1 {
                println!("Player got blackjack on hand {}.", blackjacks.get(0).unwrap());
            } else {
                println!("Player got blackjack on hands {}.", format_vec_string(&blackjacks));
            }

            return true;
        }

        false
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

    fn check_wins(&self) {
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

        for (i, hand) in winning {
            println!("Hand {}: {} ; ({})", i + 1, hand, format_vec_string(&hand.value()));
        }
    }
}

// pub fn new_game(decks: usize, hands: usize) {
//     let deck = Deck::new(decks);
//
//     play_game(deck, hands);
// }
//
// fn play_game(deck: Deck, num_hands: usize) {
//     let mut player_hands = Vec::with_capacity(num_hands);
//     for _ in 0..num_hands {
//         player_hands.push(Hand::new());
//     }
//
//     game_loop(deck, player_hands);
// }
//
// fn game_loop(mut deck: Deck, mut hands: Vec<Hand>) {
//     let hand_count = hands.len();
//
//     let mut dealer_hand = Hand::new();
//     for _ in 0..2 {
//         for hand in hands.iter_mut() {
//             hand.draw_from(&mut deck);
//         }
//         dealer_hand.draw_from(&mut deck);
//     }
//
//     if !blackjack_occured(&mut hands, &mut dealer_hand) {
//         hands = run_player_turns(&mut hands, &mut deck, 0);
//         run_dealer_turn(&mut dealer_hand, &mut deck);
//         check_wins(&mut hands, &mut dealer_hand);
//     }
//
//     for mut hand in hands {
//         loop {
//             match hand.take_card() {
//                 Some(c) => deck.discard_pile.place(c),
//                 None => break,
//             }
//         }
//     }
//
//     input!("\nEnter to continue...");
//
//     loop {
//         let again = input!("[2JPlay [A]gain\n[G]o Back\n:: ");
//         match again {
//             _ if again == "A" || again == "a" => return play_game(deck, hand_count),
//             _ if again == "G" || again == "g" => return,
//             _ => (),
//         }
//     }
// }
//
// fn blackjack_occured(hands: &mut Vec<Hand>, dealer_hand: &mut Hand) -> bool {
//     let mut blackjacks = Vec::new();
//     for (i, hand) in hands.iter().enumerate() {
//         if hand.filter_value().contains(&21) {
//             blackjacks.push(i + 1);
//         }
//     }
//
//     if dealer_hand.filter_value().contains(&21) {
//         blackjacks.push(0);
//     }
//
//     println!("[2JDealer: {}, {}", dealer_hand, format_vec_string(&dealer_hand.filter_value()));
//     for (i, hand) in hands.iter().enumerate() {
//         println!("{}: {}, {}", i + 1, hand, format_vec_string(&hand.filter_value()));
//     }
//
//     if blackjacks.len() != 0 {
//         if blackjacks.contains(&0) {
//             println!("Dealer got blackjack. Player loses all hands.");
//         } else if blackjacks.len() == 1 {
//             println!("Player got blackjack on hand {}.", blackjacks.get(0).unwrap());
//         } else {
//             println!("Player got blackjack on hands {}.", format_vec_string(&blackjacks));
//         }
//
//         return true;
//     }
//
//     false
// }
//
// fn run_player_turns(hands: &mut Vec<Hand>, deck: &mut Deck, start_hand: usize) -> Vec<Hand> {
//     let mut last_input = String::from("x");
//
//     for (i, hand) in hands.iter_mut().enumerate() {
//         if i < start_hand {
//             continue;
//         }
//
//         loop {
//             println!("[2JHand {}: {} ; ({})", i + 1, hand, format_vec_string(&hand.filter_value()));
//
//             let mut input;
//             let splittable = hand.is_splittable();
//             if splittable {
//                 input = input!("[H]it\n[S]tand\nsp[L]it\n:: ");
//             } else {
//                 input = input!("[H]it\n[S]tand\n:: ");
//             }
//
//             if input == "" {
//                 input = last_input.clone();
//             } else {
//                 last_input = input.clone();
//             }
//
//             match input {
//                 _ if input == "H" || input == "h" => hand.draw_from(deck),
//                 _ if input == "S" || input == "s" => break,
//                 _ if input == "L" || input == "l" && splittable => {
//                     let mut new_hand = Hand::new();
//                     new_hand.give_card(hand.take_card());
//
//                     hand.draw_from(deck);
//                     new_hand.draw_from(deck);
//
//                     let mut new_hands = hands.clone();
//                     new_hands.insert(i + 1, new_hand);
//
//                     return run_player_turns(&mut new_hands, deck, i);
//                 },
//                 _ => {}
//             }
//
//             if hand.filter_value().is_empty() {
//                 println!("[2JHand {}: {} ; ({})", i + 1, hand, format_vec_string(&hand.value()));
//                 input!("Hand has busted. Enter to continue...");
//                 break;
//             }
//         }
//     }
//
//     return hands.to_vec();
// }
//
// fn run_dealer_turn(dealer_hand: &mut Hand, deck: &mut Deck) {
//     while *dealer_hand.filter_value().iter().max().unwrap_or(&22) < 17 {
//         dealer_hand.draw_from(deck);
//     }
// }
//
// fn check_wins(hands: &mut Vec<Hand>, dealer_hand: &mut Hand) {
//     print!("[2J");
//
//     let dealer_val = dealer_hand.filter_value();
//     let dealer_max = dealer_val.iter().max().unwrap_or(&0);
//     let winning: Vec<(usize, &Hand)> = 
//         hands.iter()
//             .enumerate()
//             .filter(|hand| !hand.1.filter_value().is_empty())
//             .filter(|hand| hand.1.filter_value().iter().max().unwrap_or(&0) > dealer_max)
//             .collect();
//
//     if dealer_val.is_empty() {
//         println!("Dealer busted. All non-busted hands win:");
//     } else if winning.is_empty() {
//         println!("Dealer scored {}, you lost on all hands.", dealer_max);
//     } else {
//         println!("Dealer scored {}, you won on {} hands:", dealer_max, winning.len());
//     }
//
//     for (i, hand) in winning {
//         println!("Hand {}: {} ; ({})", i + 1, hand, format_vec_string(&hand.value()));
//     }
// }
