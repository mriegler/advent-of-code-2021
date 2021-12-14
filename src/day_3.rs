use std::cmp::Ordering;
use std::mem::{size_of, size_of_val};
use std::ops::{BitAnd, Shl};
use crate::util::read_lines;

pub fn execute() {
    let str_lines: Vec<String> = read_lines("./day_3_input.txt").unwrap()
        .filter_map(|line| line.ok()).collect();
    let amount_of_bits = str_lines[0].len();
    let amount_of_lines = str_lines.len();
    let lines: Vec<usize> = str_lines.iter()
        .filter_map(|line| usize::from_str_radix(line, 2).ok())
        .collect();

    let mut ones: Vec<usize> = vec![0; amount_of_bits];
    for line in &lines {
        for i in 0..amount_of_bits  {
            if get_bit(*line, i) {
                ones[i] += 1;
            }
        }
    }

    let gamma = ones.iter().rev()
        .fold(0, |acc, &val| {
            let comp = (val > amount_of_lines / 2);
            let is_bigger = (comp as usize);
            let res = acc << 1 | is_bigger;
            res
        });
    let mask = (1 << amount_of_bits) - 1;
    let epsilon = !gamma & mask;
    println!("Day 3 Part 1: gamma: {:b}, epsilon: {:b}, result {:?}", gamma, epsilon, gamma.overflowing_mul(epsilon));

    // Part 2
    let oxygen_rating = get_rating(amount_of_lines, &lines, &ones, |bit: bool, ord: &Ordering| {
        match ord {
            Ordering::Less if !bit => true,
            Ordering::Equal if bit => true,
            Ordering::Greater if bit => true,
            _ => false
        }
    });

    let co2_rating = get_rating(amount_of_lines, &lines, &ones, |bit: bool, ord: &Ordering| {
        match ord {
            Ordering::Less if bit => true,
            Ordering::Equal if !bit => true,
            Ordering::Greater if !bit => true,
            _ => false
        }
    });

    println!("Day 3 Part 2: oxygen: {}, co2: {}, result: {}", oxygen_rating, co2_rating, oxygen_rating * co2_rating);
}

fn get_bit(num: usize, i: usize) -> bool {
    let mask = 1 << i;
    let res = (num & (mask)) != 0;
    res
}

fn get_rating(total_amount_lines: usize, lines: &Vec<usize>, ones:&Vec<usize>, cmp: impl Fn(bool, &Ordering) -> bool) -> usize {
    let mut remaining_lines = lines.clone();
    for (i, one_val) in ones.iter().rev().enumerate() {
        let ord = one_val.cmp(&(total_amount_lines / 2));
        println!("Ord: {:?}, index: {}", ord, i);
        remaining_lines.retain(|line| {
            let bit = get_bit(*line, ones.len() - i - 1);
            let res = cmp(bit, &ord);
            println!("Num: {:#014b}, bit: {}, res: {}", line, bit, res);
            res
        });
        println!("remaining {}", remaining_lines.len());
        if remaining_lines.len() == 1 {
            break;
        }
    }

    remaining_lines[0]
}
