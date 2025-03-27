use crate::mtg_env::card::Card;

#[derive(Debug)]
pub struct Observation {
    pub raw_array: [i32; 8],
}

impl Observation {
    pub fn new(hand: Vec<Card>, life_points: i32) -> Observation {
        let mut raw_array = [0; 8];
        for (i, _) in hand.iter().take(7).enumerate() {
            raw_array[i] = hand[i].clone() as i32;
        }
        raw_array[7] = life_points;
        Self { raw_array }
    }
}