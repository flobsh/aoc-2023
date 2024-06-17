use aoc_lib::Result;
use day07::poker::{self, rules::CompareHands};

mod camel_cards;

use crate::camel_cards::rules::ClassicRule;
use crate::camel_cards::rules::JaJRule;
use crate::camel_cards::CamelCard;

type CamelHand = Vec<CamelCard>;
type Bid = usize;

fn parse_input(input: &str) -> Result<Vec<(CamelHand, Bid)>> {
    input
        .lines()
        .filter_map(|line| line.split_once(' '))
        .map(|(hand_str, bid_str)| Ok((poker::parse_hand(hand_str)?, bid_str.parse()?)))
        .collect::<Result<_>>()
}

fn part_one(input: &mut [(CamelHand, Bid)]) -> usize {
    input.sort_by(|(hand_1, _), (hand_2, _)| ClassicRule::cmp_hands(hand_1, hand_2));

    input
        .iter()
        .enumerate()
        .map(|(i, (_, bid))| bid * (i + 1))
        .sum()
}

fn part_two(input: &mut [(CamelHand, Bid)]) -> usize {
    input.sort_by(|(hand_1, _), (hand_2, _)| JaJRule::cmp_hands(hand_1, hand_2));

    input
        .iter()
        .enumerate()
        .map(|(i, (_, bid))| bid * (i + 1))
        .sum()
}

fn main() -> aoc_lib::Result<()> {
    let mut input = parse_input(&std::fs::read_to_string("../inputs/07.txt")?)?;

    println!("Day 07 - Part 1: {}", part_one(&mut input));
    println!("Day 07 - Part 2: {}", part_two(&mut input));

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::parse_input;
    use crate::Bid;
    use crate::CamelCard;
    use crate::CamelHand;
    use crate::{part_one, part_two};
    use aoc_lib::Result;

    fn example_data() -> Vec<(CamelHand, Bid)> {
        vec![
            (
                vec![
                    CamelCard::Three,
                    CamelCard::Two,
                    CamelCard::Ten,
                    CamelCard::Three,
                    CamelCard::King,
                ],
                765,
            ),
            (
                vec![
                    CamelCard::Ten,
                    CamelCard::Five,
                    CamelCard::Five,
                    CamelCard::Jack,
                    CamelCard::Five,
                ],
                684,
            ),
            (
                vec![
                    CamelCard::King,
                    CamelCard::King,
                    CamelCard::Six,
                    CamelCard::Seven,
                    CamelCard::Seven,
                ],
                28,
            ),
            (
                vec![
                    CamelCard::King,
                    CamelCard::Ten,
                    CamelCard::Jack,
                    CamelCard::Jack,
                    CamelCard::Ten,
                ],
                220,
            ),
            (
                vec![
                    CamelCard::Queen,
                    CamelCard::Queen,
                    CamelCard::Queen,
                    CamelCard::Jack,
                    CamelCard::Ace,
                ],
                483,
            ),
        ]
    }

    #[test]
    fn parse_example() -> Result<()> {
        let input = include_str!("../../inputs/tests/07.txt");

        assert_eq!(parse_input(input)?, example_data());

        Ok(())
    }

    #[test]
    fn part_one_example() {
        assert_eq!(part_one(&mut example_data()), 6440)
    }

    #[test]
    fn part_two_example() {
        assert_eq!(part_two(&mut example_data()), 5905)
    }
}
