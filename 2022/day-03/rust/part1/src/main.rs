/// \brief AoC 2022 Day 03 Part 1 Solution
///
/// Author: Tyler Swann (oraqlle.net@gmail.com)
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
    let file = File::open("../../day-03-input.txt").expect("Error opening file!");
    let buf = BufReader::new(file);
    let lines = buf
        .lines()
        .map(|ln| ln.expect("Error reading line!"))
        .map(|ln| {
            let mid = ln.len() / 2;
            (
                ln.get(0..mid).unwrap().to_owned(),
                ln.get(mid..).unwrap().to_owned(),
            )
        })
        .map(|a, b| {})
        .collect::<Vec<_>>();

    for line in lines {
        println!("{:?}", line);
    }
}
