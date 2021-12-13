use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;
mod day_2;
mod util;
mod day_3;

fn main() {
    day_1();
    day_2::day_2();
    day_3::part_1();
}

fn day_1() {
    let numbers: Vec<u32> = util::read_lines("./day_1_input.txt").ok().unwrap()
        .filter_map(|s| s.ok())
        .filter_map(|s| s.parse::<u32>().ok())
        .collect();

    println!("input {:?}", numbers.len());
    let mut count_increased = 0;
    for pair in numbers.windows(2) {
        if pair[0] < pair[1] {
            count_increased += 1;
        }
    }

    println!("Day 1 Part 1 result: {}", count_increased);

    // part 2
    let windowed_numbers: Vec<u32> = numbers
        .windows(3)
        .map(|w| w.iter().sum())
        .collect();

    count_increased = 0;
    for pair in windowed_numbers.windows(2) {
        if pair[0] < pair[1] {
            count_increased += 1;
        }
    }

    println!("Day 1 Part 2 result: {}", count_increased);
}
