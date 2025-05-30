use prompted::input;

use super::{game::Game, settings::{GameBank, GameSettings}};

pub struct GameState {
    settings: Option<GameSettings>,
    bank: GameBank,
}

impl GameState {
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

    pub fn reset_balance(&mut self) {
        self.bank.resets += 1;
        self.bank.balance = 1000;
    }

    pub fn start_game(&mut self) {
        self.ensure_balance();
        self.new_settings();

        let settings;
        match &self.settings {
            Some(s) => {
                settings = s;
            },
            None    => return,
        }

        let mut game = Game::new(&settings);

        self.play_game(&mut game);

        loop {
            let settings = match &self.settings {
                Some(s) => s,
                None    => panic!("No settings present in GameState::start_game."),
            };

            let again_cost = self.bank.cur_bet * settings.hand_count;
            let again_bal = self.bank.balance as i32 - again_cost as i32;
            let can_again = again_bal >= 0;
            let again;

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
                _ if again == "1" => {
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
                _ if again == "2" => {
                    self.new_settings();

                    match &self.settings {
                        Some(s) => {
                            game.update_settings(&s);
                            self.play_game(&mut game);
                        },
                        None    => ()
                    }
                } 
                _ if again == "3" => return,
                _ => (),
            }
        }
    }
    
    fn play_game(&mut self, game: &mut Game) {
        let settings = match &self.settings {
            Some(s) => s,
            None    => panic!("No settings present in GameState::play_game."),
        };

        self.bank.buy(settings.hand_count);

        let spare_hands = self.bank.balance / self.bank.cur_bet;
        let (wins, bought_hands) = game.play(spare_hands);
        self.bank.buy(bought_hands);

        input!("\nYou won back ${}\nYou now have ${}\n\nEnter to continue...", self.bank.win(wins), self.bank.balance);
    }

    fn new_settings(&mut self) {
        let mut deck_count = 0;
        let mut hand_count = 0;
        let mut bet_amount = 0;

        while deck_count == 0 
            || deck_count > 16 {

            deck_count = input!("[2JYou have ${}.\n\nDecks to use (1-16):\nHands to play (1-7):\nAmount to bet (>=$50): $\n[3A[21C", self.bank.balance).parse().unwrap_or(0);
        }

        while hand_count == 0 
            || hand_count > 7
            || hand_count * 50 > self.bank.balance {

            hand_count = input!("Hands to play (1-7):\nAmount to bet (>=$50): $\n[2F[21C[0K").parse().unwrap_or(0);
            print!("[1A");
        }

        print!("[1B");
        while bet_amount < 50 
            || bet_amount * hand_count > self.bank.balance {

            bet_amount = input!("Amount to bet (>=$50): $\n[1F[24C[0K").parse().unwrap_or(0);
            print!("[1A")
        }

        loop {
            let play_cost = hand_count * bet_amount;
            let input = input!("[2JBalance remaining after start: ${}\nPlaying with:\n{} decks,\n{} hands at ${} each (${}).\n\n1. Confirm\n2. Cancel\n:: ",
            self.bank.balance - play_cost, deck_count, hand_count, bet_amount, play_cost);

            match input {
                _ if input == "1" => {
                    self.settings = Some(GameSettings {
                        deck_count,
                        hand_count,
                    });
                    self.bank.cur_bet = bet_amount;
                    return;
                },
                _ if input == "2" => {
                    self.settings = None;
                    return;
                },
                _ => (),
            }
        }
    }

    fn ensure_balance(&mut self) {
        if self.bank.balance < 50 {
            self.reset_balance();
            input!("[2JYou ran out of money... You've now reset {} times.\n\nEnter to continue...", self.bank.resets);
        }
    }
}
