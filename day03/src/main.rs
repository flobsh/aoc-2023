use aoc_lib::Result;

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
/// and symbols' positions in an array of positions (line, column).
fn parse_input(input: &str) -> Result<(Vec<Vec<Number>>, Vec<(usize, usize)>)> {
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
                        symbols.push((line_number, position))
                    }
                    // If we are looking for a number's end and found it, we add the number into the array.
                    // If the character is also a symbol, we add its position into the array.
                    (c, ParseState::LookingForNumberEnd { start }) if !c.is_digit(10) => {
                        numbers_in_line
                            .push(Number::try_from_input(*start, &line[*start..position])?);
                        if c != '.' {
                            symbols.push((line_number, position));
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
fn sum_numbers_around_symbol(numbers: &[Vec<Number>], symbol: &(usize, usize)) -> usize {
    let (symbol_line, symbol_col) = symbol;
    // Compute range going from symbol_line - 1 to symbol_line + 1 (inclusive).
    let range = symbol_line.saturating_sub(1)..=symbol_line + 1;

    range
        .flat_map(|line_number| numbers.get(line_number))
        .map(|number_line| sum_numbers_adjacent_to(number_line, *symbol_col))
        .sum()
}

fn part_1(numbers: &[Vec<Number>], symbols: &[(usize, usize)]) -> usize {
    symbols
        .iter()
        .map(|symbol| sum_numbers_around_symbol(numbers, symbol))
        .sum()
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("../inputs/03.txt")?;
    let (numbers, symbols) = parse_input(&input)?;

    println!("Day 01 - Part 1: {}", part_1(&numbers, &symbols));

    Ok(())
}

#[cfg(test)]
mod tests {
    type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

    use super::*;

    const SAMPLE: &str = include_str!("../../inputs/tests/03.txt");

    fn parsed_sample() -> Result<(Vec<Vec<Number>>, Vec<(usize, usize)>)> {
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
        let symbols = vec![(1, 3), (3, 6), (4, 3), (5, 5), (8, 3), (8, 5)];

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
}
