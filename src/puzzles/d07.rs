use std::{
    cmp::{Ordering, Reverse},
    error::Error,
    hash::Hash,
    iter::zip,
    str::FromStr,
};

use rustc_hash::FxHashMap;

pub struct HandsList<C: Card> {
    hands: Vec<Hand<C>>,
    bids: Vec<u32>,
}

impl<C: Card> FromStr for HandsList<C> {
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

impl<C> HandsList<C>
where
    C: Card,
    Hand<C>: Ord,
{
    pub fn total_winnings(&self) -> u32 {
        let mut argsort_hands: Vec<usize> = (0..self.hands.len()).collect();
        argsort_hands.sort_by_key(|&r| &self.hands[r]);

        (0..self.hands.len())
            .map(|r| (r + 1) as u32 * self.bids[argsort_hands[r]])
            .sum::<u32>()
    }
}

pub struct Hand<C: Card> {
    cards: [C; 5],
    counts: FxHashMap<C, u8>,
}

impl<C: Card> FromStr for Hand<C> {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cards: [C; 5] = s
            .chars()
            .map(C::new)
            .collect::<Option<Vec<C>>>()
            .ok_or("Invalid card character(s)")?
            .try_into()
            .map_err(|_| "Expected 5 cards")?;

        let mut counts = FxHashMap::default();
        for c in cards {
            *counts.entry(c).or_insert(0u8) += 1;
        }

        Ok(Hand { cards, counts })
    }
}

impl<C: Card> PartialEq for Hand<C> {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}
impl<C: Card> Eq for Hand<C> {}

impl Ord for Hand<Card1> {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_count = self.sorted_counts();
        let other_count = other.sorted_counts();

        if self_count[0] != other_count[0] {
            return self_count[0].cmp(&other_count[0]);
        } else if (self_count[0] == 3 || self_count[0] == 2) && self_count[1] != other_count[1] {
            return self_count[1].cmp(&other_count[1]);
        }

        for (c1, c2) in zip(self.cards, other.cards) {
            if c1 != c2 {
                return c1.cmp(&c2);
            }
        }
        Ordering::Equal
    }
}

impl Ord for Hand<Card2> {
    fn cmp(&self, other: &Self) -> Ordering {
        // TODO: DRY
        let self_count = self.sorted_counts();
        let other_count = other.sorted_counts();

        if self_count[0] != other_count[0] {
            return self_count[0].cmp(&other_count[0]);
        } else if (self_count[0] == 3 || self_count[0] == 2) && self_count[1] != other_count[1] {
            return self_count[1].cmp(&other_count[1]);
        }

        for (c1, c2) in zip(self.cards, other.cards) {
            if c1 != c2 {
                return c1.cmp(&c2);
            }
        }
        Ordering::Equal
    }
}

impl<C> PartialOrd for Hand<C>
where
    C: Card,
    Hand<C>: Ord,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Hand<Card1> {
    fn sorted_counts(&self) -> Vec<u8> {
        let mut counts: Vec<u8> = self.counts.values().cloned().collect();
        counts.sort_by_key(|c| Reverse(*c));

        counts
    }
}

impl Hand<Card2> {
    fn sorted_counts(&self) -> Vec<u8> {
        let joker = Card2::new('J').unwrap();
        let n_joker = *self.counts.get(&joker).unwrap_or(&0);
        let card_max = *self
            .counts
            .iter()
            .filter(|(&k, _)| k != joker)
            .max_by_key(|(_, &v)| v)
            .unwrap_or((&joker, &5))
            .0;

        let mut card_counts = self.counts.clone();
        if n_joker > 0 && card_max != joker {
            *card_counts.get_mut(&card_max).unwrap() += n_joker;
            card_counts.remove(&joker).unwrap();
        }

        let mut counts: Vec<u8> = card_counts.values().cloned().collect();
        counts.sort_by_key(|c| Reverse(*c));

        counts
    }
}

pub trait Card: Eq + Copy + Clone + Hash + PartialEq {
    fn new(c: char) -> Option<Self>;
    fn value(&self) -> u8;
}

#[derive(Eq, Copy, Clone, Hash, PartialEq)]
pub struct Card1(char);

#[derive(Eq, Copy, Clone, Hash, PartialEq)]
pub struct Card2(char);

impl Card for Card1 {
    fn new(c: char) -> Option<Self> {
        if ('2'..='9').contains(&c) || "AKQJT".contains(c) {
            Some(Card1(c))
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

impl Card for Card2 {
    fn new(c: char) -> Option<Self> {
        if ('2'..='9').contains(&c) || "AKQJT".contains(c) {
            Some(Card2(c))
        } else {
            None
        }
    }

    fn value(&self) -> u8 {
        match self.0 {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'T' => 10,
            'J' => 1,
            d => d.to_digit(10).unwrap() as u8,
        }
    }
}

impl Ord for Card1 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value().cmp(&other.value())
    }
}

impl Ord for Card2 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value().cmp(&other.value())
    }
}

impl PartialOrd for Card1 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialOrd for Card2 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
