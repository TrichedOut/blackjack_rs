pub struct GameSettings {
    pub deck_count: usize,
    pub hand_count: usize,
}

pub struct GameBank {
    pub balance: usize,
    pub cur_bet: usize,
    pub resets:  usize,
}

impl GameBank {
    pub fn buy(&mut self, hands: usize) {
        self.balance -= self.cur_bet * hands;
    }

    pub fn win(&mut self, weight: f32) -> usize {
        let amount = (self.cur_bet as f32 * weight) as usize;
        self.balance += amount;
        amount
    }
}
