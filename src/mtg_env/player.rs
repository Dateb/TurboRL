use crate::mtg_env::boardstate::BoardState;
use crate::mtg_env::card::Card;
use crate::mtg_env::deck::Deck;
use crate::mtg_env::hand::Hand;
use crate::mtg_env::permanent::{Permanent, PermanentType};

#[derive(Debug)]
pub struct Player {
    pub deck: Deck,
    pub library: Deck,
    pub hand: Hand<Card>,
    pub board_state: BoardState,
    pub life_points: i32
}

impl Player {
    pub fn new(deck: Deck) -> Self {
        Self {
            deck: deck.clone(),
            library: deck,
            hand: Hand::new(),
            board_state: BoardState::new(),
            life_points: 20
        }
    }

    pub fn draw_card(&mut self) -> () {
        if let Some(card) = self.library.pop() {
            self.hand.add_card(card);
        }
    }

    pub fn draw_n_cards(&mut self, n: usize) -> () {
        for _ in 0..n {
            self.draw_card();
        }
    }

    pub fn take_mulligan(&mut self) -> () {
        let n_cards_after_mulligan = self.hand.size - 1;
        self.library = self.deck.clone();
        self.library.shuffle();
        self.hand = Hand::new();
        self.draw_n_cards(n_cards_after_mulligan);
    }

    pub fn play_card(&mut self, card_idx: usize) -> () {
        match card_idx {
            0 => {
                let card = Card::Plains;
                if self.hand.contains_card(&card) {
                    self.hand.remove_card(card);
                    self.board_state.lands.push(
                        Permanent {
                            ptype: PermanentType::Land,
                            power: 0,
                            toughness: 0,
                            is_tapped: false,
                        }
                    )
                }
            },
            1 => {
                let card = Card::SavannahLion;
                if self.hand.contains_card(&card) {
                    self.hand.remove_card(card);
                    self.board_state.creatures.push(
                        Permanent {
                            ptype: PermanentType::Creature,
                            power: 2,
                            toughness: 1,
                            is_tapped: false,
                        }
                    )
                }
            }
            _ => {}
        };
    }

    pub fn reset(&mut self) {
        self.life_points = 20;
        self.library = self.deck.clone();
        self.library.shuffle();
        self.hand = Hand::new();
        self.board_state = BoardState::new();
    }
}