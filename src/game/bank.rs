use std::{cmp, fmt::Display};

use prompted::input;
use serde::{Deserialize, Serialize};

use crate::util::input::read_one_char;

/**
 * Bank / Money handler. Holds balance, bets, and a banking history
 */
#[derive(Serialize, Deserialize)]
pub struct GameBank {
    balance: usize,
    pub cur_bet: usize,
    history: BankHistory,
}

/**
 * Stores banking history data, including resets, total spent and earned, and
 * recent transactions
 */
#[derive(Serialize, Deserialize)]
pub struct BankHistory {
    resets: usize,
    total_spent: usize,
    total_earned: usize,
    hands_bought: usize,
    recent_transactions: Vec<Transaction>,
}

/**
 * A transaction. holds amount and type
 */
#[derive(Serialize, Deserialize)]
pub struct Transaction {
    pub amount: usize,
    pub typ: TransactionType,
    pub trans_bal: usize,
}

#[derive(Serialize, Deserialize)]
pub enum TransactionType {
    SPEND,
    EARN,
    RESET,
}

impl GameBank {
    /**
     * Create a new empty bank. Typically used if no save was loaded
     */
    pub fn new() -> GameBank {
        GameBank {
            balance: 1000,
            cur_bet: 0,
            history: BankHistory {
                resets: 0,
                total_spent: 0,
                total_earned: 0,
                hands_bought: 0,
                recent_transactions: vec![],
            }
        }
    }

    /**
     * Pay for `hands` hands based on the `cur_bet`
     */
    pub fn buy(&mut self, hands: usize) {
        let amount = self.cur_bet * hands;
        self.balance -= amount;
        self.history.hands_bought += hands;
        self.history.add_transaction(self.balance, amount, TransactionType::SPEND);
    }

    /**
     * Gain payout for winning `weight` payouts.
     * 1 weight corresponds to `cur_bet`.
     * Returns the amount gained
     */
    pub fn win(&mut self, weight: f32) -> usize {
        // multiply weight and bet, round to whole by type casting
        let amount = (self.cur_bet as f32 * weight) as usize;

        self.balance += amount;
        self.history.add_transaction(self.balance, amount, TransactionType::EARN);
        amount
    }

    /**
     * Reset the current balance.
     * Typically used when completely out of money
     */
    pub fn reset_balance(&mut self) {
        self.balance = 1000;
        self.history.add_transaction(self.balance, 0, TransactionType::RESET);
    }

    /**
     * Get the current balance
     */
    pub fn get_balance(&self) -> usize {
        self.balance
    }

    /**
     * Get resets from history
     */
    pub fn get_resets(&self) -> usize {
        self.history.resets
    }

    /**
     * Runs the history UI
     */
    pub fn run_ui(&self) {
        // scroll position
        let size = self.history.recent_transactions.len();
        let mut pos = 0;

        // run until exited
        loop {
            // find the index of the end of the view
            let end = cmp::min(pos + 10, size);
            
            // reverse the list of transactions, get the visible window 
            let elements: Vec<&Transaction> = self.history.recent_transactions.iter().rev().collect();
            let visible = &elements[pos..end];

            // print transactions and title
            println!("[2JTransaction History (h for help):");
            let mut ndx = pos;
            for transaction in visible {
                ndx += 1;
                println!("{ndx}: {transaction}");
            }

            // character inputs
            match read_one_char() as char {
                // quit
                'q' => break,
                // scroll down
                'j' => pos = cmp::min(pos + 1, size.checked_sub(10).unwrap_or(0)),
                // scroll up
                'k' => pos = pos.checked_sub(1).unwrap_or(0),
                // show stats
                's' => {
                    input!("[2JTotal Resets: {}\nTotal Won: {}\nTotal Spent: {}\nHands Bought: {}\n\nEnter to continue...",
                        self.get_resets(), self.history.total_earned, self.history.total_spent, self.history.hands_bought);
                },
                // help
                'h' => {
                    input!("[2JShows the (up to) 128 most recent transactions. Navigate with your keyboard:\n\th - help\n\tq - quit\n\tj - scroll down\n\tk - scroll up\n\ts - stats\n\nEnter to continue...");
                },
                // ignore unregistered inputs
                _ => {},
            }
        }
    }
}

impl BankHistory {
    /**
     * Add a transaction to recent transactions.
     * Holds the 128 most recent transactions
     */
    fn add_transaction(&mut self, balance: usize, amount: usize, typ: TransactionType) {
        // limit 128
        if self.recent_transactions.len() >= 128 {
            self.recent_transactions.remove(0);
        }

        // add amount to spent or earned
        match typ {
            TransactionType::SPEND => self.total_spent += amount,
            TransactionType::EARN => self.total_earned += amount,
            TransactionType::RESET => self.resets += 1,
        }

        // add to list
        self.recent_transactions.push(Transaction{amount, typ, trans_bal: balance});
    }
}

impl Display for Transaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", 
            match self.typ {
                TransactionType::SPEND => {
                    // [RED]-amount[/RED] -> new_bal 
                    format!("[38;5;196m-${}[0m -> ${}", self.amount, self.trans_bal)
                },
                TransactionType::EARN => {
                    // [GREEN]+amount[/GREEN] -> new_bal 
                    format!("[38;5;40m+${}[0m -> ${}", self.amount, self.trans_bal)
                },
                TransactionType::RESET => {
                    // [YELLOW]Bank Reset[/YELLOW] -> new_bal 
                    format!("[38;5;214mBank Reset[0m -> ${}", self.trans_bal)
                },
            },
        )
    }
}
