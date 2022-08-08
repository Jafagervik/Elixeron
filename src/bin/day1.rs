use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    // One way of doing it
    // let contents = &fs::read("src/bin/day1.txt")?;

    // for item in contents.iter() {
    // println!("Item: {}", &item);
    // }
    let file = File::open("src/bin/day1.txt")?;
    let reader = BufReader::new(file);

    let coll: Vec<i32> = reader
        .lines()
        .filter(|item| item.is_okay())
        .map(|item| item.unwrap())
        .map(|item| item.parse::<i32>())
        .filter(|item| item.is_okay())
        .map(|item| item.unwrap())
        .collect();

    let mut count = 0;
    let prev = &coll[0];
    for curr in coll.iter().skip(1) {
        if prev < curr {
            count += 1;
        }
        prev = curr;
    }

    println!("The answer is {}", count);
    Ok(())
}
