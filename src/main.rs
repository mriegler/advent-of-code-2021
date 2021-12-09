use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

fn main() {
    day_1()
}

fn day_1() {
    let numbers: Vec<u32> = read_lines("./day_1_input.txt").ok().unwrap()
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

    println!("result: {}", count_increased)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
