use {
    crate::{
        card::{Card, Deck, Value},
    }
};

pub enum Action {
    Hit,
    Double,
    Split,
    Stand,
}

pub trait Worth {
    fn worth(&self) -> usize;
}

impl Worth for Deck {
    fn worth(&self) -> usize {
        let mut total = 0;
        let mut aces = 0;
        
        for card in self.cards.clone() {
            if card.value == Value::Ace {
                aces += 1; 
            } else { 
                total += card.worth();
            }
        }
        
        for _ in 0..aces {
            if total <= 10 {
                total += 11; 
            } else {
                total += 1;
            }
        }
        
        total
    }
}

impl Worth for Card {
    fn worth(&self) -> usize {
        match self.value {
            Value::Ace => 1,
            Value::Two => 2,
            Value::Three => 3,
            Value::Four => 4,
            Value::Five => 5,
            Value::Six => 6,
            Value::Seven => 7,
            Value::Eight => 8,
            Value::Nine => 9,
            Value::Ten 
            | Value::Jack
            | Value::Queen
            | Value::King => 10,
        }
    }
}