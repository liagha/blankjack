use std::collections::HashSet;
use core::fmt::Display;
use core::fmt::Formatter;
use rand::{Rng, distr::StandardUniform, prelude::Distribution, seq::SliceRandom, RngExt};

pub trait Worth {
    fn worth(&self) -> usize;
}

pub struct Shoe {
    cards: Vec<Card>,
}

impl Shoe {
    pub fn new(decks: u8) -> Self {
        let mut cards = Vec::with_capacity(52 * decks as usize);
        for _ in 0..decks {
            cards.extend(STANDARD_DECK.iter().copied());
        }
        Shoe { cards }
    }

    pub fn shuffle(&mut self) {
        self.cards.shuffle(&mut rand::rng());
    }

    pub fn draw(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    pub fn remaining(&self) -> usize {
        self.cards.len()
    }
}

pub struct Hand {
    pub cards: Vec<Card>,
}

impl Hand {
    pub fn new() -> Self {
        Hand { cards: Vec::new() }
    }

    pub fn add(&mut self, card: Card) {
        self.cards.push(card);
    }
}

impl Worth for Hand {
    fn worth(&self) -> usize {
        let mut total = 0;
        let mut aces = 0;

        for card in &self.cards {
            if card.value == Value::Ace {
                aces += 1;
            } else {
                total += card.worth();
            }
        }

        for _ in 0..aces {
            if total + 11 <= 21 {
                total += 11;
            } else {
                total += 1;
            }
        }

        total
    }
}

const STANDARD_DECK: [Card; 52] = [
    Card { suit: Suit::Clubs,    value: Value::Ace },
    Card { suit: Suit::Clubs,    value: Value::Two },
    Card { suit: Suit::Clubs,    value: Value::Three },
    Card { suit: Suit::Clubs,    value: Value::Four },
    Card { suit: Suit::Clubs,    value: Value::Five },
    Card { suit: Suit::Clubs,    value: Value::Six },
    Card { suit: Suit::Clubs,    value: Value::Seven },
    Card { suit: Suit::Clubs,    value: Value::Eight },
    Card { suit: Suit::Clubs,    value: Value::Nine },
    Card { suit: Suit::Clubs,    value: Value::Ten },
    Card { suit: Suit::Clubs,    value: Value::Jack },
    Card { suit: Suit::Clubs,    value: Value::Queen },
    Card { suit: Suit::Clubs,    value: Value::King },
    Card { suit: Suit::Diamonds, value: Value::Ace },
    Card { suit: Suit::Diamonds, value: Value::Two },
    Card { suit: Suit::Diamonds, value: Value::Three },
    Card { suit: Suit::Diamonds, value: Value::Four },
    Card { suit: Suit::Diamonds, value: Value::Five },
    Card { suit: Suit::Diamonds, value: Value::Six },
    Card { suit: Suit::Diamonds, value: Value::Seven },
    Card { suit: Suit::Diamonds, value: Value::Eight },
    Card { suit: Suit::Diamonds, value: Value::Nine },
    Card { suit: Suit::Diamonds, value: Value::Ten },
    Card { suit: Suit::Diamonds, value: Value::Jack },
    Card { suit: Suit::Diamonds, value: Value::Queen },
    Card { suit: Suit::Diamonds, value: Value::King },
    Card { suit: Suit::Hearts,   value: Value::Ace },
    Card { suit: Suit::Hearts,   value: Value::Two },
    Card { suit: Suit::Hearts,   value: Value::Three },
    Card { suit: Suit::Hearts,   value: Value::Four },
    Card { suit: Suit::Hearts,   value: Value::Five },
    Card { suit: Suit::Hearts,   value: Value::Six },
    Card { suit: Suit::Hearts,   value: Value::Seven },
    Card { suit: Suit::Hearts,   value: Value::Eight },
    Card { suit: Suit::Hearts,   value: Value::Nine },
    Card { suit: Suit::Hearts,   value: Value::Ten },
    Card { suit: Suit::Hearts,   value: Value::Jack },
    Card { suit: Suit::Hearts,   value: Value::Queen },
    Card { suit: Suit::Hearts,   value: Value::King },
    Card { suit: Suit::Spades,   value: Value::Ace },
    Card { suit: Suit::Spades,   value: Value::Two },
    Card { suit: Suit::Spades,   value: Value::Three },
    Card { suit: Suit::Spades,   value: Value::Four },
    Card { suit: Suit::Spades,   value: Value::Five },
    Card { suit: Suit::Spades,   value: Value::Six },
    Card { suit: Suit::Spades,   value: Value::Seven },
    Card { suit: Suit::Spades,   value: Value::Eight },
    Card { suit: Suit::Spades,   value: Value::Nine },
    Card { suit: Suit::Spades,   value: Value::Ten },
    Card { suit: Suit::Spades,   value: Value::Jack },
    Card { suit: Suit::Spades,   value: Value::Queen },
    Card { suit: Suit::Spades,   value: Value::King },
];

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub struct Card {
    pub suit: Suit,
    pub value: Value,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
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

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
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
            Value::Ten | Value::Jack | Value::Queen | Value::King => 10,
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