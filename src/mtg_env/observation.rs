use crate::mtg_env::card::Card;
use crate::mtg_env::hand::Hand;

#[derive(Debug)]
pub struct Observation {
    pub raw_array: [i32; Observation::SIZE],
}

impl Observation {
    pub const SIZE: usize = 5;
    pub fn new(
        hand: Hand<Card>,
        life_points: i32,
        opponent_life_points: i32,
        has_game_started: bool
    ) -> Observation {
        let mut raw_array = [0; Self::SIZE];
        let mut i = 0;
        for (_, count) in hand.card_counts.iter() {
            raw_array[i] = *count;
            i += 1;
        }
        raw_array[Self::SIZE - 3] = life_points;
        raw_array[Self::SIZE - 2] = opponent_life_points;
        raw_array[Self::SIZE - 1] = has_game_started as i32;
        Self { raw_array }
    }
}