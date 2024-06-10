use aoc_lib::Error;
use aoc_lib::Result;

use crate::PokerCard;

/// Set of Camel Cards, as defined in Advent of Code 2023, Day 07.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum CamelCard {
    Two = 2,
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
    Ace,
}

impl PokerCard for CamelCard {}

impl TryFrom<char> for CamelCard {
    type Error = Error;

    /// Converts a char to a [CamelCard].
    fn try_from(value: char) -> Result<Self> {
        match value {
            '2' => Ok(Self::Two),
            '3' => Ok(Self::Three),
            '4' => Ok(Self::Four),
            '5' => Ok(Self::Five),
            '6' => Ok(Self::Six),
            '7' => Ok(Self::Seven),
            '8' => Ok(Self::Eight),
            '9' => Ok(Self::Nine),
            'T' => Ok(Self::Ten),
            'J' => Ok(Self::Jack),
            'Q' => Ok(Self::Queen),
            'K' => Ok(Self::King),
            'A' => Ok(Self::Ace),
            _ => Err(aoc_lib::Error::ParseError(format!(
                "{value} does not correspond to any card"
            ))),
        }
    }
}

/// Parses a hand of CamelCards
pub fn parse_hand(input: &str) -> Result<Vec<CamelCard>> {
    input.chars().map(|c| CamelCard::try_from(c)).collect()
}

#[cfg(test)]
mod tests {
    use aoc_lib::Result;

    use crate::camel_cards::{parse_hand, CamelCard};

    #[test]
    fn parse_hands() -> Result<()> {
        assert_eq!(
            parse_hand("32T3K")?,
            vec![
                CamelCard::Three,
                CamelCard::Two,
                CamelCard::Ten,
                CamelCard::Three,
                CamelCard::King
            ]
        );
        assert_eq!(
            parse_hand("T55J5")?,
            vec![
                CamelCard::Ten,
                CamelCard::Five,
                CamelCard::Five,
                CamelCard::Jack,
                CamelCard::Five
            ]
        );
        assert_eq!(
            parse_hand("KK677")?,
            vec![
                CamelCard::King,
                CamelCard::King,
                CamelCard::Six,
                CamelCard::Seven,
                CamelCard::Seven
            ]
        );
        assert_eq!(
            parse_hand("KTJJT")?,
            vec![
                CamelCard::King,
                CamelCard::Ten,
                CamelCard::Jack,
                CamelCard::Jack,
                CamelCard::Ten
            ]
        );
        assert_eq!(
            parse_hand("QQQJA")?,
            vec![
                CamelCard::Queen,
                CamelCard::Queen,
                CamelCard::Queen,
                CamelCard::Jack,
                CamelCard::Ace
            ]
        );

        Ok(())
    }
}
