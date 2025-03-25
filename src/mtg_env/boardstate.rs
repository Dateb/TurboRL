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

    pub fn get_creature_count(&self) -> usize {
        self.creatures.len()
    }

    pub fn get_land_count(&self) -> usize {
        self.lands.len()
    }
}