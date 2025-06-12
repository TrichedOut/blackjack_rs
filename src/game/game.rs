use prompted::input;

use crate::{cards::{deck::Deck, hand::Hand}, util::util::format_vec_string};

use super::settings::GameSettings;

pub struct Game {
    deck: Deck,
    hands: Vec<Hand>,
    dealer: Hand,
}

impl Game {
    /**
     * Create a new game with `GameSettings`. 
     * Creates the empty hands needed for playing.
     * Creates the full deck needed for playing
     */
    pub fn new(settings: &GameSettings) -> Game {
        // create the game
        let mut g = Game {
            deck: Deck::new(settings.deck_count),
            hands: Vec::with_capacity(settings.hand_count),
            dealer: Hand::new(),
        };

        // add the hands
        for _ in 0..settings.hand_count {
            g.hands.push(Hand::new());
        }

        g
    }

    /**
     * Modifies this game to use new settings.
     * Removes the old hands and creates new ones if the number has changed.
     * Removes the old deck and creates a new one if the number has changed
     */
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

    /**
     * Play a game of blackjack with the current settings.
     * hands must be empty for correct functionality.
     * `spare_hands` is the amount of hands currently purchasable;
     * this is used for purchasing hands on split or double.
     * returns the amount of payout and remaining spare hands.
     * hands are emptied at the end of this function
     */
    pub fn play(&mut self, spare_hands: usize) -> (f32, usize) {
        // deal two cards to each player and the dealer, one at a 
        // time in a circle
        for _ in 0..2 {
            for hand in self.hands.iter_mut() {
                hand.draw_from(&mut self.deck);
            }
            self.dealer.draw_from(&mut self.deck);
        }

        let wins;
        let remaining_spares;
        // switch on if dealer got blackjack.
        // gets `if blackjack` and `num player blackjacks`
        match self.dealer_blackjack() {
            (true, amt) => {
                wins = amt as f32;
                remaining_spares = spare_hands;
            },
            _ => {
                // run the player turns, tracking bought hands.
                // `0` means to start at the first hand
                remaining_spares = self.run_player_turns(spare_hands, 0);
                // run the dealer's turn then run win detection and feedback
                self.run_dealer_turn();
                wins = self.check_wins() as f32;
            }
        }

        // discard all cards in all hands
        for hand in self.hands.iter_mut() {
            self.deck.discard_hand(hand);
            hand.set_doubled(false);
        }
        self.deck.discard_hand(&mut self.dealer);

        // return the amount of payout, and the amount of bought hands
        return (wins, spare_hands - remaining_spares)
    }

