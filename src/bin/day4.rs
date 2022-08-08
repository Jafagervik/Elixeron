use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::path::Iter;

#[derive(Debug, Clone)]
struct Board {
    board: Vec<Vec<u16>>,
    won: bool,
}

fn generate_winning_numbers(reader: &BufReader<File>) -> Vec<i16> {
    reader
        .lines()
        .nth(0)
        .iter()
        .filter_map(|item| match item.ok() {
            Some(_item) => _item.parse::<i16>().unwrap(),
            None => 0,

        })
        .collect()
}

fn parse_boards(boards: &BufReader<File>) -> Vec<Board> {}

fn

fn main() -> io::Result<()> {
    let file = File::open("src/bin/day3.txt")?;
    let reader = BufReader::new(file);

    let winning_numbers = generate_winning_numbers(&reader);

    let boards: Vec<Board> = parse_boards(&reader);

    let winner_score = 0;

    println!("Winner score is {}", winner_score);
    Ok(())
}
