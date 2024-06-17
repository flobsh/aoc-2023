use std::collections::{BTreeSet, HashSet, LinkedList};

use crate::poker::Card;

// Regular poker hand types.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

// Represents a poker hand.
pub trait Hand {
    type Card: Card;
}

impl<T: Card> Hand for Vec<T> {
    type Card = T;
}

impl<T: Card> Hand for HashSet<T> {
    type Card = T;
}

impl<T: Card> Hand for BTreeSet<T> {
    type Card = T;
}

impl<T: Card> Hand for LinkedList<T> {
    type Card = T;
}

// Parses a string to a Hand, if the target Card type
// implements the TryFrom<char> trait.
pub fn parse_hand<T: Hand<Card: TryFrom<char>> + FromIterator<T::Card>>(
    input: &str,
) -> Result<T, <<T as Hand>::Card as TryFrom<char>>::Error> {
    input
        .chars()
        .map(|c| TryInto::<T::Card>::try_into(c))
        .collect::<Result<_, _>>()
}

#[cfg(test)]
mod tests {
    use crate::poker::{parse_hand, Card};

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
    enum BasicCard {
        Queen,
        King,
        Ace,
    }

    impl Card for BasicCard {}

    impl TryFrom<char> for BasicCard {
        type Error = aoc_lib::Error;

        fn try_from(value: char) -> Result<Self, Self::Error> {
            match value {
                'Q' => Ok(BasicCard::Queen),
                'K' => Ok(BasicCard::King),
                'A' => Ok(BasicCard::Ace),
                _ => Err(aoc_lib::Error::ParseError(format!("unknown card {value}"))),
            }
        }
    }

    #[test]
    fn parse_hand_basic_cards() -> aoc_lib::Result<()> {
        assert_eq!(
            parse_hand::<Vec<BasicCard>>("QKA")?,
            vec![BasicCard::Queen, BasicCard::King, BasicCard::Ace]
        );

        Ok(())
    }
}
