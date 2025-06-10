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
    
    // simple input loop to play or exit
    loop {
        print_menu();
        input = input!(":: ");

        match input {
            _ if input == "1" => gamestate.start_game(),
            _ if input == "2" => exit(0),
            _ => (),
        }
    }
}
