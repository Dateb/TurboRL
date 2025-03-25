use crate::mtg_env::card::Card;

pub struct Observation {
    hand: Vec<Card>
}

impl Observation {
    pub fn new(hand: Vec<Card>) -> Observation {
        Self { hand }
    }
}