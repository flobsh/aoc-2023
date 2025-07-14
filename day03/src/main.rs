//! Advent of Code 2023 Day 3
//!
//! [](https://adventofcode.com/2023/day/3)
//!
//! All symbols are stored in `Vec<Symbol>` that
//! contains the character and position of each symbol: (line_number, column_number).
//!
//! Numbers are stored in a `Vec<Vec<Number>>`, line by line, e.g.
//! `numbers[2][1]` is the second number on the third line, so 633 in
//! the given example.
//!
//! # Part 1 algorithm:
//!
//! For each symbol, we check if numbers are adjacent to it
//! on the same line, the line above and the line below it.
//! Then all adjacent numbers are summed up, and the operation
//! is repeated for all symbols.
//!
//! # Part 2 algorithm:
//!
//! For each `*` symbol, we check if it has exactly two numbers
//! adjacent to it. Then these numbers are multiplied.
use aoc_lib::Result;

/// Symbol struct
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Symbol {
    position: (usize, usize),
    sym: char,
}

/// Struct that holds the position of a number
/// on a line, as well as its length and its value as `usize`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Number {
    position: usize,
    len: usize,
    value: usize,
}

impl Number {
    /// Parse a number from a string, starting at `start_position`.
    fn try_from_input(start_position: usize, input: &str) -> Result<Self> {
        Ok(Self {
            position: start_position,
            len: input.len(),
            value: input.parse()?,
        })
    }

    /// Check if the number is adjacent to the given position.
    fn is_adjacent_to(&self, position: usize) -> bool {
        position >= self.position.saturating_sub(1) && position <= self.position + self.len
    }
}

/// Enum to help parsing the input.
enum ParseState {
    LookingForDigitOrSymbol,
    LookingForNumberEnd { start: usize },
}

/// Parse the challenge's input, returning found numbers in a two-dimension array of [`Number`],
/// and symbols in a `Vec<Symbol>`.
fn parse_input(input: &str) -> Result<(Vec<Vec<Number>>, Vec<Symbol>)> {
    input.lines().enumerate().try_fold(
        (Vec::new(), Vec::new()),
        |(mut numbers, mut symbols), (line_number, line)| {
            let mut numbers_in_line = Vec::new();
            let mut state = ParseState::LookingForDigitOrSymbol;

            for (position, character) in line.chars().enumerate() {
                match (character, &state) {
                    // If we are looking for a digit and found one, we record its position and
                    // change state to look for the number's end.
                    (c, ParseState::LookingForDigitOrSymbol) if c.is_digit(10) => {
                        state = ParseState::LookingForNumberEnd { start: position }
                    }
                    // If we are looking for a symbol and found one, we add its position into
                    // the array.
                    (c, ParseState::LookingForDigitOrSymbol) if c != '.' => {
                        symbols.push(Symbol {
                            position: (line_number, position),
                            sym: c,
                        });
                    }
                    // If we are looking for a number's end and found it, we add the number into the array.
                    // If the character is also a symbol, we add its position into the array.
                    (c, ParseState::LookingForNumberEnd { start }) if !c.is_digit(10) => {
                        numbers_in_line
                            .push(Number::try_from_input(*start, &line[*start..position])?);
                        if c != '.' {
                            symbols.push(Symbol {
                                position: (line_number, position),
                                sym: c,
                            });
                        }
                        state = ParseState::LookingForDigitOrSymbol;
                    }
                    (_, _) => (),
                }
            }

            // If we are still looking for a number's end, we end it here.
            if let ParseState::LookingForNumberEnd { start } = state {
                numbers_in_line.push(Number::try_from_input(start, &line[start..])?);
            }

            numbers.push(numbers_in_line);
            Ok((numbers, symbols))
        },
    )
}

/// Compute the sum of numbers adjacent to the provided position on one line.
fn sum_numbers_adjacent_to(numbers: &[Number], symbol_position: usize) -> usize {
    numbers
        .iter()
        .filter(|number| number.is_adjacent_to(symbol_position))
        .map(|number| number.value)
        .sum()
}

