use std::cmp::Ordering;
use crate::util::read_lines;

// Assumes little endian arch
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
        let comp = val > amount_of_lines / 2;
        let is_bigger = comp as usize;
        let res = acc << 1 | is_bigger;
        res
    });
    let mask = (1 << amount_of_bits) - 1;
    let epsilon = !gamma & mask;
    println!("Day 3 Part 1: gamma: {:b}, epsilon: {:b}, result {:?}", gamma, epsilon, gamma.overflowing_mul(epsilon));
    
    // Part 2
    let oxygen_rating = get_oxygen_rating(&str_lines);
    
    let co2_rating = get_co2_rating(&str_lines);
    
    println!("Day 3 Part 2: oxygen: {}, co2: {}, result: {}", oxygen_rating, co2_rating, oxygen_rating * co2_rating);
}

fn get_oxygen_rating(str_lines: &Vec<String>) -> usize {
    let oxygen_rating = get_rating(str_lines, |bit: bool, ord: &Ordering| {
        match ord {
            Ordering::Less if !bit => true,
            Ordering::Equal if bit => true,
            Ordering::Greater if bit => true,
            _ => false
        }
    });
    oxygen_rating
}

fn get_co2_rating(str_lines: &Vec<String>) -> usize {
    let co2_rating = get_rating(&str_lines, |bit: bool, ord: &Ordering| {
        match ord {
            Ordering::Less if bit => true,
            Ordering::Equal if !bit => true,
            Ordering::Greater if !bit => true,
            _ => false
        }
    });
    co2_rating
}

fn get_bit(num: usize, i: usize) -> bool {
    let mask = 1 << i;
    let res = (num & (mask)) != 0;
    res
}

fn get_rating(lines: &Vec<String>, cmp: impl Fn(bool, &Ordering) -> bool) -> usize {
    let mut remaining_lines = lines.clone();
    let amount_of_bits = lines[0].len();
    for i in 0..amount_of_bits {
        let ones = count_ones(&remaining_lines, i);
        let ord = ones.cmp(&(remaining_lines.len() - ones));
        remaining_lines.retain(|line| {
            let num = usize::from_str_radix(line, 2).unwrap();
            let bit = get_bit(num, amount_of_bits - i - 1);
            let res = cmp(bit, &ord);
            res
        });
        if remaining_lines.len() == 1 {
            break;
        }
    }
    
    usize::from_str_radix(&remaining_lines[0], 2).unwrap()
}


fn count_ones(lines: &Vec<String>, idx: usize) -> usize {
    let amount_of_bits = lines[0].len();
    let mut ones: usize = 0;
    for line in lines {
        if get_bit(usize::from_str_radix(line, 2).unwrap(), amount_of_bits - idx - 1) {
            ones += 1;
        }
    }
    
    ones
}

#[test]
fn test_part_2() {
    let str_lines : Vec<String> = vec![
        "00100".to_owned(),
        "11110".to_owned(),
        "10110".to_owned(),
        "10111".to_owned(),
        "10101".to_owned(),
        "01111".to_owned(),
        "00111".to_owned(),
        "11100".to_owned(),
        "10000".to_owned(),
        "11001".to_owned(),
        "00010".to_owned(),
        "01010".to_owned()
    ];

    let actual_o2 = get_oxygen_rating(&str_lines);

    assert_eq!(actual_o2, 23);


    let actual_co2 = get_co2_rating(&str_lines);

    assert_eq!(actual_co2, 10);
    
}
