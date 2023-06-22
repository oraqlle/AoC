use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

fn main() {

    let file = File::open("../day-01-input.txt").expect("Error opening file!");
    let buf = BufReader::new(file);
    let max = buf.lines()
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
    
