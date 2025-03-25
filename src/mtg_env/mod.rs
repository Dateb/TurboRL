use std::collections::HashMap;

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

pub fn play_mtg() {
    // Parameters for value iteration
    let mut value_function: HashMap<([i32; 7], usize), f32> = HashMap::new();
    let discount_factor = 0.9;
    let learning_rate = 0.1;
    let num_actions = 7;
    let n_total_steps = 10000;
    let mut total_wins = 0.0;
    let mut total_games = 0;

    // Initialize decks and players
    let deck1 = Deck::new();
    let deck2 = Deck::new();
    let learning_player = Player::new(deck1);
    let opponent_player = Player::new(deck2);

    let mut game = Game::new(learning_player, opponent_player);
    let (mut observation, _, _) = game.reset();
    let mut current_state = observation.hand_array;

    for _ in 0..n_total_steps {
        let action = select_action(&current_state, &value_function, num_actions);
        let (next_observation, mut reward, mut done) = game.step(action);

        let next_state = next_observation.hand_array;
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
            println!("Game {} finished! Current winrate: {}", total_games, win_rate);

            (observation, reward, done) = game.reset();
            current_state = observation.hand_array;
        } else {
            current_state = next_state;
        }
    }

    println!("Final Reward: {:?}", game.current_reward);
}

// Action selection function
fn select_action(state: &[i32; 7], value_function: &HashMap<([i32; 7], usize), f32>, num_actions: usize) -> usize {
    // Example: Epsilon-greedy action selection.
    let epsilon = 0.1;
    if rand::random::<f64>() < epsilon {
        rand::random::<usize>() % num_actions
    } else {
        0
    }
}