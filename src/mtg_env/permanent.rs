#[derive(Debug, Clone)]
pub enum PermanentType {
    Land,
    Creature
}

#[derive(Debug, Clone)]
pub struct Permanent {
    pub ptype: PermanentType,
    pub power: usize,
    pub toughness: usize,
    pub is_tapped: bool
}
