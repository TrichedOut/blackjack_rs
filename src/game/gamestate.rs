use std::{cmp, fs::File, io::{Read, Write}};
use serde::{Serialize, Deserialize};
use prompted::input;

use crate::util::input::{read_one_char, validated_input};

use super::{game::Game, settings::GameSettings, bank::GameBank};

// settings and state for the game
#[derive(Serialize, Deserialize)]
pub struct GameState {
    settings: Option<GameSettings>,
    pub bank: GameBank,
}

impl GameState {
    /**
     * Create a new default gamestate with no settings, 1000 bal, and
     * no history
     */
    pub fn new() -> GameState {
        return GameState {
            settings: None,
            bank: GameBank::new(),
        }
    }

    pub fn can_start(&self) -> bool {
        match &self.settings {
            Some(s) => {
                s.hand_count * self.bank.cur_bet <= self.bank.get_balance()
            },
            None => false,
        }
    }

    /**
     * Start setup to begin a new game.
     * Used if no games currently exist, typically coming from the main menu
     */
    pub fn start_game(&mut self) {
        // ensure the game can start
        if !self.can_start() {
            return;
        }
        let settings = match &self.settings {
            Some(s) => s,
            None    => return,
        };

        // make a new game from the settings
        let mut game = Game::new(settings);

        // play the game
        self.play_game(&mut game);

        // loop until player exits
        loop {
            // if there are no settings here, fail hard. this should not be 
            // possible. 
            // dont ask why this is needed. I dont know. something about
            // immutable borrowing
            let settings = match &self.settings {
                Some(s) => s,
                None    => panic!("No settings present in GameState::start_game."),
            };

            // calculate some display numbers
            let again_cost = self.bank.cur_bet * settings.hand_count;
            let again_bal = self.bank.get_balance() as i32 - again_cost as i32;
            let can_again = again_bal >= 0;

            // display the play again menu based on balance. get input
            if can_again {
                print!(
                    "[2JYou now have ${}\nIt costs ${} to play {} more hands\nYou will be left with ${}\n\n1. Play Again\n2. Change Settings\n3. Main Menu\n:: ",
                    self.bank.get_balance(), again_cost, settings.hand_count, again_bal
                );
            } else {
                print!(
                    "[2JYou now have ${}\nIt costs ${} to play {} more hands.\nYou do not have enough to play again, please change settings or incur a balance reset.\n\n1. Reset Balance\n2. Change Settings\n3. Main Menu\n:: ",
                    self.bank.get_balance(), again_cost, settings.hand_count
                );
            };
            let again = validated_input(|c| '1' <= c && c <= '3', |inp| 1 <= inp && inp <= 3);

            match again {
                // play again
                1 => {
                    // play game or reset balance; reflected from display
                    if can_again {
                        self.play_game(&mut game);
                    } else {
                        self.bank.reset_balance();
                        input!(
                            "[2JBalance reset to ${}. You now have {} resets.\n\nEnter to continue...",
                            self.bank.get_balance(), self.bank.get_resets()
                        );
                    }
                },
                // new settings
                2 => {
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
                3 => return,
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

        // calc amount of spare hands, play the game
        let spare_hands = self.bank.get_balance() / self.bank.cur_bet;
        let (wins, bought_hands) = game.play(spare_hands);
        
        // pay for bought hands
        if bought_hands > 0 {
            self.bank.buy(bought_hands);
        }

        // print winnings and update bank
        if wins == 0 as f32 {
            input!("\nYou didn't win anything...\nYou now have ${}\n\nEnter to continue...", self.bank.get_balance());
        } else {
            input!("\nYou won back ${}\nYou now have ${}\n\nEnter to continue...", self.bank.win(wins), self.bank.get_balance());
        }

        _ = self.save_state();
    }

    /**
     * Create settings to use for a game.
     * Is set to `None` if backed out at the end
     */
    pub fn new_settings(&mut self) {
        // if there is not enough money to buy at least 1 hand, attempt a reset
        if self.bank.get_balance() < 50 {
            print!("\n[2JYou do not have enough money to buy any hands. Reset your balance? [y/n]\n:: ");
            loop {
                match read_one_char() {
                    'y' => {
                        self.bank.reset_balance();
                        break;
                    },
                    'n' => return,
                     _  => return, // shouldnt be here
                }
            }
        }

        // calc max hands based on balance, or 7
        let max_hands = cmp::min(self.bank.get_balance() / 50, 7);

        // get the deck count. 1 <= x <= 16
        print!("\n[2JYou have ${}.\n\nDecks to use (1-16):\nHands to play (1-{}):\nAmount to bet (>=$50): $\n[3A[21C", self.bank.get_balance(), max_hands);
        let deck_count = validated_input(|c| c >= '0' && c <= '9', |deck| deck != 0 && deck <= 16);

        // get the hand count. checks for valid amount based on balance.
        // 1 <= x <= 7
        print!("\nHands to play (1-{}):\nAmount to bet (>=$50): $\n[2F[21C[0K", max_hands);
        let hand_count = validated_input(|c| c >= '0' && c <= '9', |hand| hand != 0 && hand <= max_hands);

        // get bet. min val 50
        print!("\n[1A[1BAmount to bet (>=$50): $\n[1F[24C[0K");
        let bet_amount = validated_input(|c| c >= '0' && c <= '9', |bet| bet >= 50 && bet * hand_count <= self.bank.get_balance());
        println!("[1A");

        // confirm settings. in loop in case of invalid input
        loop {
            // display confirmation info
            let play_cost = hand_count * bet_amount;
            print!("[2JBalance remaining after start: ${}\nPlaying with:\n{} decks,\n{} hands at ${} each (${}).\n\n1. Confirm\n2. Cancel\n:: ",
                self.bank.get_balance() - play_cost, deck_count, hand_count, bet_amount, play_cost);
            let input = validated_input(|c| c == '1' || c == '2', |inp| inp == 1 || inp == 2);

            match input {
                // confirm; create settings
                1 => {
                    self.settings = Some(GameSettings {
                        deck_count,
                        hand_count,
                    });
                    self.bank.cur_bet = bet_amount;
                    return;
                },
                // cancel; set to None
                2 => {
                    self.settings = None;
                    return;
                },
                // go again
                _ => (),
            }
        }
    }

    /**
     * Save the current gamestate to a save file
     */
    pub fn save_state(&self) -> Result<(), &'static str> {
        // create / clear the save file
        let mut file = match File::create("save.bjrs") {
            Ok(f) => f,
            _     => return Err("Could not open save file, did not save."),
        };

        // convert self into json
        let json = match serde_json::to_string(&self) {
            Ok(js) => js,
            Err(_) => return Err("Could not convert to json")
        };

        // write the json to save file
        match file.write(json.as_bytes()) {
            Ok(_) =>  Ok(()),
            Err(_) => Err("Could not write to file"),
        }
    }

    /**
     * Load a new state from the save file
     */
    pub fn load_state() -> Result<GameState, &'static str> {
        // open the save file
        let mut file = match File::open("save.bjrs") {
            Ok(f)  => f,
            Err(_) => return Err("Could not open save file"),
        };

        // read entirety of save file
        // I would allocate an array of exact size, but that is quite literally
        // not possible in rust so I'm stuck with this.
        let mut buf: Vec<u8> = vec![];
        match file.read_to_end(&mut buf) {
            Ok(_)  => (),
            Err(_) => return Err("Could not read from save file"),
        }

        // parse the buffer into a GameState
        match serde_json::from_slice(&buf) {
            Ok(gs) => Ok(gs),
            Err(_) => return Err("Could not deserialize the save file")
        }
    }
}
