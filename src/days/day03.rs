pub fn part_one(input: &str) -> u32 {
    let mut char_matrix: Vec<Vec<char>> =
        input.lines().map(|line| line.chars().collect()).collect();
    let clone = char_matrix.clone();
    let mut nums: Vec<u32> = vec![];
    clone.iter().enumerate().for_each(|(y, line)| {
        line.iter().enumerate().for_each(|(x, char)| {
            if !char.is_digit(10) && *char != '.' {
                // check surroundings
                check_digit_and_replace(&mut nums, &mut char_matrix, (x - 1, y - 1));
                check_digit_and_replace(&mut nums, &mut char_matrix, (x, y - 1));
                check_digit_and_replace(&mut nums, &mut char_matrix, (x + 1, y - 1));
                check_digit_and_replace(&mut nums, &mut char_matrix, (x - 1, y));
                check_digit_and_replace(&mut nums, &mut char_matrix, (x + 1, y));
                check_digit_and_replace(&mut nums, &mut char_matrix, (x - 1, y + 1));
                check_digit_and_replace(&mut nums, &mut char_matrix, (x, y + 1));
                check_digit_and_replace(&mut nums, &mut char_matrix, (x + 1, y + 1));
            }
        })
    });

    return nums.iter().sum();
}

fn check_digit_and_replace(
    nums: &mut Vec<u32>,
    char_matrix: &mut Vec<Vec<char>>,
    coords: (usize, usize),
) {
    let (x, y) = coords;
    let is_digit = match char_matrix.get(y) {
        Some(line) => match line.get(x) {
            Some(char) => char.is_digit(10),
            None => false,
        },
        None => false,
    };

    if is_digit {
        let rchars: String = char_matrix[y]
            .iter()
            .skip(x)
            .take_while(|char| char.is_digit(10))
            .collect();
        let lchars: String = char_matrix[y]
            .iter()
            .rev()
            .skip(char_matrix[y].len() - x)
            .take_while(|char| char.is_digit(10))
            .map(|c| *c)
            .collect();
        let lfinal: String = lchars.chars().rev().collect();
        for i in &mut char_matrix[y][x - lchars.len()..x + rchars.len()] {
            *i = '.';
        }

        nums.push(format!("{lfinal}{rchars}").parse::<u32>().unwrap());
    }

    ()
}

pub fn part_two(input: &str) -> u32 {
    let mut char_matrix: Vec<Vec<char>> =
        input.lines().map(|line| line.chars().collect()).collect();
    let clone = char_matrix.clone();
    let mut nums: Vec<u32> = vec![];
    clone.iter().enumerate().for_each(|(y, line)| {
        line.iter().enumerate().for_each(|(x, char)| {
            if *char == '*' {
                let mut ns: Vec<u32> = vec![];
                // check surroundings
                check_digit_and_replace(&mut ns, &mut char_matrix, (x - 1, y - 1));
                check_digit_and_replace(&mut ns, &mut char_matrix, (x, y - 1));
                check_digit_and_replace(&mut ns, &mut char_matrix, (x + 1, y - 1));
                check_digit_and_replace(&mut ns, &mut char_matrix, (x - 1, y));
                check_digit_and_replace(&mut ns, &mut char_matrix, (x + 1, y));
                check_digit_and_replace(&mut ns, &mut char_matrix, (x - 1, y + 1));
                check_digit_and_replace(&mut ns, &mut char_matrix, (x, y + 1));
                check_digit_and_replace(&mut ns, &mut char_matrix, (x + 1, y + 1));

                if ns.len() > 1 {
                    nums.push(ns.iter().product());
                }
            }
        })
    });

    return nums.iter().sum();
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
    const PART_ONE_EXPECTED_RESULT: u32 = 4361;
    const PART_TWO_EXPECTED_RESULT: u32 = 467835;

    #[test]
    fn it_solves_part_one_example() {
        assert_eq!(part_one(&INPUT_ONE), PART_ONE_EXPECTED_RESULT);
    }

    #[test]
    fn it_solves_part_two_example() {
        assert_eq!(part_two(&INPUT_TWO), PART_TWO_EXPECTED_RESULT);
    }
}
