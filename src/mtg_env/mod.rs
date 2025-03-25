mod player;
mod game;
mod card;
mod deck;
mod permanent;
mod boardstate;

use game::Game;
use player::Player;
use deck::Deck;

pub fn play_mtg() -> () {
    let deck1 = Deck::new();
    let deck2 = Deck::new();

    let player1 = Player::new(deck1);
    let player2 = Player::new(deck2);

    let mut game = Game::new(player1, player2);

    game.reset();

    while !game.done {
        let action = 0;
        game.step(action);
    }
}