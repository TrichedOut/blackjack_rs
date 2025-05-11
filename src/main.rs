mod game;
use game::new_game;

mod cards;
mod util;

use std::process::exit;
use prompted::input;


fn print_menu() {
    println!("[2J--Rust Blackjack--\n1. Play Game\n2. Exit");
}

fn get_settings() {
    print!("[2J");

    let mut deck_count = 0;
    while deck_count == 0 || deck_count > 16 {
        let decks = input!("How many decks (1-16)?\n:: ");
        deck_count = decks.parse().unwrap_or(0);
    }

    let mut hand_count = 0;
    while hand_count == 0 || hand_count > 7 {
        let hands = input!("How many hands (1-7)?\n:: ");
        hand_count = hands.parse().unwrap_or(0);
    }

    new_game(deck_count, hand_count);
}

fn main() {
    let mut input;
    
    loop {
        print_menu();
        input = input!(":: ");

        match input {
            _ if input == "1" => get_settings(),
            _ if input == "2" => exit(0),
            _ => (),
        }
    }
}
