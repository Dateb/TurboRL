use crate::mtg_env::card::Card;

#[derive(Debug)]
pub struct Observation {
    pub(crate) hand_array: [i32; 7]
}

impl Observation {
    pub fn new(hand: Vec<Card>) -> Observation {
        let mut hand_array = [0; 7];
        for (i, _) in hand.iter().take(7).enumerate() {
            hand_array[i] = hand[i].clone() as i32;
        }
        Self { hand_array }
    }
}