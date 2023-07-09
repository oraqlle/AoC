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
    let goal: isize = 2020;
    let mut inv = HashSet::<isize>::new();

    let file = File::open("../../day-01-input.txt").expect("Error opening file!");
    let result = BufReader::new(file)
        .lines()
        .map(|ln| ln.expect("Error reading line!"))
        .map(|ln| ln.parse::<isize>().expect("Error parsing integer!"))
        .fold(0isize, |acc, n| {
            inv.insert(goal - n);

            if inv.contains(&n) {
                n * (goal - n)
            } else {
                acc
            }
        });

    println!("{}", result);
}
