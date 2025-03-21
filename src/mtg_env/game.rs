use crate::mtg_env::card::Card;
use crate::mtg_env::player::Player;

pub struct Game {
    player1: Player,
    player2: Player,
    is_player1_active: bool,
    pub done: bool
}

impl Game {
    pub fn new(player1: Player, player2: Player) -> Self {
        Self {
            player1,
            player2,
            is_player1_active: true,
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

        self.player1.hand.clone()
    }

    pub fn step(&mut self) -> (Vec<Card>, i32, bool) {
        if self.player1.library.is_empty() && self.player2.library.is_empty() {
            self.done = true;
            println!("Game Over: Both players are out of cards.");
        }

        let active_player = if self.is_player1_active {
            &mut self.player1
        } else {
            &mut self.player2
        };
        active_player.take_turn();

        self.is_player1_active = !self.is_player1_active;

        (
            active_player.hand.clone(),
            0,
            self.done,
        )
    }
}