    /**
     * Check for a dealer blackjack, and if present, for player blackjacks.
     * Returns:
     *      if dealer got blackjack
     *      if above is true, number of hands player got blackjack on.
     *          if false, 0.
     */
    fn dealer_blackjack(&self) -> (bool, usize) {
        let mut blackjacks = Vec::new();

        // check for dealer blackjack
        if self.dealer.is_blackjack() {
            blackjacks.push(0);
        }

        // check for player blackjacks
        for (i, hand) in self.hands.iter().enumerate() {
            if hand.is_blackjack() {
                blackjacks.push(i + 1);
            }
        }

        // output
        println!("[2JDealer: {}, {}", self.dealer, format_vec_string(&self.dealer.filter_value()));
        for (i, hand) in self.hands.iter().enumerate() {
            println!("{}: {}, {}", i + 1, hand, format_vec_string(&hand.filter_value()));
        }

        // check counts
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

    /**
     * Run the player gameplay loop for the turn.
     * Takes number of buyable hands and starting hand index.
     * Returns number of remaining buyable hands
     */
    fn run_player_turns(&mut self, mut spare_hands: usize, from: usize) -> usize {
        // unusable last inut
        let mut last_input = String::from("x");

        // consume all previously played hands
        let mut iter = self.hands.iter_mut().enumerate();
        for _ in 0..from {
            iter.next();
        }

        // play the rest of the hands.
        for (i, hand) in iter {
            // play until break conditions are met:
            // 1) player gets blackjack
            // 2) player stands
            // 3) player busts
            // 4) player doubles down
            // 5) player splits, though this occurs with a return
            loop {
                // display current hand
                println!("[2JHand {}: {} ; ({})", i + 1, hand, format_vec_string(&hand.filter_value()));

                // if blackjack, stand automatically
                if hand.is_blackjack() {
                    input!("Hand is blackjack. Standing.\nEnter to continue...");
                    break;
                }

                // show available moves, get input
                let mut input;
                let splittable = hand.is_splittable();
                let buyable = spare_hands > 0;
                match (splittable, buyable) {
                    ( true,  true) => input = input!("[H]it\n[S]tand\n[D]ouble\nsp[L]it\n:: "),
                    (false,  true) => input = input!("[H]it\n[S]tand\n[D]ouble\n:: "),
                    ( ____, false) => input = input!("[H]it\n[S]tand\n:: "),
                }

                // if player just pressed 'enter', use previous move.
                // otherwise store move as new previous
                match input {
                    _ if input == "" => input = last_input.clone(),
                    _ => last_input = input.clone(),
                }

                // perform operations
                match input {
                    // hit
                    _ if input == "H" || input == "h" => {
                        hand.draw_from(&mut self.deck);
                    },
                    // split
                    _ if input == "S" || input == "s" => break,
                    // double
                    _ if (input == "D" || input == "d") && buyable => {
                        // consume a spare hand, draw a card, set doubled
                        spare_hands -= 1;
                        let drawn = hand.draw_from(&mut self.deck);
                        hand.set_doubled(true);

                        // display new hand and if busted
                        match hand.is_busted() {
                            true  => {
                                input!("[2JYou drew {}, busting your hand:\n{} ; ({})\n\nEnter to continue...", drawn, hand, format_vec_string(&hand.value()));
                            },
                            false  => {
                                input!("[2JYou drew {}. Your hand is now:\n{} ; ({})\n\nEnter to continue...", drawn, hand, format_vec_string(&hand.filter_value()));
                            },
                        }
                        // go to next hand
                        break;
                    }
                    // split
                    _ if (input == "L" || input == "l") && splittable && buyable => return self.split_hand(spare_hands, i),
                    // invalid
                    _ => {}
                }

                // check for a busted hand
                if hand.filter_value().is_empty() {
                    println!("[2JHand {}: {} ; ({})", i + 1, hand, format_vec_string(&hand.value()));
                    input!("Hand has busted. Enter to continue...");
                    break;
                }
            }
        }

        spare_hands
    }

    /**
     * Handle splitting a hand.
     * Meant to be used from run_player_turns by returning this function.
     */
    fn split_hand(&mut self, spare_hands: usize, ndx: usize) -> usize {
        // get the current hand. create a new hand, take one card from current
        // and give to new hand.
        let prev_hand = self.hands.get_mut(ndx).unwrap();
        let mut new_hand = Hand::new();
        new_hand.give_card(prev_hand.take_card());

        // each hand draws 1 card
        prev_hand.draw_from(&mut self.deck);
        new_hand.draw_from(&mut self.deck);

        // add the hand, continue playing from current hand
        self.hands.insert(ndx + 1, new_hand);
        return self.run_player_turns(spare_hands - 1, ndx);
    }

    /**
     * Runs the dealer's turn.
     * Plays by hitting until >=17
     */
    fn run_dealer_turn(&mut self) {
        // calc value once per iteration.
        // value is only 0 if busted
        let mut value = self.dealer.true_value();
        while value < 17 && value != 0 {
            self.dealer.draw_from(&mut self.deck);
            value = self.dealer.true_value();
        }
    }

    /**
     * Check win count and calculate payout scalar.
     */
    fn check_wins(&self) -> f32 {
        print!("[2J");

        // get the dealer's hand value, get hands that beat the dealer
        let dealer_max = self.dealer.true_value();
        let winning: Vec<(usize, &Hand)> = 
            self.hands.iter()
            .enumerate()
            .filter(|hand| hand.1.true_value() > dealer_max)
            .collect();

        // display corresponding header
        println!("Dealer: {} ; ({})", self.dealer, format_vec_string(&self.dealer.value()));
        if self.dealer.is_busted() {
            println!("Dealer busted. All non-busted hands win:");
        } else if winning.is_empty() {
            println!("Dealer scored {}, you lost on all hands.", dealer_max);
        } else {
            println!("Dealer scored {}, you won on {} hands:", dealer_max, winning.len());
        }

        // take each hand that beat the dealer and convert it to its win amount
        let payouts: Vec<f32> = winning.iter().map(|hand| {
            if hand.1.is_blackjack() { // blackjacks get 1.5x bet
                1.5
            } else if hand.1.is_doubled() { // doubles get 4x bet
                4.0
            } else { // standard hands get 2x bet
                2.0
            }
        }).collect();

        // print all winning hands
        for (i, hand) in winning {
            println!("Hand {}: {} ; ({})", i + 1, hand, format_vec_string(&hand.filter_value()));
        }

        // sum the payouts and return
        payouts.iter().sum()
    }
}
