use std::mem::{size_of, size_of_val};
use std::ops::{BitAnd, Shl};
use crate::util::read_lines;

pub fn part_1() {
    let str_lines: Vec<String> = read_lines("./day_3_input.txt").unwrap()
        .filter_map(|line| line.ok()).collect();
    let amount_of_bits = str_lines[0].len();
    let amount_of_lines = str_lines.len();
    let lines: Vec<usize> = str_lines.iter()
        .filter_map(|line| usize::from_str_radix(line, 2).ok())
        .collect();

    let mut ones: Vec<usize> = vec![0; amount_of_bits];
    for line in lines {
        for i in 0..amount_of_bits  {
            if get_bit(line, i) {
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
}

fn get_bit(num: usize, i: usize) -> bool {
    let mask = 1 << i;
    let res = (num & (mask)) != 0;
    res
}
