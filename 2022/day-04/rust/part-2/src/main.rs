/// \brief AoC 2022 Day 04 Part 2 Solution
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

fn parse_range(s: &str) -> (usize, usize) {
    s.split_once("-")
        .map(|(n1, n2)| (n1.parse::<usize>().unwrap(), n2.parse::<usize>().unwrap()))
        .unwrap()
}

fn main() {
    let file = File::open("../../day-04-input.txt").expect("Error opening file!");
    let buf = BufReader::new(file);
    let sum = buf
        .lines()
        .map(|ln| ln.expect("Error reading line!"))
        .map(|ln| {
            ln.split_once(",")
                .map(|(s1, s2)| (parse_range(s1), parse_range(s2)))
                .unwrap()
        })
        .map(|((a, b), (c, d))| ((a <= c && c <= b) || (c <= a && a <= d)) as usize)
        .sum::<usize>();

    println!("Result: {:?}", sum);
}
