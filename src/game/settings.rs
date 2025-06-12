use serde::{Deserialize, Serialize};

/**
 * Simple game settings, tracking deck and hand counts
 */
#[derive(Serialize, Deserialize)]
pub struct GameSettings {
    pub deck_count: usize,
    pub hand_count: usize,
}
