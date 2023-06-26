use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

fn play(mv: (usize, usize)) -> Result<usize, &'static str> {
    match mv {
        (1, 3) => Ok(mv.1),
        (3, 1) => Ok(mv.1 + 6usize),
        (op, me) => {
            if me < op {
                Ok(me)
            } else if me > op {
                Ok(me + 6usize)
            } else if me == op {
                Ok(me + 3usize)
            } else {
                Err("Invalid Round!")
            }
        }
    }
}

fn main() {
    let file = File::open("../../../../day-02-input.txt").expect("Error opening file!");
    let buf = BufReader::new(file);
    let score = buf
        .lines()
        .map(|ln| ln.expect("Error reading line!"))
        .map(|ln| {
            (
                ln.chars().nth(0).to_owned().unwrap(),
                ln.chars().nth(2).to_owned().unwrap(),
            )
        })
        .map(|(op, me)| (((op as usize) - 64usize), ((me as usize) - 87usize)))
        .map(|mv| play(mv).expect("Invalid Round!"))
        .sum::<usize>();

    println!("My final score is: {}", score);
}
