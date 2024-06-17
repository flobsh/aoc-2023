use aoc_lib::Error as AoCError;

use day07::poker;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CamelCard {
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
    Ace,
}

impl poker::Card for CamelCard {}

impl TryFrom<char> for CamelCard {
    type Error = AoCError;
    fn try_from(value: char) -> Result<Self, Self::Error> {
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
            _ => Err(AoCError::ParseError(format!("unknown card '{value}'"))),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::camel_cards::CamelCard;

    #[test]
    fn parse_camel_cards() -> aoc_lib::Result<()> {
        let camel_cards_str = "23456789TJQKA";
        let camel_cards: Vec<CamelCard> = camel_cards_str
            .chars()
            .map(|c| TryInto::<CamelCard>::try_into(c))
            .collect::<Result<_, _>>()?;

        assert_eq!(
            camel_cards,
            vec![
                CamelCard::Two,
                CamelCard::Three,
                CamelCard::Four,
                CamelCard::Five,
                CamelCard::Six,
                CamelCard::Seven,
                CamelCard::Eight,
                CamelCard::Nine,
                CamelCard::Ten,
                CamelCard::Jack,
                CamelCard::Queen,
                CamelCard::King,
                CamelCard::Ace,
            ]
        );

        Ok(())
    }
}
