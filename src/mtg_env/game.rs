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
    learning_player: Player,
    opponent_player: Player,
    current_turn_phase: TurnPhase,
    pub done: bool
}

impl Game {
    pub fn new(learning_player: Player, opponent_player: Player) -> Self {
        Self {
            learning_player,
            opponent_player,
            current_turn_phase: TurnPhase::Main1,
            done: false
        }
    }

    pub fn reset(&mut self) -> Vec<Card> {
        self.learning_player.reset();
        self.opponent_player.reset();

        self.learning_player.draw_starting_hand();
        self.opponent_player.draw_starting_hand();

        self.done = false;

        self.current_turn_phase = TurnPhase::Main1;
        self.learning_player.hand.clone()
    }

    pub fn step(&mut self, action: usize) -> (Vec<Card>, i32, bool) {
        self.player_turn(action);
        self.opponent_turn();
        let obs = (
            self.learning_player.hand.clone(),
            0,
            self.done,
        );

        self.print_game_state();

        obs
    }

    fn player_turn(&mut self, action: usize) -> () {
        execute_turn(action, &mut self.learning_player, &mut self.opponent_player);

        if self.opponent_player.life_points <= 0 {
            self.done = true;
        }
    }

    fn opponent_turn(&mut self) -> () {
        execute_turn(0, &mut self.opponent_player, &mut self.learning_player);

        if self.learning_player.life_points <= 0 {
            self.done = true;
        }
    }

    pub fn print_game_state(&mut self) -> () {
        println!("{}", "/".repeat(100));

        println!("{}", self.opponent_player.life_points);
        println!("{:?}", self.opponent_player.hand);
        println!("{}", "P".repeat(self.opponent_player.board_state.get_land_count()));
        println!("{}", "S".repeat(self.opponent_player.board_state.get_creature_count()));
        println!("{}", "-".repeat(100));
        println!("{}", "S".repeat(self.learning_player.board_state.get_creature_count()));
        println!("{}", "P".repeat(self.learning_player.board_state.get_land_count()));
        println!("{:?}", self.learning_player.hand);
        println!("{}", self.learning_player.life_points);
    }
}

fn execute_turn(action: usize, active_player: &mut Player, inactive_player: &mut Player) -> () {
    let mut current_turn_phase = TurnPhase::Draw;
    while current_turn_phase != TurnPhase::End {
        match current_turn_phase {
            TurnPhase::Draw => {
                active_player.draw_card();
                current_turn_phase = TurnPhase::Main1;
            },
            TurnPhase::Main1 => {
                active_player.play_card(action);
                current_turn_phase = TurnPhase::Combat;
            },
            TurnPhase::Combat => {
                inactive_player.life_points -=
                    (active_player.board_state.get_creature_count() * 2) as i16;

                current_turn_phase = TurnPhase::End;
            }
            TurnPhase::End => {}
        }
    }
}