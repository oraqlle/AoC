/// \brief AoC 2022 Day 05 Part 2 Solution
///
/// Author: Tyler Swann (oraqlle.net@gmail.com)
///
/// Date: 09/08/2023
///
/// License: Apache-2.0 license
///
/// Copyright (c) 2023 - present
/// \file main.rs
use std::{
    collections::VecDeque,
    fs::File,
    io::{prelude::*, BufReader},
};

// fn parse_stack(stack_str: Vec<String>, num_stacks: usize) -> Vec<usize> {
//     vec![]
// }

fn main() {
    let file = File::open("../../day-05-input.txt").expect("Error opening file!");
    let buf = BufReader::new(file);
    let lines = buf
        .lines()
        .map(|ln| ln.expect("Error reading line!"))
        .collect::<Vec<_>>();

    let split_idx = lines.iter().position(|ln| ln == "").unwrap();
    let (stack_str, moves) = lines.split_at(split_idx);
    let num_stacks = stack_str
        .last()
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap()
        .parse::<usize>()
        .unwrap_or(0);

    let mut stacks = vec![Vec::<char>::new(); num_stacks];

    stack_str.iter().rev().for_each(|row| {
        row.chars().enumerate().for_each(|(i, chr)| {
            if chr.is_alphabetic() {
                stacks.get_mut(i / 4).and_then(|v| {
                    v.push(chr);
                    Some(v)
                });
            }
        })
    });

    moves
        .iter()
        .skip(1)
        .map(|s| {
            s.split(' ')
                .filter_map(|ss| ss.parse::<usize>().ok())
                .collect::<Vec<_>>()
        })
        .for_each(|mv| {
            let count = *mv.get(0).unwrap();
            let from = *mv.get(1).unwrap() - 1;
            let to = *mv.get(2).unwrap() - 1;
            let mut tmp = VecDeque::<char>::new();

            for _ in 0..count {
                let chr = stacks.get_mut(from).unwrap().pop().unwrap();
                tmp.push_front(chr);
            }

            stacks.get_mut(to).unwrap().append(&mut tmp.into());
        });

    let result = stacks
        .iter()
        .map(|stck| stck.last().unwrap())
        .collect::<String>();

    println!("Result: {}", result);
}
