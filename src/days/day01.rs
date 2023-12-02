use std::collections::HashMap;

pub fn part_one(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let digits: Vec<char> = line.chars().filter(|c| c.is_digit(10)).collect();
            let first = digits.first();
            let last = digits.last();
            match (first, last) {
                (Some(first), Some(last)) => format!("{first}{last}").parse().unwrap(),
                _ => 0,
            }
        })
        .sum()
}

pub fn part_two(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            // written digits
            let zeros = get_written_digit_matches(line, "zero", 0);
            let ones = get_written_digit_matches(line, "one", 1);
            let twos = get_written_digit_matches(line, "two", 2);
            let threes = get_written_digit_matches(line, "three", 3);
            let fours = get_written_digit_matches(line, "four", 4);
            let fives = get_written_digit_matches(line, "five", 5);
            let sixes = get_written_digit_matches(line, "six", 6);
            let sevens = get_written_digit_matches(line, "seven", 7);
            let eights = get_written_digit_matches(line, "eight", 8);
            let nines = get_written_digit_matches(line, "nine", 9);

            // regular digits
            let digits = line
                .chars()
                .enumerate()
                .filter_map(|(index, char)| match char.to_digit(10) {
                    Some(digit) => Some((index, digit)),
                    None => None,
                })
                .collect();

            let mut all = [
                zeros, ones, twos, threes, fours, fives, sixes, sevens, eights, nines, digits,
            ]
            .concat();

            all.sort_by(|a, b| a.0.cmp(&b.0));

            let first = all.first();
            let last = all.last();
            match (first, last) {
                (Some(&(_, d1)), Some(&(_, d2))) => format!("{d1}{d2}").parse().unwrap(),
                _ => 0,
            }
        })
        .sum()
}

fn get_written_digit_matches(line: &str, pattern: &str, digit: u32) -> Vec<(usize, u32)> {
    let mut matches = HashMap::new();
    line.match_indices(pattern).for_each(|(index, _)| {
        matches.insert(index, digit);
        ()
    });

    matches.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_ONE: &str = "\
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
    const INPUT_TWO: &str = "\
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
    const PART_ONE_EXPECTED_RESULT: u32 = 142;
    const PART_TWO_EXPECTED_RESULT: u32 = 281;

    #[test]
    fn it_solves_part_one_example() {
        assert_eq!(part_one(&INPUT_ONE), PART_ONE_EXPECTED_RESULT);
    }

    #[test]
    fn it_solves_part_two_example() {
        assert_eq!(part_two(&INPUT_TWO), PART_TWO_EXPECTED_RESULT);
    }
}
