mod game;
mod cards;
mod util;

use std::process::exit;
use game::gamestate::GameState;
use prompted::input;


/**
 * Print main menu
 */
fn print_menu() {
    println!("[2J--Rust Blackjack--\n1. Play Game\n2. Exit");
}

fn main() {
    // get new gamestate
    let mut input;
    let mut gamestate = GameState::new();

    // attempt to load save file
    match gamestate.load_state() {
        Ok(_)  => (),
        Err(_) => {
            input!("Failed to load from save file. generating a new one.\nEnter to continue...");
            // dont actually need to do anything here
        },
    }
    
    // simple input loop to play or exit
    loop {
        print_menu();
        input = input!(":: ");

        match input {
            _ if input == "1" => gamestate.start_game(),
            _ if input == "2" => {
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
