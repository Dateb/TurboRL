use crate::mtg_env::card::Card;
use crate::mtg_env::deck::Deck;

#[derive(Debug)]
pub struct Player {
    pub deck: Deck,
    pub library: Deck,
    pub hand: Vec<Card>,
}

impl Player {
    pub fn new(deck: Deck) -> Self {
        Self {
            deck: deck.clone(),
            library: deck,
            hand: Vec::new(),
        }
    }

    pub fn draw_card(&mut self) -> Option<Card> {
        self.library.pop()
    }

    pub fn draw_starting_hand(&mut self) -> () {
        for _ in 0..7 {
            if let Some(card) = self.draw_card() {
                self.hand.push(card);
            }
        }
    }

    pub fn take_turn(&mut self) {
        if let Some(card) = self.draw_card() {
            self.hand.push(card);
        } else {
            println!("No cards left to draw!");
        }
    }

    pub fn play_card(&mut self, index: usize) -> Option<Card> {
        if index < self.hand.len() {
            Some(self.hand.remove(index))
        } else {
            None
        }
    }

    pub fn reset(&mut self) {
        self.library = self.deck.clone();
        self.library.shuffle();
        self.hand.clear();
    }
}