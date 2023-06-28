#![feature(iter_array_chunks)]

/// \brief AoC 2022 Day 03 Part 2 Solution
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

fn to_num(chr: char) -> usize {
    match chr {
        'a'..='z' => (chr as usize) - 96usize,
        'A'..='Z' => (chr as usize) - 38usize,
        _ => 0,
    }
}

fn main() {
    let file = File::open("../../day-03-input.txt").expect("Error opening file!");
    let buf = BufReader::new(file);
    let sum = buf
        .lines()
        .map(|ln| ln.expect("Error reading line!"))
        .array_chunks::<3>()
        .map(|chunk| match chunk {
            [a, b, c] => a
                .chars()
                .find(|chr| b.contains(*chr) && c.contains(*chr))
                .unwrap_or(0 as char),
        })
        .map(|c| to_num(c))
        .sum::<usize>();

    println!("Result: {}", sum);
}
