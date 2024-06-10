pub mod camel_cards;

use std::collections::BTreeMap;

/// PokerCard trait.
///
/// Poker cards can be ordered.
pub trait PokerCard: PartialEq + Eq + PartialOrd + Ord {}

/// Poker Hand types
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum PokerHandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

/// PokerHand trait.
///
/// One can compute the [PokerHandType] for a PokerHand and poker hands can be compared.
pub trait PokerHand {
    fn hand_type(&self) -> Option<PokerHandType>;
    fn cmp_to<F>(&self, other: &Self, strategy: F) -> std::cmp::Ordering
    where
        F: Fn(&Self, &Self) -> std::cmp::Ordering,
    {
        strategy(self, other)
    }
}

/// PokerHand trait blanket implementation for all types whose reference implements the IntoIterator trait.
impl<T: ?Sized, U> PokerHand for T
where
    for<'b> &'b T: IntoIterator<Item = &'b U>,
    U: PokerCard,
{
    fn hand_type(&self) -> Option<PokerHandType> {
        let cards_frequency = self.into_iter().fold(BTreeMap::new(), |mut map, card| {
            map.entry(card).and_modify(|freq| *freq += 1).or_insert(1);
            map
        });

        let frequencies = {
            let mut frequencies: Vec<usize> = cards_frequency.values().copied().collect();
            frequencies.sort();
            frequencies.reverse();
            frequencies
        };

        Some(
            match frequencies
                .get(..std::cmp::min(frequencies.len(), 5))
                .filter(|a| !a.is_empty())?
            {
                [x, ..] if *x >= 5 => PokerHandType::FiveOfAKind,
                [4, ..] => PokerHandType::FourOfAKind,
                [3, 2, ..] => PokerHandType::FullHouse,
                [3, ..] => PokerHandType::ThreeOfAKind,
                [2, 2, ..] => PokerHandType::TwoPair,
                [2, ..] => PokerHandType::OnePair,
                _ => PokerHandType::HighCard,
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::{PokerCard, PokerHand, PokerHandType};

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    enum BasicCard {
        Ten,
        Jack,
        Queen,
        King,
        Ace,
    }

    impl PokerCard for BasicCard {}

    #[test]
    fn hand_type_five_of_a_kind() {
        let hand = vec![BasicCard::Jack; 5];
        assert_eq!(hand.hand_type(), Some(PokerHandType::FiveOfAKind));
    }

    #[test]
    fn hand_type_four_of_a_kind() {
        let hand = vec![
            BasicCard::Jack,
            BasicCard::Jack,
            BasicCard::Jack,
            BasicCard::Jack,
            BasicCard::Queen,
        ];
        assert_eq!(hand.hand_type(), Some(PokerHandType::FourOfAKind));
    }

    #[test]
    fn hand_type_full_house() {
        let hand = vec![
            BasicCard::Jack,
            BasicCard::Jack,
            BasicCard::Jack,
            BasicCard::Ten,
            BasicCard::Ten,
        ];
        assert_eq!(hand.hand_type(), Some(PokerHandType::FullHouse));
    }

    #[test]
    fn hand_type_three_of_a_kind() {
        let hand = vec![
            BasicCard::Jack,
            BasicCard::Jack,
            BasicCard::Jack,
            BasicCard::Ten,
            BasicCard::Queen,
        ];
        assert_eq!(hand.hand_type(), Some(PokerHandType::ThreeOfAKind));
    }

    #[test]
    fn hand_type_two_pair() {
        let hand = vec![
            BasicCard::Jack,
            BasicCard::Jack,
            BasicCard::Queen,
            BasicCard::Ten,
            BasicCard::Queen,
        ];
        assert_eq!(hand.hand_type(), Some(PokerHandType::TwoPair));
    }

    #[test]
    fn hand_type_one_pair() {
        let hand = vec![
            BasicCard::Jack,
            BasicCard::Jack,
            BasicCard::King,
            BasicCard::Ten,
            BasicCard::Queen,
        ];
        assert_eq!(hand.hand_type(), Some(PokerHandType::OnePair));
    }

    #[test]
    fn hand_type_high_card() {
        let hand = vec![
            BasicCard::Ace,
            BasicCard::Jack,
            BasicCard::King,
            BasicCard::Ten,
            BasicCard::Queen,
        ];
        assert_eq!(hand.hand_type(), Some(PokerHandType::HighCard));
    }

    #[test]
    fn hand_type_none() {
        let hand = Vec::<BasicCard>::new();
        assert_eq!(hand.hand_type(), None);
    }
}
