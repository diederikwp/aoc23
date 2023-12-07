use std::{
    cmp::{Ordering, Reverse},
    error::Error,
    iter::zip,
    str::FromStr,
};

use rustc_hash::FxHashMap;

pub struct HandsList {
    hands: Vec<Hand>,
    bids: Vec<u32>,
}

impl FromStr for HandsList {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut hands = Vec::new();
        let mut bids = Vec::new();

        for l in s.lines() {
            let (hand_str, bid_str) = l.split_once(' ').ok_or("Invalid syntax")?;
            let hand = hand_str.parse()?;
            let bid = bid_str.parse()?;

            hands.push(hand);
            bids.push(bid);
        }

        Ok(HandsList { hands, bids })
    }
}

impl HandsList {
    pub fn total_winnings(&self) -> u32 {
        let mut argsort_hands: Vec<usize> = (0..self.hands.len()).collect();
        argsort_hands.sort_by_key(|&r| &self.hands[r]);

        (0..self.hands.len())
            .map(|r| (r + 1) as u32 * self.bids[argsort_hands[r]])
            .sum::<u32>()
    }
}

struct Hand {
    cards: [Card; 5],
    counts: Vec<u8>,
}

impl FromStr for Hand {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cards: [Card; 5] = s
            .chars()
            .map(Card::new)
            .collect::<Option<Vec<Card>>>()
            .ok_or("Invalid card character(s)")?
            .try_into()
            .map_err(|_| "Expected 5 cards")?;

        let mut counts = FxHashMap::default();
        for c in cards {
            *counts.entry(c).or_insert(0u8) += 1;
        }
        let mut counts: Vec<u8> = counts.values().cloned().collect();
        counts.sort_by_key(|c| Reverse(*c));

        Ok(Hand { cards, counts })
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}
impl Eq for Hand {}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.counts[0] != other.counts[0] {
            return self.counts[0].cmp(&other.counts[0]);
        } else if (self.counts[0] == 3 || self.counts[0] == 2) && self.counts[1] != other.counts[1]
        {
            return self.counts[1].cmp(&other.counts[1]);
        }

        for (c1, c2) in zip(self.cards, other.cards) {
            if c1 != c2 {
                return c1.cmp(&c2);
            }
        }
        Ordering::Equal
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Eq, Copy, Clone, Hash, PartialEq)]
struct Card(char);

impl Card {
    fn new(c: char) -> Option<Self> {
        if ('2'..='9').contains(&c) || "AKQJT".contains(c) {
            Some(Card(c))
        } else {
            None
        }
    }

    fn value(&self) -> u8 {
        match self.0 {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 11,
            'T' => 10,
            d => d.to_digit(10).unwrap() as u8,
        }
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value().cmp(&other.value())
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
