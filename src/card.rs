use hashish::HashMap;
use core::fmt::Display;
use std::fmt::Formatter;
use rand::{Rng, distr::StandardUniform, prelude::Distribution};

#[derive(Debug, Clone)]
pub struct Deck {
    pub cards: HashMap<bool, Card>,
}

#[derive(Debug, Clone, Copy)]
pub struct Card {
    pub suit: Suit,
    pub value: Value,
}

#[derive(Debug, Clone, Copy)]
pub enum Value {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

#[derive(Debug, Clone, Copy)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Ace => write!(f, "A"),
            Value::Two => write!(f, "2"),
            Value::Three => write!(f, "3"),
            Value::Four => write!(f, "4"),
            Value::Five => write!(f, "5"),
            Value::Six => write!(f, "6"),
            Value::Seven => write!(f, "7"),
            Value::Eight => write!(f, "8"),
            Value::Nine => write!(f, "9"),
            Value::Ten => write!(f, "10"),
            Value::Jack => write!(f, "J"),
            Value::Queen => write!(f, "Q"),
            Value::King => write!(f, "K"),
        }
    }
}

impl Display for Suit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Suit::Clubs => write!(f, "♣"),
            Suit::Diamonds => write!(f, "♦"),
            Suit::Hearts => write!(f, "♥"),
            Suit::Spades => write!(f, "♠"),
        }
    }
}

impl Distribution<Card> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Card {
        Card {
            suit: rng.random::<Suit>(),
            value: rng.random::<Value>(),
        }
    }
}

impl Distribution<Value> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Value {
        match rng.random_range(0..13) {
            0 => Value::Ace,
            1 => Value::Two,
            2 => Value::Three,
            3 => Value::Four,
            4 => Value::Five,
            5 => Value::Six,
            6 => Value::Seven,
            7 => Value::Eight,
            8 => Value::Nine,
            9 => Value::Ten,
            10 => Value::Jack,
            11 => Value::Queen,
            12 => Value::King,
            _ => unreachable!(),
        }
    }
}

impl Distribution<Suit> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Suit {
        match rng.random_range(0..4) {
            0 => Suit::Clubs,
            1 => Suit::Diamonds,
            2 => Suit::Hearts,
            3 => Suit::Spades,
            _ => unreachable!(),
        }
    }
}
