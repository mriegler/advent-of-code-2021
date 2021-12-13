use std::io::{Error, ErrorKind};
use std::num::ParseIntError;
use std::str::FromStr;
use crate::util;

pub enum SubMovement {
    Forward(usize),
    Backward(usize),
    Down(usize),
    Up(usize),
}

impl FromStr for SubMovement {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (command, num_str) = s.split_once(" ").ok_or(Error::new(ErrorKind::Other, ""))?;
        let num = num_str.parse::<usize>().map_err(|_| Error::new(ErrorKind::Other, ""))?;
        match command {
            "forward" => Ok(SubMovement::Forward(num)),
            "backward" => Ok(SubMovement::Backward(num)),
            "down" => Ok(SubMovement::Down(num)),
            "up" => Ok(SubMovement::Up(num)),
            _ => Err(Error::new(ErrorKind::Other, ""))
        }
    }
}

pub fn day_2() {
    let movements : Vec<SubMovement> = util::read_lines("./day_2_input.txt").ok().unwrap()
        .filter_map(|line| line.ok())
        .filter_map(|line| line.parse::<SubMovement>().ok())
        .collect();
    let mut coords: Vec<usize> = movements
        .iter()
        .fold(vec![0, 0], |mut coords, movement| {
            match movement {
                SubMovement::Forward(num) => coords[0] += num,
                SubMovement::Backward(num) => coords[0] -= num,
                SubMovement::Down(num) => coords[1] += num,
                SubMovement::Up(num) => coords[1] -= num
            }
            coords
        });

    println!("Day 2 Part 1: {:?}", coords[0] * coords[1]);

    // Part 2
    coords = movements
        .iter()
        .fold(vec![0, 0, 0], |mut coords, movement| {
        match movement {
            SubMovement::Forward(num) => {
                coords[0] += num;
                coords[1] += num * coords[2]
            }
            SubMovement::Backward(num) => {
                coords[0] -= num;
                coords[1] -= num * coords[2]
            }
            SubMovement::Down(num) => {
                coords[2] += num;
            }
            SubMovement::Up(num) => {
                coords[2] -= num;
            }
        }
        coords
    });

    println!("Day 2 Part 2: {:?}", coords[0] * coords[1]);
}
