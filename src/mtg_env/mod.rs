use std::collections::HashMap;
use wandb::{BackendOptions, RunInfo, WandB};

mod player;
mod game;
mod card;
mod deck;
mod permanent;
mod boardstate;
mod observation;

use game::Game;
use player::Player;
use deck::Deck;

pub async fn play_mtg() -> Result<(), Box<dyn std::error::Error>> {
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

    // Parameters for value iteration
    let mut value_function: HashMap<([i32; 8], usize), f32> = HashMap::new();
    let discount_factor = 0.9;
    let learning_rate = 0.1;
    let num_actions = 7;
    let n_total_steps = 50000000;
    let mut total_wins = 0.0;
    let mut total_games = 0;

    // Initialize decks and players
    let deck1 = Deck::new();
    let deck2 = Deck::new();
    let learning_player = Player::new(deck1);
    let opponent_player = Player::new(deck2);

    let mut game = Game::new(learning_player, opponent_player);
    let (mut observation, _, _) = game.reset();
    let mut current_state = observation.raw_array;

    for _ in 0..n_total_steps {
        let action = select_action(&current_state, &value_function, num_actions);
        let (next_observation, mut reward, mut done) = game.step(action);

        let next_state = next_observation.raw_array;
        // Update value function (simple Bellman equation)
        let max_next_value = (0..num_actions)
            .map(|a| value_function.get(&(next_state, a)).copied().unwrap_or(0.0))
            .fold(f32::NEG_INFINITY, f32::max);
        let current_value = value_function.entry((current_state, action)).or_insert(0.0);
        *current_value += learning_rate * (reward + discount_factor * max_next_value - *current_value);

        if done {
            total_wins += reward;
            total_games += 1;

            let win_rate = total_wins / total_games as f32;

            if total_games % 10000 == 0 {
                println!("Game {} finished! Current winrate: {}", total_games, win_rate);
                run.log((
                    ("_step", total_games),
                    ("Win Rate", win_rate),
                ))
                    .await;
            }

            (observation, reward, done) = game.reset();
            current_state = observation.raw_array;
        } else {
            current_state = next_state;
        }
    }

    println!("Final Reward: {:?}", game.current_reward);
    Ok(())
}

// Action selection function
fn select_action(state: &[i32; 8], value_function: &HashMap<([i32; 8], usize), f32>, num_actions: usize) -> usize {
    // Example: Epsilon-greedy action selection.
    let epsilon = 0.1;
    if rand::random::<f64>() < epsilon {
        rand::random::<usize>() % num_actions
    } else {
        let mut max_value = f32::NEG_INFINITY;
        let mut best_action = 0;

        for action in 0..num_actions {
            // Look up value in the value function map
            let state_action_value = value_function
                .get(&(state.clone(), action))
                .copied()
                .unwrap_or(0.0); // Default value 0.0 if no value is found

            if state_action_value > max_value {
                max_value = state_action_value;
                best_action = action;
            }
        }

        best_action
    }
}