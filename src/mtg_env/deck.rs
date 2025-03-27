use rand::thread_rng;
use rand::seq::SliceRandom;

use crate::mtg_env::card::Card;

#[derive(Debug, Clone)]
pub struct Deck {
    cards: Vec<Card>
}

impl Deck {
    pub fn new() -> Deck {
        Self {
            cards: [vec![Card::Plains; 50], vec![Card::SavannahLion; 10]].concat()
        }
    }

    pub fn shuffle(&mut self) -> () {
        self.cards.shuffle(&mut thread_rng())
    }

    pub fn pop(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }
}
