mod game;
mod cards;
mod util;

use std::process::exit;
use game::gamestate::GameState;
use prompted::input;


fn main() {
    // get new gamestate
    let mut input;
    let mut gamestate;

    // attempt to load save file
    match GameState::load_state() {
        Ok(gs)  => gamestate = gs,
        Err(_) => {
            input!("Failed to load from save file. generating a new one.\nEnter to continue...");
            gamestate = GameState::new();
        },
    }
    
    // simple input loop to play or exit
    loop {
        match gamestate.has_settings() {
            true  => {
                println!("[2J--Rust Blackjack--\n1. Play Game\n2. Play With Last Settings\n3. Bank History\n4. Exit");
            }
            false => {
                println!("[2J--Rust Blackjack--\n1. Play Game\n[2m2. Play With Last Settings[0m\n3. Bank History\n4. Exit");
            }
        }
        input = input!(":: ");

        match input {
            _ if input == "1" => {
                gamestate.new_settings();
                gamestate.start_game();
            },
            _ if input == "2" => gamestate.start_game(),
            _ if input == "3" => {
                gamestate.bank.run_ui();
            },
            _ if input == "4" => {
                match gamestate.save_state() {
                    Ok(_) => (),
                    Err(e) => println!("{e}"),
                }
                exit(0);
            }
            _ => (),
        }
    }
}
