mod game;
mod cards;
mod util;

use std::process::exit;
use game::gamestate::GameState;
use prompted::input;

use crate::util::input::validated_input;


fn main() {
    // get new gamestate
    let mut input;

    // attempt to load save file
    let mut gamestate = match GameState::load_state() {
        Ok(gs)  => gs,
        Err(_) => {
            input!("Failed to load from save file. generating a new one.\nEnter to continue...");
            GameState::new()
        },
    };

    // simple input loop to play or exit
    loop {
        input = match gamestate.can_start() {
            true  => {
                print!("\n[2J--Rust Blackjack--\n1. Play Game\n2. Play With Last Settings\n3. Bank History\n4. Exit\n:: ");
                validated_input(|c| '1' <= c && c <= '4', |inp| 1 <= inp && inp <= 4)
            },
            false => {
                print!("\n[2J--Rust Blackjack--\n1. Play Game\n[2m2. Play With Last Settings[0m\n3. Bank History\n4. Exit\n:: ");
                validated_input(|c| '1' <= c && c <= '4', |inp| 1 == inp || inp == 3 || inp == 4)
            },
        };

        match input {
            1 => {
                gamestate.new_settings();
                gamestate.start_game();
            },
            2 => gamestate.start_game(),
            3 => gamestate.bank.run_ui(),
            4 => exit(0),
            _ => (),
        }
    }
}
