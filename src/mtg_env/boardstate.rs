#[derive(Debug, Clone)]
pub enum PermanentType {
    Land,
    Creature
}

struct Permanent {
    ptype: PermanentType,
    power: i8,
    toughness: i8
}

struct BoardState {
    player1_permanents: Vec<Permanent>,
    player2_permanents: Vec<Permanent>,
}