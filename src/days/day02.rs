use regex::Regex;

pub fn part_one(input: &str) -> u32 {
    input
        .lines()
        .enumerate()
        .map(|(game_id, line)| {
            let red_regex = Regex::new(r"([\d]+) red").unwrap();
            let green_regex = Regex::new(r"([\d]+) green").unwrap();
            let blue_regex = Regex::new(r"([\d]+) blue").unwrap();

            let reds = has_match_exceeding_limit(line, red_regex, 12);
            let greens = has_match_exceeding_limit(line, green_regex, 13);
            let blues = has_match_exceeding_limit(line, blue_regex, 14);

            if reds == true || greens == true || blues == true {
                0
            } else {
                u32::try_from(game_id + 1).unwrap()
            }
        })
        .sum()
}

pub fn part_two(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let red_regex = Regex::new(r"([\d]+) red").unwrap();
            let green_regex = Regex::new(r"([\d]+) green").unwrap();
            let blue_regex = Regex::new(r"([\d]+) blue").unwrap();

            let reds = get_highest_match(line, red_regex);
            let greens = get_highest_match(line, green_regex);
            let blues = get_highest_match(line, blue_regex);

            reds * greens * blues
        })
        .sum()
}

fn has_match_exceeding_limit(line: &str, regex: Regex, limit: u32) -> bool {
    regex
        .captures_iter(line)
        .map(|captures| {
            let (_, [amount]) = captures.extract();
            amount.parse::<u32>().unwrap()
        })
        .any(|n| n > limit)
}

fn get_highest_match(line: &str, regex: Regex) -> u32 {
    regex
        .captures_iter(line)
        .map(|captures| {
            let (_, [amount]) = captures.extract();
            amount.parse::<u32>().unwrap()
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_ONE: &str = "\
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";
    const PART_ONE_EXPECTED_RESULT: u32 = 8;
    const PART_TWO_EXPECTED_RESULT: u32 = 2286;

    #[test]
    fn it_solves_part_one_example() {
        assert_eq!(part_one(&INPUT_ONE), PART_ONE_EXPECTED_RESULT);
    }

    #[test]
    fn it_solves_part_two_example() {
        assert_eq!(part_two(&INPUT_ONE), PART_TWO_EXPECTED_RESULT);
    }
}
