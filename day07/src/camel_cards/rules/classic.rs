use std::{
    cmp::{self, Ordering, Reverse},
    collections::HashMap,
};

use day07::poker::{
    rules::{CompareCards, CompareHands, ComputeHandType},
    HandType,
};

use crate::{camel_cards::CamelCard, CamelHand};

pub struct ClassicRule;

impl CompareCards<CamelCard> for ClassicRule {
    fn cmp_cards(card_1: &CamelCard, card_2: &CamelCard) -> Ordering {
        card_1.cmp(card_2)
    }
}

impl ComputeHandType<CamelHand> for ClassicRule {
    fn hand_type(hand: &CamelHand) -> Option<HandType> {
        let frequencies = {
            let mut frequencies = hand
                .iter()
                .fold(HashMap::new(), |mut frequencies, card| {
                    frequencies
                        .entry(*card)
                        .and_modify(|count| *count += 1)
                        .or_insert(1);
                    frequencies
                })
                .values()
                .copied()
                .collect::<Vec<usize>>();
            frequencies.sort_unstable_by_key(|c| Reverse(*c));
            frequencies
        };

        (!frequencies.is_empty()).then(|| {
            Some(match frequencies.get(..cmp::min(5, frequencies.len()))? {
                [5, ..] => HandType::FiveOfAKind,
                [4, ..] => HandType::FourOfAKind,
                [3, 2, ..] => HandType::FullHouse,
                [3, ..] => HandType::ThreeOfAKind,
                [2, 2, ..] => HandType::TwoPair,
                [2, ..] => HandType::OnePair,
                _ => HandType::HighCard,
            })
        })?
    }
}

impl CompareHands<CamelHand> for ClassicRule {
    fn cmp_hands(hand_1: &CamelHand, hand_2: &CamelHand) -> std::cmp::Ordering {
        Self::hand_type(hand_1)
            .cmp(&Self::hand_type(hand_2))
            .then_with(|| hand_1.iter().cmp(hand_2.iter()))
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use aoc_lib::Result;
    use day07::poker::{
        parse_hand,
        rules::{CompareCards, CompareHands, ComputeHandType},
        HandType,
    };

    use super::ClassicRule;
    use crate::CamelCard;

    #[test]
    fn classic_rule_cmp_camel_cards() {
        assert_eq!(
            ClassicRule::cmp_cards(&CamelCard::Ten, &CamelCard::Jack),
            Ordering::Less
        );
    }

    #[test]
    fn classic_rule_camel_hand_types() -> Result<()> {
        assert_eq!(
            ClassicRule::hand_type(&parse_hand("AAAAA")?),
            Some(HandType::FiveOfAKind)
        );

        assert_eq!(
            ClassicRule::hand_type(&parse_hand("AA8AA")?),
            Some(HandType::FourOfAKind)
        );

        assert_eq!(
            ClassicRule::hand_type(&parse_hand("23332")?),
            Some(HandType::FullHouse)
        );

        assert_eq!(
            ClassicRule::hand_type(&parse_hand("TTT98")?),
            Some(HandType::ThreeOfAKind)
        );

        assert_eq!(
            ClassicRule::hand_type(&parse_hand("23432")?),
            Some(HandType::TwoPair)
        );

        assert_eq!(
            ClassicRule::hand_type(&parse_hand("A23A4")?),
            Some(HandType::OnePair)
        );

        assert_eq!(
            ClassicRule::hand_type(&parse_hand("23456")?),
            Some(HandType::HighCard)
        );

        Ok(())
    }

    #[test]
    fn classic_rule_cmp_camel_hands_equal() -> Result<()> {
        assert_eq!(
            ClassicRule::cmp_hands(&parse_hand("32T3K")?, &parse_hand("32T3K")?),
            Ordering::Equal
        );

        Ok(())
    }

    #[test]
    fn classic_rule_cmp_camel_hands_type_based() -> Result<()> {
        assert_eq!(
            ClassicRule::cmp_hands(&parse_hand("32T3K")?, &parse_hand("KK677")?),
            Ordering::Less
        );

        Ok(())
    }

    #[test]
    fn classic_rule_cmp_camel_hands_iter_based() -> Result<()> {
        assert_eq!(
            ClassicRule::cmp_hands(&parse_hand("KTJJT")?, &parse_hand("KK677")?),
            Ordering::Less
        );

        Ok(())
    }
}
