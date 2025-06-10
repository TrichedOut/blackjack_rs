use prompted::input;

use super::{game::Game, settings::{GameBank, GameSettings}};

// settings and state for the game
pub struct GameState {
    settings: Option<GameSettings>,
    bank: GameBank,
}

impl GameState {
    /**
     * Create a new default gamestate with no settings, no resets, and 1000 bal
     */
    pub fn new() -> GameState {
        return GameState {
            settings: None,
            bank: GameBank {
                balance: 1000,
                resets: 0,
                cur_bet: 0,
            },
        }
    }

    /**
     * Reset balance back to 1000.
     * Increases reset tracker by 1.
     * Typically used when player can afford nothing
     */
    pub fn reset_balance(&mut self) {
        self.bank.resets += 1;
        self.bank.balance = 1000;
    }

    /**
     * Start setup to begin a new game.
     * Used if no games currently exist, typically coming from the main menu
     */
    pub fn start_game(&mut self) {
        // ensure the balance is >=50, get new settings
        self.ensure_balance();
        self.new_settings();

        // if settings were made, store for later, otherwise if backed out, ret
        let settings;
        match &self.settings {
            Some(s) => {
                settings = s;
            },
            None    => return,
        }

        // make a new game from the settings
        let mut game = Game::new(&settings);

        // play the game
        self.play_game(&mut game);

        // loop until player exits
        loop {
            // if there are no settings here, fail hard. this should not be 
            // possible
            let settings = match &self.settings {
                Some(s) => s,
                None    => panic!("No settings present in GameState::start_game."),
            };

            // calculate some display numbers
            let again_cost = self.bank.cur_bet * settings.hand_count;
            let again_bal = self.bank.balance as i32 - again_cost as i32;
            let can_again = again_bal >= 0;
            let again;

            // display the play again menu based on balance. get input
            if can_again {
                again = input!(
                    "[2JYou now have ${}\nIt costs ${} to play {} more hands\nYou will be left with ${}\n\n1. Play Again\n2. Change Settings\n3. Main Menu\n:: ",
                    self.bank.balance, again_cost, settings.hand_count, again_bal
                );
            } else {
                again = input!(
                    "[2JYou now have ${}\nIt costs ${} to play {} more hands.\nYou do not have enough to play again, please change settings or incur a balance reset.\n\n1. Reset Balance\n2. Change Settings\n3. Main Menu\n:: ",
                    self.bank.balance, again_cost, settings.hand_count
                );
            }

            match again {
                // play again
                _ if again == "1" => {
                    // play game or reset balance; reflected from display
                    if can_again {
                        self.play_game(&mut game);
                    } else {
                        self.reset_balance();
                        input!(
                            "[2JBalance reset to ${}. You now have {} resets.\n\nEnter to continue...",
                            self.bank.balance, self.bank.resets
                        );
                    }
                },
                // new settings
                _ if again == "2" => {
                    // get new settings
                    self.new_settings();

                    // save or back out if backed out
                    match &self.settings {
                        Some(s) => {
                            game.update_settings(&s);
                            self.play_game(&mut game);
                        },
                        None    => ()
                    }
                } 
                // back to main menu
                _ if again == "3" => return,
                // reloop
                _ => (),
            }
        }
    }
    
    /**
     * Play a game loop
     */
    fn play_game(&mut self, game: &mut Game) {
        // if there are no settings panic, shouldn't be possible
        let settings = match &self.settings {
            Some(s) => s,
            None    => panic!("No settings present in GameState::play_game."),
        };

        // take balance away for purchased hands
        self.bank.buy(settings.hand_count);

        // calc amount of spare hands, play the game, and pay for bought hands
        let spare_hands = self.bank.balance / self.bank.cur_bet;
        let (wins, bought_hands) = game.play(spare_hands);
        self.bank.buy(bought_hands);

        // print winnings
        input!("\nYou won back ${}\nYou now have ${}\n\nEnter to continue...", self.bank.win(wins), self.bank.balance);
    }

    /**
     * Create settings to use for a game.
     * Is set to `None` if backed out at the end
     */
    fn new_settings(&mut self) {
        let mut deck_count = 0;
        let mut hand_count = 0;
        let mut bet_amount = 0;

        // get the deck count. 1 <= x <= 16
        while deck_count == 0 
            || deck_count > 16 {

            deck_count = input!("[2JYou have ${}.\n\nDecks to use (1-16):\nHands to play (1-7):\nAmount to bet (>=$50): $\n[3A[21C", self.bank.balance).parse().unwrap_or(0);
        }

        // get the hand count. checks for valid amount based on balance.
        // 1 <= x <= 7
        while hand_count == 0 
            || hand_count > 7
            || hand_count * 50 > self.bank.balance {

            hand_count = input!("Hands to play (1-7):\nAmount to bet (>=$50): $\n[2F[21C[0K").parse().unwrap_or(0);
            print!("[1A");
        }

        // get bet. min val 50
        print!("[1B");
        while bet_amount < 50 
            || bet_amount * hand_count > self.bank.balance {

            bet_amount = input!("Amount to bet (>=$50): $\n[1F[24C[0K").parse().unwrap_or(0);
            print!("[1A")
        }

        // confirm settings. in loop in case of invalid input
        loop {
            // display confirmation info
            let play_cost = hand_count * bet_amount;
            let input = input!("[2JBalance remaining after start: ${}\nPlaying with:\n{} decks,\n{} hands at ${} each (${}).\n\n1. Confirm\n2. Cancel\n:: ",
            self.bank.balance - play_cost, deck_count, hand_count, bet_amount, play_cost);

            match input {
                // confirm; create settings
                _ if input == "1" => {
                    self.settings = Some(GameSettings {
                        deck_count,
                        hand_count,
                    });
                    self.bank.cur_bet = bet_amount;
                    return;
                },
                // cancel; set to None
                _ if input == "2" => {
                    self.settings = None;
                    return;
                },
                // go again
                _ => (),
            }
        }
    }

    /**
     * Ensure balance is >=50. 
     * If balance is <50, reset balance and let player know
     */
    fn ensure_balance(&mut self) {
        if self.bank.balance < 50 {
            self.reset_balance();
            input!("[2JYou ran out of money... You've now reset {} times.\n\nEnter to continue...", self.bank.resets);
        }
    }
}
