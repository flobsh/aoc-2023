use aoc_lib::Result;
use day07::camel_cards::CamelCard;
use day07::poker::Hand;

fn parse_input(input: &str) -> Result<Vec<(Hand<CamelCard>, usize)>> {
    Ok(input
        .lines()
        .filter_map(|line| line.split_once(' '))
        .map(|(cards_str, bid)| Ok((cards_str.parse::<Hand<CamelCard>>()?, bid.parse::<usize>()?)))
        .collect::<Result<_>>()?)
}

fn part_1(input: &mut [(Hand<CamelCard>, usize)]) -> usize {
    input.sort_by(|(hand1, _), (hand2, _)| hand1.cmp(hand2));

    input
        .iter()
        .enumerate()
        .map(|(rank, (_, bid))| (rank + 1) * bid)
        .sum()
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("../inputs/07.txt")?;
    let mut input = parse_input(&input)?;

    println!("Day 01 - Part 1: {}", part_1(&mut input));
    // println!("Day 01 - Part 2: {}", part_2(&input));

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
                    Hand::new(vec![
                        CamelCard::Three,
                        CamelCard::Two,
                        CamelCard::Ten,
                        CamelCard::Three,
                        CamelCard::King,
                    ]),
                    765,
                ),
                (
                    Hand::new(vec![
                        CamelCard::Ten,
                        CamelCard::Five,
                        CamelCard::Five,
                        CamelCard::Jack,
                        CamelCard::Five,
                    ]),
                    684,
                ),
                (
                    Hand::new(vec![
                        CamelCard::King,
                        CamelCard::King,
                        CamelCard::Six,
                        CamelCard::Seven,
                        CamelCard::Seven,
                    ]),
                    28,
                ),
                (
                    Hand::new(vec![
                        CamelCard::King,
                        CamelCard::Ten,
                        CamelCard::Jack,
                        CamelCard::Jack,
                        CamelCard::Ten,
                    ]),
                    220,
                ),
                (
                    Hand::new(vec![
                        CamelCard::Queen,
                        CamelCard::Queen,
                        CamelCard::Queen,
                        CamelCard::Jack,
                        CamelCard::Ace,
                    ]),
                    483,
                ),
            ]
        );

        Ok(())
    }

    #[test]
    fn test_part_one() -> Result<()> {
        let input = include_str!("../../inputs/tests/02.txt");
        assert_eq!(part_1(&mut parse_input(input)?), 6440);

        Ok(())
    }
}
