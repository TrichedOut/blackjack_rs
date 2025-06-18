use prompted::input;

use crate::{cards::{deck::Deck, hand::Hand}, util::{input::{validated_input}, util::format_vec_string}};

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
        // get the current size of the hand. needed to remove hands if split
        let hand_count = self.hands.len();

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
        (wins, remaining_spares) = match self.dealer_blackjack() {
            (true, amt) => {
                (amt as f32, spare_hands)
            },
            _ => {
                // run the player turns, tracking bought hands.
                // `0` means to start at the first hand
                let remaining_spares = self.run_player_turns(spare_hands, 0);

                // run the dealer's turn then run win detection and feedback
                self.run_dealer_turn();
                (self.check_wins() as f32, remaining_spares)
            }
        };

        // discard all cards in all hands
        for hand in self.hands.iter_mut() {
            self.deck.discard_hand(hand);
            hand.set_doubled(false);
        }
        self.deck.discard_hand(&mut self.dealer);

        // remove any hands gained from splitting
        for _ in 0..(self.hands.len() - hand_count) {
            self.hands.pop();
        }

        // return the amount of payout, and the amount of bought hands
        (wins, spare_hands - remaining_spares)
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
        println!("\n[2JDealer: {}, {}", self.dealer, format_vec_string(&self.dealer.filter_value()));
        for (i, hand) in self.hands.iter().enumerate() {
            println!("{}: {}, {}", i + 1, hand, format_vec_string(&hand.filter_value()));
        }

        // check counts
        let len = blackjacks.len();
        let dealer_jack = blackjacks.contains(&0);
        match (dealer_jack, len) {
            (true, 1) => { // only dealer blackjack
                println!("Dealer got blackjack. Player loses all hands.");
                (true, 0)
            },

            (true, _) => { // both dealer and player blackjack
                blackjacks.remove(0);
                if len == 2 {
                    println!("Both Player and Dealer got blackjack. Player regains bet for hand {}.", format_vec_string(&blackjacks));
                } else {
                    println!("Both Player and Dealer got blackjack. Player regains bet for hands {}.", format_vec_string(&blackjacks));
                }

                (true, len - 1)
            },

            (_, _) => (false, 0),
        }
    }

    /**
     * Run the player gameplay loop for the turn.
     * Takes number of buyable hands and starting hand index.
     * Returns number of remaining buyable hands
     */
    fn run_player_turns(&mut self, mut spare_hands: usize, from: usize) -> usize {
        let mut last_input = 'x';
        let mut cl = self.hands.clone();
        let iter = cl.iter_mut().enumerate().skip(from);


        // for each hand
        for (n, mut hand) in iter {
            // keep going until turn ends
            loop {
                // show dealer hand
                print!("[2J\nDealer Hand: {}, ??\n\nOptions: ", self.dealer.top_card());

                // check split and buy status
                let splittable = hand.is_splittable();
                let buyable = spare_hands > 0;

                // get and display hand play options
                let options = match (splittable, buyable) {
                    (true, true) => {
                        println!("[H]it, [S]tand, [D]ouble, sp[L]it");
                        vec!['h', 's', 'd', 'l', 'H', 'S', 'D', 'L']
                    },
                    (false, true) => {
                        println!("[H]it, [S]tand, [D]ouble");
                        vec!['h', 's', 'd', 'H', 'S', 'D']
                    },
                    (_, false) => {
                        println!("[H]it, [S]tand");
                        vec!['h', 's', 'H', 'S']
                    },
                };

                // display all hands, revealing that which has been played
                let mut out = String::new();
                for (i, hand) in self.hands.iter().enumerate() {
                    match (i < n, i == n) {
                        (true , false) => {
                            match hand.is_busted() {
                                true  => println!("Hand {}: {} ; ({}, busted)", i + 1, hand, format_vec_string(&hand.value())),
                                false => println!("Hand {}: {} ; ({})", i + 1, hand, format_vec_string(&hand.filter_value())),
                            }
                        },
                        (false, true) => {
                            out = format!("Hand {}: {} ; ({}) <- ", i + 1, hand, format_vec_string(&hand.filter_value()));
                            println!()
                        },
                        (_    , _    ) => println!("Hand {}: {}, ??", i + 1, hand.top_card()),
                    }
                }

                // move cursor to correct hand pos
                let v_shift = self.hands.len() - n;
                print!("[{}A{out}", v_shift);

                // handle input
                let (to_break, bought) = match validated_input(|c| options.contains(&c), |s: String| s.len() <= 1) {
                    input if input.len() == 0 => {
                        self.handle_play_input(&mut hand, last_input)
                    },
                    input => {
                        last_input = input.chars().nth(0).unwrap();
                        self.handle_play_input(&mut hand, last_input)
                    },
                };
                self.hands[n] = hand.clone();

                match (to_break, bought) {
                    (true, true) => {
                        spare_hands -= 1;
                        break
                    },
                    (true, false) => break,
                    (false, true) => {
                        spare_hands -= 1;
                        return self.split_hand(spare_hands, n)
                    },
                    (false, false) => (),
                }
                if bought {
                    spare_hands -= 1;
                }

                if hand.is_busted() {
                    break;
                }
            }
        }

        // show state before moving on
        print!("[2J\nDealer Hand: {}, ??\n\n\n", self.dealer.top_card());
        for (i, hand) in self.hands.iter().enumerate() {
            match hand.is_busted() {
                true  => println!("Hand {}: {} ; ({}, busted)", i + 1, hand, format_vec_string(&hand.value())),
                false => println!("Hand {}: {} ; ({})", i + 1, hand, format_vec_string(&hand.filter_value())),
            }
        }
        input!("Moving to dealer's turn. Enter to continue...");

        spare_hands
    }

    /**
     * Handle input during play
     */
    fn handle_play_input(&mut self, hand: &mut Hand, c: char) -> (bool, bool) {
        match c {
            'h' | 'H' => {
                hand.draw_from(&mut self.deck);
                (false, false)
            },
            's' | 'S' => (true, false),
            'd' | 'D' => {
                hand.draw_from(&mut self.deck);
                hand.set_doubled(true);
                (true, true)
            },
            'l' | 'L' => (false, true),
            _ => (false, false),
        }
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
        self.run_player_turns(spare_hands, ndx)
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
        print!("[1E[2J");

        // get the dealer's hand value, get hands that beat the dealer
        let dealer_max = self.dealer.true_value();
        let winning_hands: Vec<&Hand> = 
            self.hands.iter()
            .filter(|hand| hand.true_value() > dealer_max)
            .collect();
        let winning_ndxs: Vec<usize> = 
            self.hands.iter()
            .enumerate()
            .filter(|enume| enume.1.true_value() > dealer_max)
            .map(|enume| enume.0)
            .collect();

        // display corresponding header
        if self.dealer.is_busted() {
            println!("Dealer: {} ; ({})\n", self.dealer, format_vec_string(&self.dealer.value()));
            println!("Dealer busted. All non-busted hands win:");
        } else if winning_hands.is_empty() {
            println!("Dealer: {} ; ({})\n", self.dealer, self.dealer.true_value());
            println!("Dealer scored {}, you lost on all hands.", dealer_max);
        } else {
            println!("Dealer: {} ; ({})\n", self.dealer, self.dealer.true_value());
            println!("Dealer scored {}, you won on {} hands:", dealer_max, winning_hands.len());
        }

        // take each hand that beat the dealer and convert it to its win amount
        let payouts: Vec<f32> = winning_hands.iter().map(|hand| {
            if hand.is_blackjack() { // blackjacks get 1.5x bet
                1.5
            } else if hand.is_doubled() { // doubles get 4x bet
                4.0
            } else { // standard hands get 2x bet
                2.0
            }
        }).collect();

        // print all winning hands
        for (i, hand) in self.hands.iter().enumerate() {
            match (winning_ndxs.contains(&i), hand.is_blackjack(), hand.is_doubled(), hand.is_busted()) {
                (true, true, false, false)  => println!("Hand {}: {} ; ({}) [38;5;220m[Blackjack][0m", i + 1, hand, hand.true_value()),
                (true, false, true, false)  => println!("Hand {}: {} ; ({}) [38;5;40m[Win][38;5;220m[x2][0m", i + 1, hand, hand.true_value()),
                (true, false, false, false) => println!("Hand {}: {} ; ({}) [38;5;40m[Win][0m", i + 1, hand, hand.true_value()),
                (false, false, false, true) => println!("Hand {}: {} ; ({}) [38;5;196m[Busted][0m", i + 1, hand, format_vec_string(&hand.value())),
                (_, _, _, _)                => println!("Hand {}: {} ; ({}) [38;5;196m[Lost][0m", i + 1, hand, format_vec_string(&hand.value())),
            }
        }

        // sum the payouts and return
        payouts.iter().sum()
    }
}
