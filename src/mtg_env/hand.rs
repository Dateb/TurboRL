use std::collections::HashMap;
use std::hash::Hash;
use crate::mtg_env::card::Card;

#[derive(Debug, Clone)]
pub struct Hand<Card> {
    pub card_counts: HashMap<Card, i32>,
    pub size: usize,
}

impl Hand<Card> {
    pub fn new() -> Self {
        let mut card_counts: HashMap<Card, i32> = HashMap::new();
        card_counts.insert(Card::SavannahLion, 0);
        card_counts.insert(Card::Plains, 0);
        Self {
            card_counts,
            size: 0,
        }
    }

    pub fn add_card(&mut self, card: Card) -> () {
        let count = self.card_counts.entry(card.clone()).or_insert(0);
        *count += 1;
        self.size += 1;
    }

    pub fn remove_card(&mut self, card: Card) -> () {
        if let Some(count) = self.card_counts.get_mut(&card) {
            if *count > 0 {
                *count -= 1;
                self.size -= 1;
            }
        }
    }

    pub fn contains_card(&self, card: &Card) -> bool {
        self.card_counts[card] > 0
    }
}

#[cfg(test)]
mod tests {
    use crate::mtg_env::card::Card;
    use super::*;

    #[test]
    fn test_add_card() {
        let mut hand = Hand::new();
        hand.add_card(Card::SavannahLion);
        hand.add_card(Card::SavannahLion);

        hand.add_card(Card::Plains);

        assert!(hand.contains_card(&Card::SavannahLion));
        assert!(hand.contains_card(&Card::Plains));
        assert_eq!(*hand.card_counts.get(&Card::SavannahLion).unwrap(), 2);
        assert_eq!(*hand.card_counts.get(&Card::Plains).unwrap(), 1);
    }

    #[test]
    fn test_remove_card_decrements_count() {
        let mut hand = Hand::new();
        hand.add_card(Card::SavannahLion);
        hand.add_card(Card::SavannahLion);
        hand.add_card(Card::Plains);

        hand.remove_card(Card::SavannahLion);
        assert!(hand.contains_card(&Card::SavannahLion));
        assert_eq!(*hand.card_counts.get(&Card::SavannahLion).unwrap(), 1);

        hand.remove_card(Card::SavannahLion);
        assert!(!hand.contains_card(&Card::SavannahLion));
        assert_eq!(*hand.card_counts.get(&Card::SavannahLion).unwrap(), 0);
    }

    #[test]
    fn test_remove_card_not_in_hand() {
        let mut hand = Hand::new();
        hand.add_card(Card::SavannahLion);

        hand.remove_card(Card::Plains);
        assert!(hand.contains_card(&Card::SavannahLion));
        assert!(!hand.contains_card(&Card::Plains));
        assert_eq!(*hand.card_counts.get(&Card::SavannahLion).unwrap(), 1);
    }

    #[test]
    fn test_contains() {
        let mut hand = Hand::new();
        hand.add_card(Card::SavannahLion);

        assert!(hand.contains_card(&Card::SavannahLion));
        assert!(!hand.contains_card(&Card::Plains));
    }

    #[test]
    fn test_empty_hand() {
        let hand: Hand<Card> = Hand::new();

        assert!(!hand.contains_card(&Card::SavannahLion));
    }

    #[test]
    fn test_remove_last_card_cleans_up() {
        let mut hand = Hand::new();
        hand.add_card(Card::SavannahLion);

        hand.remove_card(Card::SavannahLion);

        assert!(!hand.contains_card(&Card::SavannahLion));
    }
}
