use rand::Rng;
use crate::mtg_env::player::Player;
use crate::mtg_env::observation::Observation;

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
    pub done: bool,
    pub current_reward: f32,
    is_learning_player_first: bool,
    has_game_started: bool,
}

impl Game {
    pub fn new(learning_player: Player, opponent_player: Player) -> Self {
        Self {
            learning_player,
            opponent_player,
            current_turn_phase: TurnPhase::Main1,
            done: false,
            current_reward: 0.0,
            is_learning_player_first: false,
            has_game_started: false,
        }
    }

    pub fn reset(&mut self) -> (Observation, f32, bool) {
        self.learning_player.reset();
        self.opponent_player.reset();

        let mut rng = rand::thread_rng();
        self.is_learning_player_first = rng.gen_bool(0.5);
        self.has_game_started = false;

        self.learning_player.draw_n_cards(7);
        self.opponent_player.draw_n_cards(7);

        self.done = false;
        self.current_reward = 0.0;

        self.current_turn_phase = TurnPhase::Main1;
        (
            Observation::new(
                self.learning_player.hand.clone(),
                self.learning_player.life_points,
                self.opponent_player.life_points,
                self.has_game_started,
            ),
            0.0,
            false
        )
    }

    pub fn step(&mut self, action: usize) -> (Observation, f32, bool) {
        if self.has_game_started {
            self.play_turn(action);
        } else {
            if action == 0 && self.learning_player.hand.size > 0 {
                self.learning_player.take_mulligan()
            } else {
                self.has_game_started = true;
            }
        }

        (
            Observation::new(
                self.learning_player.hand.clone(),
                self.learning_player.life_points,
                self.opponent_player.life_points,
                self.has_game_started
            ),
            self.current_reward,
            self.done
        )
    }

    fn play_turn(&mut self, action: usize) {
        if self.is_learning_player_first {
            self.player_turn(action);
            if self.opponent_player.life_points <= 0 {
                self.done = true;
                self.current_reward = 1.0;
            } else {
                self.opponent_turn();
                if self.learning_player.life_points <= 0 {
                    self.done = true;
                    self.current_reward = 0.0;
                }
            }
        } else {
            self.opponent_turn();
            if self.learning_player.life_points <= 0 {
                self.done = true;
                self.current_reward = 0.0;
            } else {
                self.player_turn(action);
                if self.opponent_player.life_points <= 0 {
                    self.done = true;
                    self.current_reward = 1.0;
                }
            }
        }
    }

    fn player_turn(&mut self, action: usize) -> () {
        execute_turn(action, &mut self.learning_player, &mut self.opponent_player);
    }

    fn opponent_turn(&mut self) -> () {
        let n_actions = 2;
        let mut rng = rand::thread_rng();
        let random_action = rng.gen_range(0..n_actions);
        execute_turn(random_action, &mut self.opponent_player, &mut self.learning_player);
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
                    (active_player.board_state.get_creature_count() * 2) as i32;

                current_turn_phase = TurnPhase::End;
            }
            TurnPhase::End => {}
        }
    }
}