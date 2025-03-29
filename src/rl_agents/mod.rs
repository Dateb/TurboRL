use std::collections::HashMap;
use wandb::{BackendOptions, RunInfo, WandB};
use crate::mtg_env::Game;
use crate::mtg_env::Observation;

pub struct Agent {
    value_function: HashMap<([i32; Observation::SIZE], usize), f32>,
    discount_factor: f32,
    learning_rate: f32,
    epsilon: f64,
    num_actions: usize,
}

impl Agent {
    pub fn new(discount_factor: f32, learning_rate: f32, epsilon: f64, num_actions: usize) -> Self {
        Self {
            value_function: HashMap::new(),
            discount_factor,
            learning_rate,
            epsilon,
            num_actions,
        }
    }

    pub async fn train(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let api_key = "89f9b3ae8db7bbef2a013956b8bb49e683d5330b".to_string();
        let wandb = WandB::new(BackendOptions::new(api_key));

        let run = wandb
            .new_run(
                RunInfo::new("RL_mtg_learning")
                    .entity("mtg-sim-team")
                    .name(format!(
                        "test-{}",
                        std::time::UNIX_EPOCH.elapsed().unwrap().as_millis()
                    ))
                    .build()?,
            )
            .await?;

        let n_total_steps = 50000000;
        let mut total_wins = 0.0;
        let mut total_games = 0;

        let mut game = Game::new();
        let (mut observation, _, _) = game.reset();
        let mut current_state = observation.raw_array;

        let mut win_rate = 0.0;

        for _ in 0..n_total_steps {
            let action = self.select_action(&current_state);
            let (next_observation, reward, done) = game.step(action);

            let next_state = next_observation.raw_array;
            self.update_value_function(&current_state, action, reward, &next_state, done);

            if done {
                total_wins += reward;
                total_games += 1;

                win_rate = (1.0 - 0.0001) * win_rate + 0.0001 * reward;

                if total_games % 10000 == 0 {
                    println!("Game {} finished! Current winrate: {}", total_games, win_rate);
                    run.log((
                        ("_step", total_games),
                        ("Win Rate", win_rate),
                    ))
                        .await;
                }

                let (new_observation, _, _) = game.reset();
                current_state = new_observation.raw_array;
            } else {
                current_state = next_state;
            }
        }

        println!("Final Reward: {:?}", game.current_reward);
        Ok(())
    }

    pub fn select_action(&self, state: &[i32; Observation::SIZE]) -> usize {
        if rand::random::<f64>() < self.epsilon {
            rand::random::<usize>() % self.num_actions
        } else {
            let mut max_value = f32::NEG_INFINITY;
            let mut best_action = 0;

            for action in 0..self.num_actions {
                let state_action_value = self
                    .value_function
                    .get(&(state.clone(), action))
                    .copied()
                    .unwrap_or(0.0);

                if state_action_value > max_value {
                    max_value = state_action_value;
                    best_action = action;
                }
            }

            best_action
        }
    }

    pub fn update_value_function(
        &mut self,
        current_state: &[i32; Observation::SIZE],
        action: usize,
        reward: f32,
        next_state: &[i32; Observation::SIZE],
        done: bool,
    ) {
        let max_next_value = if done {
            0.0
        } else {
            (0..self.num_actions)
                .map(|a| self.value_function.get(&(next_state.clone(), a)).copied().unwrap_or(0.0))
                .fold(f32::NEG_INFINITY, f32::max)
        };

        let current_value = self
            .value_function
            .entry((current_state.clone(), action))
            .or_insert(0.0);

        *current_value +=
            self.learning_rate * (reward + self.discount_factor * max_next_value - *current_value);
    }
}