use crate::mtg_env::boardstate::BoardState;
use crate::mtg_env::card::Card;
use crate::mtg_env::deck::Deck;
use crate::mtg_env::permanent::{Permanent, PermanentType};

#[derive(Debug)]
pub struct Player {
    pub deck: Deck,
    pub library: Deck,
    pub hand: Vec<Card>,
    pub board_state: BoardState,
    pub life_points: i32
}

impl Player {
    pub fn new(deck: Deck) -> Self {
        Self {
            deck: deck.clone(),
            library: deck,
            hand: Vec::new(),
            board_state: BoardState::new(),
            life_points: 20
        }
    }

    pub fn draw_card(&mut self) -> () {
        if let Some(card) = self.library.pop() {
            self.hand.push(card);
        }
    }

    pub fn draw_starting_hand(&mut self) -> () {
        for _ in 0..7 {
            self.draw_card();
        }
    }

    pub fn play_card(&mut self, index: usize) -> () {
        if let Some(card) = self.hand.get(index) {
            match card {
                Card::Plains => self.board_state.lands.push(
                    Permanent {
                        ptype: PermanentType::Land,
                        power: 0,
                        toughness: 0,
                        is_tapped: false,
                    }
                ),
                Card::SavannahLion => self.board_state.creatures.push(
                    Permanent {
                        ptype: PermanentType::Creature,
                        power: 2,
                        toughness: 1,
                        is_tapped: false,
                    }
                ),
            };
        }

        if index < self.hand.len() {
            Some(self.hand.remove(index));
        }
    }

    pub fn reset(&mut self) {
        self.life_points = 20;
        self.library = self.deck.clone();
        self.library.shuffle();
        self.hand.clear();
        self.board_state = BoardState::new();
    }
}