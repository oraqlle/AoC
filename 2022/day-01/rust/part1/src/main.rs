/// \brief AoC 2022 Day 01 Part 1 Solution
///
/// Author: Tyler Swann (oraqlle.net@gmail.com)
///
/// Date: 20/06/2023
///
/// License: Apache-2.0 license
///
/// Copyright (c) 2023 - present
/// \file main.rs
use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

fn main() {
    let file = File::open("../../day-01-input.txt").expect("Error opening file!");
    let buf = BufReader::new(file);
    let max = buf
        .lines()
        .map(|ln| ln.expect("Error reading line!"))
        .collect::<Vec<String>>()
        .split(|ln| ln == "")
        .map(|rng| {
            rng.iter()
                .map(|ln| ln.parse::<usize>().unwrap())
                .sum::<usize>()
        })
        .max();

    println!("{}", max.unwrap());
}
