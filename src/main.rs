use std::ftm;

mod files;
mod matrix;

impl Default for Matrix<T> {
    fn default() -> Self {
        return Matrix {
            list: vec![vec![]],
            size: (3, 2),
        };
    }
}

fn main() {}
