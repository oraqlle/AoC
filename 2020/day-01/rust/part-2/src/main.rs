/// \brief AoC 2020 Day 01 Part 1 Solution
///
/// Author: Tyler Swann (oraqlle.net@gmail.com)
///
/// Date: 09/07/2023
///
/// License: Apache-2.0 license
///
/// Copyright (c) 2023 - present
/// \file main.rs
use std::{
    collections::HashSet,
    fs::File,
    io::{prelude::*, BufReader},
};

fn main() {
    let mut inv = HashSet::<isize>::new();
    let goal: isize = 2020;
    let mut offset: usize = 1;
    let result: usize;

    let file = File::open("../../day-01-input.txt").expect("Error opening file!");
    let buf = BufReader::new(file)
        .lines()
        .map(|ln| ln.expect("Error reading line!"))
        .map(|ln| ln.parse::<isize>().expect("Error parsing integer!"))
        .collect::<Vec<_>>();

    'outerloop: loop {
        let start = buf.iter().skip(offset);
        let first = *buf.iter().skip(offset - 1usize).nth(0).unwrap();
        let target: isize = goal - first;

        let inner_result = start.fold(0isize, |acc, n| {
            inv.insert(target - n);

            if inv.contains(&n) {
                n * (target - n)
            } else {
                acc
            }
        });

        if inner_result != 0isize {
            result = (inner_result * first) as usize;
            break 'outerloop;
        };

        offset += 1usize;
    }

    println!("{}", result);
}
