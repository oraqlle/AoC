/// \brief AoC 2022 Day 04 Part 1 Solution
///
/// Author: Tyler Swann (tyler.swann05@gmail.com)
///
/// Date: 28/06/2023
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
    let file = File::open("../../day-04-input.txt").expect("Error opening file!");
    let buf = BufReader::new(file);
    let sum = buf
        .lines()
        .map(|ln| ln.expect("Error reading line!"))
        .collect::<Vec<_>>();

    println!("Result: {:?}", sum);
}