/// Compute the sum of numbers adjacent to the provided symbol.
fn sum_numbers_around_symbol(numbers: &[Vec<Number>], symbol: &Symbol) -> usize {
    let (symbol_line, symbol_col) = symbol.position;
    // Compute range going from symbol_line - 1 to symbol_line + 1 (inclusive).
    let range = symbol_line.saturating_sub(1)..=symbol_line + 1;

    range
        // Safely index into `numbers` to get a line.
        .flat_map(|line_number| numbers.get(line_number))
        .map(|number_line| sum_numbers_adjacent_to(number_line, symbol_col))
        .sum()
}

fn part_1(numbers: &[Vec<Number>], symbols: &[Symbol]) -> usize {
    symbols
        .iter()
        .map(|symbol| sum_numbers_around_symbol(numbers, symbol))
        .sum()
}

fn compute_gear_ratio(numbers: &[Vec<Number>], symbol: &Symbol) -> Option<usize> {
    let (symbol_line, symbol_col) = symbol.position;
    // Compute range going from symbol_line - 1 to symbol_line + 1 (inclusive).
    let range = symbol_line.saturating_sub(1)..=symbol_line + 1;

    let candidate_numbers: Vec<&Number> = range
        // Safely index into `numbers` to get a line.
        .flat_map(|line_number| numbers.get(line_number))
        // Then iterate over all the numbers of the line
        .flat_map(|number_line| number_line.iter())
        // And keep only those adjacent to the symbol
        .filter(|number| number.is_adjacent_to(symbol_col))
        .collect();

    match candidate_numbers.len() {
        2 => Some(
            candidate_numbers
                .iter()
                .map(|number| number.value)
                .product(),
        ),
        _ => None,
    }
}

fn part_2(numbers: &[Vec<Number>], symbols: &[Symbol]) -> usize {
    symbols
        .iter()
        // Keep only '*' symbols
        .filter(|symbol| symbol.sym == '*')
        // And filter those with exactly 2 adjacent numbers, computing their product
        .filter_map(|symbol| compute_gear_ratio(numbers, symbol))
        .sum()
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("../inputs/03.txt")?;
    let (numbers, symbols) = parse_input(&input)?;

    println!("Day 03 - Part 1: {}", part_1(&numbers, &symbols));
    println!("Day 03 - Part 2: {}", part_2(&numbers, &symbols));

    Ok(())
}

#[cfg(test)]
mod tests {
    type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

    use super::*;

    const SAMPLE: &str = include_str!("../../inputs/tests/03.txt");

    fn parsed_sample() -> Result<(Vec<Vec<Number>>, Vec<Symbol>)> {
        let numbers = vec![
            vec![
                Number::try_from_input(0, "467")?,
                Number::try_from_input(5, "114")?,
            ],
            vec![],
            vec![
                Number::try_from_input(2, "35")?,
                Number::try_from_input(6, "633")?,
            ],
            vec![],
            vec![Number::try_from_input(0, "617")?],
            vec![Number::try_from_input(7, "58")?],
            vec![Number::try_from_input(2, "592")?],
            vec![Number::try_from_input(6, "755")?],
            vec![],
            vec![
                Number::try_from_input(1, "664")?,
                Number::try_from_input(5, "598")?,
            ],
        ];
        let symbols = vec![
            Symbol {
                position: (1, 3),
                sym: '*',
            },
            Symbol {
                position: (3, 6),
                sym: '#',
            },
            Symbol {
                position: (4, 3),
                sym: '*',
            },
            Symbol {
                position: (5, 5),
                sym: '+',
            },
            Symbol {
                position: (8, 3),
                sym: '$',
            },
            Symbol {
                position: (8, 5),
                sym: '*',
            },
        ];

        Ok((numbers, symbols))
    }

    #[test]
    fn test_parse_sample() -> Result<()> {
        assert_eq!(parse_input(SAMPLE)?, parsed_sample()?);

        Ok(())
    }

    #[test]
    fn test_part_1() -> Result<()> {
        let (numbers, symbols) = parsed_sample()?;
        assert_eq!(part_1(&numbers, &symbols), 4361);

        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        let (numbers, symbols) = parsed_sample()?;
        assert_eq!(part_2(&numbers, &symbols), 467835);

        Ok(())
    }
}
