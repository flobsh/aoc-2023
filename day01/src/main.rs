const NUMBERS: [&str; 20] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "0", "1", "2",
    "3", "4", "5", "6", "7", "8", "9",
];

fn str_to_double_digit_number(input: &str) -> Option<usize> {
    let first_digit = input.chars().find_map(|c| c.to_digit(10));
    let last_digit = input.chars().rev().find_map(|c| c.to_digit(10));

    Some((first_digit? * 10 + last_digit?) as usize)
}

fn str_literal_to_double_digit_number(input: &str) -> Option<usize> {
    let first_digit = (0..input.len()).find_map(|i| {
        NUMBERS
            .iter()
            .enumerate()
            .find_map(|(j, num)| input[i..].starts_with(num).then_some(j % 10))
    });
    let last_digit = (0..input.len()).rev().find_map(|i| {
        NUMBERS
            .iter()
            .enumerate()
            .find_map(|(j, num)| input[i..].starts_with(num).then_some(j % 10))
    });

    Some(first_digit? * 10 + last_digit?)
}

fn parse_input(input: &str, parse_line: fn(&str) -> Option<usize>) -> usize {
    input
        .lines()
        .map(|line| parse_line(line).expect(&format!("{line} should contain a number")))
        .sum()
}

fn part_1(input: &str) -> usize {
    parse_input(input, str_to_double_digit_number)
}

fn part_2(input: &str) -> usize {
    parse_input(input, str_literal_to_double_digit_number)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("../inputs/01.txt")?;

    println!("Day 01 - Part 1: {}", part_1(&input));
    println!("Day 01 - Part 2: {}", part_2(&input));

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn part_one() {
        let input = include_str!("../../inputs/tests/01-1.txt");
        assert_eq!(part_1(input), 142);
    }

    #[test]
    fn part_two() {
        let input = include_str!("../../inputs/tests/01-2.txt");
        assert_eq!(part_2(input), 281);
    }
}
