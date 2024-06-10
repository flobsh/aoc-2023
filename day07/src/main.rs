use aoc_lib::Result;
use day07::camel_cards::CamelCard;

use day07::PokerHand;

type Hand = Vec<CamelCard>;
type Bid = usize;

/// Parses a hand of CamelCards
fn parse_hand(input: &str) -> Result<Hand> {
    input.chars().map(|c| CamelCard::try_from(c)).collect()
}

/// Parses day 07 input into a vector of hands and bids.
fn parse_input(input: &str) -> Result<Vec<(Hand, Bid)>> {
    Ok(input
        .lines()
        .filter_map(|line| line.split_once(' '))
        .map(|(cards, bid)| Ok((parse_hand(cards)?, bid.parse::<usize>()?)))
        .collect::<Result<_>>()?)
}

fn part_1(input: &mut [(Hand, Bid)]) -> usize {
    input.sort_by(|(h1, _), (h2, _)| {
        h1.cmp_to(h2, |h1, h2| match (h1.hand_type(), h2.hand_type()) {
            (Some(t1), Some(t2)) => t1.cmp(&t2).then_with(|| {
                h1.iter()
                    .zip(h2.iter())
                    .map(|(card1, card2)| card1.cmp(card2))
                    .find(|cmp| !cmp.is_eq())
                    .unwrap_or(std::cmp::Ordering::Equal)
            }),
            (Some(_), None) => std::cmp::Ordering::Greater,
            (None, Some(_)) => std::cmp::Ordering::Less,
            _ => std::cmp::Ordering::Equal,
        })
    });

    input
        .iter()
        .enumerate()
        .map(|(rank, (_, bid))| (rank + 1) * bid)
        .sum()
}

fn part_2(input: &mut [(Hand, Bid)]) -> usize {
    todo!()
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("../inputs/07.txt")?;
    let mut input = parse_input(&input)?;

    println!("Day 01 - Part 1: {}", part_1(&mut input));
    println!("Day 01 - Part 2: {}", part_2(&mut input));

    Ok(())
}

#[cfg(test)]
mod tests {
    use aoc_lib::Result;

    use crate::parse_input;
    use crate::part_1;
    use crate::CamelCard;
    use crate::Hand;

    #[test]
    fn test_parse_input() -> Result<()> {
        let input = include_str!("../../inputs/tests/07.txt");

        assert_eq!(
            parse_input(input)?,
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
        );

        Ok(())
    }

    #[test]
    fn test_part_one() -> Result<()> {
        let input = include_str!("../../inputs/tests/07.txt");
        assert_eq!(part_1(&mut parse_input(input)?), 6440);

        Ok(())
    }
}
