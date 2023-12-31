mod days;

use days::{day01, day02, day03};
use std::{fs, time::Instant};

fn main() {
    const DAY: &str = "03";
    run_day(DAY)
}

fn run_day(day: &str) {
    let (part_one, part_two) = get_day_parts(day);
    let input = fs::read_to_string(format!("./inputs/day{day}.txt")).expect("lol I failed that");

    let mut start = Instant::now();
    println!(
        "Part One: {} (took {:?})",
        part_one(&input),
        start.elapsed()
    );
    start = Instant::now();
    println!(
        "Part Two: {} (took {:?})",
        part_two(&input),
        start.elapsed()
    );
}

fn get_day_parts(day: &str) -> (fn(&str) -> u32, fn(&str) -> u32) {
    match day {
        "01" => (day01::part_one, day01::part_two),
        "02" => (day02::part_one, day02::part_two),
        "03" => (day03::part_one, day03::part_two),
        _ => unimplemented!(),
    }
}
