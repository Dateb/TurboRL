use crate::mtg_env::boardstate::BoardState;
use crate::mtg_env::card::Card;
use crate::mtg_env::player::Player;

#[derive(Debug, PartialEq)]
pub enum TurnPhase {
    Draw,
    Main1,
    Combat,
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

    pub fn step(&mut self, action: usize) -> ((Vec<Card>, BoardState), i32, bool) {
        if self.player1.library.is_empty() || self.player2.library.is_empty() {
            self.done = true;
            println!("Game Over: One player is out of cards.");
        }

        let (active_player, inactive_player) = if self.is_player1_active {
            (&mut self.player1, &mut self.player2)
        } else {
            (&mut self.player2, &mut self.player1)
        };

        while self.current_turn_phase != TurnPhase::End {
            match self.current_turn_phase {
                TurnPhase::Draw => {
                    active_player.draw_card();
                    self.current_turn_phase = TurnPhase::Main1;
                },
                TurnPhase::Main1 => {
                    active_player.play_card(action);
                    self.current_turn_phase = TurnPhase::Combat;
                },
                TurnPhase::Combat => {
                    inactive_player.life_points -=
                        (active_player.board_state.get_creature_count() * 2) as i16;

                    if inactive_player.life_points <= 0 {
                        self.done = true;
                    }
                    self.current_turn_phase = TurnPhase::End;
                }
                TurnPhase::End => {}
            }
        }

        self.current_turn_phase = TurnPhase::Draw;

        self.is_player1_active = !self.is_player1_active;

        let obs = (
            (active_player.hand.clone(), active_player.board_state.clone()),
            0,
            self.done,
        );

        self.print_game_state();

        obs
    }

    pub fn print_game_state(&mut self) -> () {
        println!("{}", "/".repeat(100));

        println!("{}", self.player2.life_points);
        println!("{:?}", self.player2.hand);
        println!("{}", "P".repeat(self.player2.board_state.get_land_count()));
        println!("{}", "S".repeat(self.player2.board_state.get_creature_count()));
        println!("{}", "-".repeat(100));
        println!("{}", "S".repeat(self.player1.board_state.get_creature_count()));
        println!("{}", "P".repeat(self.player1.board_state.get_land_count()));
        println!("{:?}", self.player1.hand);
        println!("{}", self.player1.life_points);
    }
}