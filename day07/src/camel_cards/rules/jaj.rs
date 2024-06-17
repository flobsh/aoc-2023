use std::{cmp::Ordering, marker::PhantomData};

use day07::poker::{
    rules::{CompareCards, CompareHands, ComputeHandType},
    Card, HandType,
};

use crate::{
    camel_cards::{rules::ClassicRule, CamelCard},
    CamelHand,
};

pub struct JaJRule<T: Card> {
    card_type: PhantomData<T>,
}

impl JaJRule<CamelCard> {
    const JOKER: CamelCard = CamelCard::Jack;
}

impl CompareCards<CamelCard> for JaJRule<CamelCard> {
    fn cmp_cards(card_1: &CamelCard, card_2: &CamelCard) -> Ordering {
        match (*card_1, *card_2) {
            (Self::JOKER, Self::JOKER) => Ordering::Equal,
            (Self::JOKER, _) => Ordering::Less,
            (_, Self::JOKER) => Ordering::Greater,
            _ => ClassicRule::cmp_cards(card_1, card_2),
        }
    }
}

impl ComputeHandType<CamelHand> for JaJRule<CamelCard> {
    fn hand_type(hand: &CamelHand) -> Option<HandType> {
        if hand.contains(&Self::JOKER) {
            let hand_no_joker = hand
                .iter()
                .filter(|card| **card != Self::JOKER)
                .copied()
                .collect();

            let hand_type_no_joker = ClassicRule::hand_type(&hand_no_joker);
            let joker_count = hand.len() - hand_no_joker.len();

            Some(match hand_type_no_joker {
                None => match joker_count {
                    1 => HandType::HighCard,
                    2 => HandType::OnePair,
                    3 => HandType::ThreeOfAKind,
                    4 => HandType::FourOfAKind,
                    _ => HandType::FiveOfAKind,
                },
                Some(HandType::HighCard) => match joker_count {
                    1 => HandType::OnePair,
                    2 => HandType::ThreeOfAKind,
                    3 => HandType::FourOfAKind,
                    _ => HandType::FiveOfAKind,
                },
                Some(HandType::OnePair) => match joker_count {
                    1 => HandType::ThreeOfAKind,
                    2 => HandType::FourOfAKind,
                    _ => HandType::FiveOfAKind,
                },
                Some(HandType::TwoPair) => match joker_count {
                    _ => HandType::FullHouse,
                },
                Some(HandType::ThreeOfAKind) => match joker_count {
                    1 => HandType::FourOfAKind,
                    _ => HandType::FiveOfAKind,
                },
                Some(HandType::FullHouse) => HandType::FullHouse,
                Some(HandType::FourOfAKind) | Some(HandType::FiveOfAKind) => HandType::FiveOfAKind,
            })
        } else {
            ClassicRule::hand_type(hand)
        }
    }
}

impl CompareHands<CamelHand> for JaJRule<CamelCard> {
    fn cmp_hands(hand_1: &CamelHand, hand_2: &CamelHand) -> Ordering {
        Self::hand_type(hand_1)
            .cmp(&Self::hand_type(hand_2))
            .then_with(|| {
                hand_1
                    .iter()
                    .zip(hand_2.iter())
                    .map(|(card_1, card_2)| JaJRule::cmp_cards(card_1, card_2))
                    .find(|cmp| !cmp.is_eq())
                    .unwrap_or(Ordering::Equal)
            })
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use aoc_lib::Result;

    use crate::CamelCard;
    use day07::poker::{
        parse_hand,
        rules::{CompareCards, CompareHands, ComputeHandType},
        HandType,
    };

    use crate::camel_cards::rules::JaJRule;

    #[test]
    fn jaj_rule_cmp_camel_cards() {
        // Jack is lower than Two
        assert_eq!(
            JaJRule::cmp_cards(&CamelCard::Jack, &CamelCard::Two),
            Ordering::Less
        );
        // Two Jacks are equal
        assert_eq!(
            JaJRule::cmp_cards(&CamelCard::Jack, &CamelCard::Jack),
            Ordering::Equal
        );
    }

    #[test]
    fn jaj_rule_hand_types() -> Result<()> {
        assert_eq!(
            JaJRule::hand_type(&parse_hand("32T3K")?),
            Some(HandType::OnePair)
        );

        assert_eq!(
            JaJRule::hand_type(&parse_hand("KK677")?),
            Some(HandType::TwoPair)
        );

        assert_eq!(
            JaJRule::hand_type(&parse_hand("T55J5")?),
            Some(HandType::FourOfAKind)
        );

        assert_eq!(
            JaJRule::hand_type(&parse_hand("KTJJT")?),
            Some(HandType::FourOfAKind)
        );

        assert_eq!(
            JaJRule::hand_type(&parse_hand("QQQJA")?),
            Some(HandType::FourOfAKind)
        );

        Ok(())
    }

    #[test]
    fn jaj_rule_cmp_camel_hands_equal() -> Result<()> {
        assert_eq!(
            JaJRule::cmp_hands(&parse_hand("32T3K")?, &parse_hand("32T3K")?),
            Ordering::Equal
        );

        Ok(())
    }

    #[test]
    fn jaj_rule_cmp_camel_hands_type_based() -> Result<()> {
        assert_eq!(
            JaJRule::cmp_hands(&parse_hand("KK677")?, &parse_hand("T55J5")?),
            Ordering::Less
        );

        Ok(())
    }

    #[test]
    fn jaj_rule_cmp_camel_hands_iter_based() -> Result<()> {
        assert_eq!(
            JaJRule::cmp_hands(&parse_hand("T55J5")?, &parse_hand("KTJJT")?),
            Ordering::Less
        );

        Ok(())
    }
}
