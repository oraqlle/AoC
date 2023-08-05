/// \brief AoC 2022 Day 05 Part 1 Solution
///
/// Author: Tyler Swann (oraqlle.net@gmail.com)
///
/// Date: 24/07/2023
///
/// License: Apache-2.0 license
///
/// Copyright (c) 2023 - present
/// \file main.rs
use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

fn parse_stack(stack_str: Vec<String>, num_stacks: usize) -> Vec<usize> {
    vec![]
}

fn main() {
    let file = File::open("../../day-05-input.txt").expect("Error opening file!");
    let buf = BufReader::new(file);
    let lines = buf
        .lines()
        .map(|ln| ln.expect("Error reading line!"))
        .collect::<Vec<_>>();

    let split_idx = lines.iter().position(|ln| ln == "").unwrap();        
    let (stack_str, moves) = lines.split_at(split_idx);
    let num_stacks = stack_str.last().unwrap().split_whitespace().collect::<Vec<_>>();
    
    println!("{:?}", &num_stacks);

    //let stack = parse_stack(stack_str, );

    println!("Result: {:?}", stack_str);
    println!("Result: {:?}", moves);
}
