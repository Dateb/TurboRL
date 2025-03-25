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

pub fn play_mtg() -> () {
    let deck1 = Deck::new();
    let deck2 = Deck::new();

    let learning_player = Player::new(deck1);
    let opponent_player = Player::new(deck2);

    let mut game = Game::new(learning_player, opponent_player);

    game.reset();

    while !game.done {
        let action = 0;
        game.step(action);
    }
}