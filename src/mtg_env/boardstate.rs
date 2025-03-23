use crate::mtg_env::permanent::Permanent;

#[derive(Debug, Clone)]
pub struct BoardState {
    pub creatures: Vec<Permanent>,
    pub lands: Vec<Permanent>,
}

impl BoardState {
    pub fn new() -> Self {
        Self {
            creatures: vec![],
            lands: vec![],
        }
    }
}