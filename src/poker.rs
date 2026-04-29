use crate::card::{Card, Hand, Suit};

#[derive(PartialEq, PartialOrd, Eq, Ord)]
pub enum Rank {
    High,
    Pair,
    TwoPair,
    Three,
    Straight,
    Flush,
    FullHouse,
    Four,
    StraightFlush,
}

#[derive(PartialEq, PartialOrd, Eq, Ord)]
pub struct Score {
    pub rank: Rank,
    pub power: u32,
}

pub fn evaluate(hole: &Hand, board: &Hand) -> Score {
    let mut pool = Vec::new();
    pool.extend(&hole.cards);
    pool.extend(&board.cards);

    let multi = multiples(&pool);
    let mut best = multi;

    if let Some(suity) = flush(&pool) {
        if suity > best {
            best = suity;
        }
    }

    if let Some(run) = straight(&pool) {
        if run > best {
            best = run;
        }
    }

    best
}

fn multiples(pool: &[Card]) -> Score {
    let mut counts = [0u8; 15];
    for card in pool {
        counts[card.value.score() as usize] += 1;
    }

    let mut items = Vec::new();
    for (val, &count) in counts.iter().enumerate() {
        if count > 0 {
            items.push((count, val as u8));
        }
    }
    items.sort_by(|a, b| b.0.cmp(&a.0).then_with(|| b.1.cmp(&a.1)));

    let rank = match items[0].0 {
        4 => Rank::Four,
        3 if items.len() > 1 && items[1].0 >= 2 => Rank::FullHouse,
        3 => Rank::Three,
        2 if items.len() > 1 && items[1].0 >= 2 => Rank::TwoPair,
        2 => Rank::Pair,
        _ => Rank::High,
    };

    let mut power = 0;
    let mut shift = 16;
    let mut taken = 0;

    for &(count, val) in &items {
        let limit = count.min(5 - taken);
        for _ in 0..limit {
            power += (val as u32) << shift;
            if shift > 0 {
                shift -= 4;
            }
            taken += 1;
        }
        if taken == 5 {
            break;
        }
    }

    Score { rank, power }
}

fn flush(pool: &[Card]) -> Option<Score> {
    let suits = [Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades];

    for suit in suits {
        let mut values = Vec::new();
        for card in pool {
            if card.suit == suit {
                values.push(card.value.score());
            }
        }

        if values.len() >= 5 {
            values.sort_by(|a, b| b.cmp(a));

            if let Some(high) = find_straight(&values) {
                return Some(Score {
                    rank: Rank::StraightFlush,
                    power: high as u32,
                });
            }

            let mut power = 0;
            let mut shift = 16;

            for &val in values.iter().take(5) {
                power += (val as u32) << shift;
                if shift > 0 {
                    shift -= 4;
                }
            }

            return Some(Score {
                rank: Rank::Flush,
                power,
            });
        }
    }

    None
}

fn straight(pool: &[Card]) -> Option<Score> {
    let mut values = Vec::new();
    for card in pool {
        values.push(card.value.score());
    }

    values.sort_by(|a, b| b.cmp(a));
    values.dedup();

    find_straight(&values).map(|high| Score {
        rank: Rank::Straight,
        power: high as u32,
    })
}

fn find_straight(values: &[u8]) -> Option<u8> {
    if values.len() < 5 {
        return None;
    }

    for i in 0..=values.len() - 5 {
        if values[i] - values[i + 4] == 4 {
            return Some(values[i]);
        }
    }

    if values.contains(&14)
        && values.contains(&5)
        && values.contains(&4)
        && values.contains(&3)
        && values.contains(&2)
    {
        return Some(5);
    }

    None
}
