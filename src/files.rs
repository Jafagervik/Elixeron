use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::vec;

fn open_file(path: &Path) -> File {
    let display = path.display();
    return match File::open(path) {
        Ok(file) => file,
        Error(err) => panic!("Could not open file {}", display, err),
    };
}

fn write_file<T>(path: &str, content: Vec<Vec<T>>) {
    let file = File::new(&path);
}
