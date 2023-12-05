pub fn part_one(_input: &str) -> u32 {
    return 0;
}

pub fn part_two(_input: &str) -> u32 {
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_ONE: &str = "\
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";
    const INPUT_TWO: &str = "\
";
    const PART_ONE_EXPECTED_RESULT: u32 = 0;
    const PART_TWO_EXPECTED_RESULT: u32 = 0;

    #[test]
    fn it_solves_part_one_example() {
        assert_eq!(part_one(&INPUT_ONE), PART_ONE_EXPECTED_RESULT);
    }

    #[test]
    fn it_solves_part_two_example() {
        assert_eq!(part_two(&INPUT_TWO), PART_TWO_EXPECTED_RESULT);
    }
}
