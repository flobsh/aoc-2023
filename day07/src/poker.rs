use std::{cmp::Ordering, collections::BTreeMap, fmt::Display, str::FromStr};

/// Represents a poker Hand of any card type.
#[derive(Debug, PartialEq, Eq, PartialOrd)]
pub struct Hand<T: PartialEq + Eq + PartialOrd + Ord> {
    cards: Vec<T>,
}

impl<T: PartialEq + Eq + PartialOrd + Ord> Hand<T> {
    /// Creates a new hand from a Vec of cards.
    pub fn new(cards: Vec<T>) -> Self {
        Self { cards }
    }
}

impl<T: PartialEq + Eq + PartialOrd + Ord> Ord for Hand<T> {
    /// Order hands by strength based on the following rules:
    /// - The strongest hand is the one with the strongest hand type
    /// - If hand type is equal on both hands, compare cards one by one
    ///   in the order they appear and return the first non-equal result.
    /// - Both hands are equal if all their cards are the same and appear in the same order,
    ///   or both hands are empty.
    /// - A hand with a least one card is stonger than an empty hand.
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (compute_type(self), compute_type(other)) {
            (Some(s), Some(o)) => s.cmp(&o).then_with(|| {
                self.cards
                    .iter()
                    .zip(other.cards.iter())
                    .map(|(s, o)| s.cmp(o))
                    .find(|cmp| !cmp.is_eq())
                    .unwrap_or(Ordering::Equal)
            }),
            (Some(_), None) => Ordering::Greater,
            (None, Some(_)) => Ordering::Less,
            _ => Ordering::Equal,
        }
    }
}

impl<T: PartialEq + Eq + PartialOrd + Ord + Display> Display for Hand<T> {
    /// Displays cards in a hand, in the order they appear.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.cards
                .iter()
                .map(|card| card.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

impl<T: PartialEq + Eq + PartialOrd + Ord + TryFrom<char>> FromStr for Hand<T> {
    type Err = <T as TryFrom<char>>::Error;

    /// Parses a string to [Hand] of type `T`, using the [`TryFrom<char>`]
    /// trait implementation of `T`.
    fn from_str(cards_str: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(
            cards_str
                .chars()
                .map(|c| TryInto::<T>::try_into(c))
                .collect::<Result<_, _>>()?,
        ))
    }
}

/// Poker hand types.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

/// Computes the poker hand type.
fn compute_type<T: PartialEq + Eq + PartialOrd + Ord>(hand: &Hand<T>) -> Option<HandType> {
    let cards_frequency = hand.cards.iter().fold(BTreeMap::new(), |mut map, card| {
        map.entry(card).and_modify(|freq| *freq += 1).or_insert(1);
        map
    });

    let mut frequencies: Vec<usize> = cards_frequency.values().copied().collect();
    frequencies.sort();
    frequencies.reverse();

    Some(
        match frequencies.get(..std::cmp::min(frequencies.len(), 5))? {
            [x, ..] if *x >= 5 => HandType::FiveOfAKind,
            [4, ..] => HandType::FourOfAKind,
            [3, 2, ..] => HandType::FullHouse,
            [3, ..] => HandType::ThreeOfAKind,
            [2, 2, ..] => HandType::TwoPair,
            [2, ..] => HandType::OnePair,
            _ => HandType::HighCard,
        },
    )
}

#[cfg(test)]
mod tests {
    use crate::poker::{compute_type, HandType};

    use super::Hand;

    #[test]
    fn hand_type_is_one_pair() {
        assert_eq!(
            compute_type(&Hand::new(vec!['3', '2', 'T', '3', 'K'])),
            Some(HandType::OnePair)
        )
    }

    #[test]
    fn hand_type_is_two_pair() {
        assert_eq!(
            compute_type(&Hand::new(vec!['K', 'K', '6', '7', '7'])),
            Some(HandType::TwoPair)
        )
    }

    #[test]
    fn hand_type_is_three_of_a_kind() {
        assert_eq!(
            compute_type(&Hand::new(vec!['T', '5', '5', 'J', '5'])),
            Some(HandType::ThreeOfAKind)
        )
    }

    #[test]
    fn hand_ordering_empty_are_equal() {
        assert!(&Hand::<char>::new(vec![]) == &Hand::<char>::new(vec![]))
    }

    #[test]
    fn hand_ordering_empty_weaker_than_non_empty() {
        assert!(&Hand::new(vec![]) < &Hand::new(vec!['T', '5', '5', 'J', '5']))
    }

    #[test]
    fn hand_ordering_equal() {
        assert!(
            &Hand::new(vec!['T', '5', '5', 'J', '5']) == &Hand::new(vec!['T', '5', '5', 'J', '5'])
        )
    }

    #[test]
    fn hand_ordering_type_based() {
        assert!(
            &Hand::new(vec!['K', 'K', '6', '7', '7']) < &Hand::new(vec!['T', '5', '5', 'J', '5'])
        )
    }

    #[test]
    fn hand_ordering_card_order_based() {
        assert!(
            &Hand::new(vec!['9', '9', '6', '7', '7']) > &Hand::new(vec!['9', '7', '6', '6', '7'])
        )
    }
}
