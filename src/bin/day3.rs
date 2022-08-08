use std::fs::File;
use std::io::{self, prelude::*, BufReader};

use bit_vec::BitVec;

fn bits_to_u8(collection: &Vec<u8>) -> u8 {
    collection
        .iter()
        .fold(0, |result, &bit| (result << 1) ^ bit)
}

fn main() -> io::Result<()> {
    let file = File::open("src/bin/day3.txt")?;
    let reader = BufReader::new(file);

    let mut lines = reader.lines();
    println!("{}", lines.nth(1));

    let size: usize = match lines.nth(1) {
        Some(s) => s.unwrap().len(),
        _ => 0,
    };

    let gammas = BitVec::from_elem(size, false);

    let mut epsilon = gammas.clone();
    epsilon.negate();

    let gamma_byte: Vec<u8> = gammas.to_bytes();
    let epsilon_byte: Vec<u8> = epsilon.to_bytes();

    let gamma_rate = bits_to_u8(&gamma_byte);
    let epsilon_rate = bits_to_u8(&epsilon_byte);

    println!("The Answer is: {}", gamma_rate * epsilon_rate);

    Ok(())
}
