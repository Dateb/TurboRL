use crate::mtg_env::card::Card;
use crate::mtg_env::player::Player;

#[derive(Debug, PartialEq)]
pub enum TurnPhase {
    Draw,
    Main1,
    End
}

pub struct Game {
    player1: Player,
    player2: Player,
    is_player1_active: bool,
    current_turn_phase: TurnPhase,
    pub done: bool
}

impl Game {
    pub fn new(player1: Player, player2: Player) -> Self {
        Self {
            player1,
            player2,
            is_player1_active: true,
            current_turn_phase: TurnPhase::Main1,
            done: false
        }
    }

    pub fn reset(&mut self) -> Vec<Card> {
        self.player1.reset();
        self.player2.reset();

        self.player1.draw_starting_hand();
        self.player2.draw_starting_hand();

        self.is_player1_active = true;
        self.done = false;

        self.current_turn_phase = TurnPhase::Main1;
        self.player1.hand.clone()
    }

    pub fn step(&mut self) -> (Vec<Card>, i32, bool) {
        if self.player1.library.is_empty() || self.player2.library.is_empty() {
            self.done = true;
            println!("Game Over: One player is out of cards.");
        }

        let active_player = if self.is_player1_active {
            &mut self.player1
        } else {
            &mut self.player2
        };

        while self.current_turn_phase != TurnPhase::End {
            match self.current_turn_phase {
                TurnPhase::Draw => {
                    active_player.draw_card();
                    self.current_turn_phase = TurnPhase::Main1
                },
                TurnPhase::Main1 => self.current_turn_phase = TurnPhase::End,
                TurnPhase::End => {}
            }
        }

        self.current_turn_phase = TurnPhase::Draw;

        self.is_player1_active = !self.is_player1_active;

        (
            active_player.hand.clone(),
            0,
            self.done,
        )
    }
}