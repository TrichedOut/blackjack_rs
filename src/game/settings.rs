/**
 * Simple game settings, tracking deck and hand counts
 */
pub struct GameSettings {
    pub deck_count: usize,
    pub hand_count: usize,
}

/**
 * Bank / Money handler. Holds balance, bets, and bank resets
 */
pub struct GameBank {
    pub balance: usize,
    pub cur_bet: usize,
    pub resets:  usize,
}

impl GameBank {
    /**
     * Pay for `hands` hands based on the `cur_bet`
     */
    pub fn buy(&mut self, hands: usize) {
        self.balance -= self.cur_bet * hands;
    }

    /**
     * Gain payout for winning `weight` payouts.
     * 1 weight corresponds to `cur_bet`.
     * Returns the amount gained
     */
    pub fn win(&mut self, weight: f32) -> usize {
        // multiply weight and bet, round to whole by type casting
        let amount = (self.cur_bet as f32 * weight) as usize;

        // add amount to balance and return
        self.balance += amount;
        amount
    }
}
