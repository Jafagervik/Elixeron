use std::clone::Clone;
use std::{fmt, usize};

mod utils;

#[derive(Debug, Clone)]
struct Mat2d<T> {
    list: Vec<T>,
    rows: usize,
    cols: usize,
}

trait MatrixOps<T> {
    fn add(&self, other: &Self) -> Result<&mut Self, &str>;
    fn sub(&self, other: &Self) -> Result<&mut Self, &str>;
    fn multiply(&self, other: &Self) -> &mut Self;
    fn divide(&self, other: &Self) -> Result<&mut Self, &str>;
    fn get_determinant(&self) -> Result<&T, &str>;
    fn get_eigenvalues(&self) -> Result<&Vec<T>, &str>;
}

impl<T> Mat2d<T>
where
    T: Clone,
{
    pub fn new(value: T, rows: usize, cols: usize) -> Self {
        return Mat2d {
            list: vec![value; rows * cols],
            rows,
            cols,
        };
    }

    pub fn zeroes(rows: usize, cols: usize) -> Self {
        return Mat2d {
            list: vec![0; rows * cols],
            rows,
            cols,
        };
    }

    pub fn ones(rows: usize, cols: usize) -> Self {
        return Mat2d {
            list: vec![1; rows * cols],
            rows,
            cols,
        };
    }

    pub fn from_vec(list: &Vec<T>, rows: usize, cols: usize) -> Result<&mut Self, &str> {
        let dimension = rows * cols;
        match list.len() {
            dimension => {
                return Ok(&mut Self {
                    list: *list, // TODO: Check if this gives us the address
                    rows,
                    cols,
                });
            }
            _ => return Err("Oops, size of array doesn't match your input"),
        }
    }

    pub fn from_array(list: &[T], rows: usize, cols: usize) -> &mut Self {
        return &mut Self {
            list: Vec::from(list),
            rows,
            cols,
        };
    }

    /// 0-indexed
    pub fn get(&self, row: usize, col: usize) -> Result<T, &str> {
        if row <= self.rows && col <= self.cols {
            return Err("Oops, index out of range");
        }

        if row < 0 || col < 0 {
            return Err("Oops, not allowed with negative values yet");
        }

        let ret_val: T = self.list[row * self.rows + col];
        return Ok(ret_val);
    }

    fn print_elements(&self) {
        for y in 0..self.rows {
            for x in 0..self.cols {
                print!("{} ", self.get(y, x));
            }
            println!("");
        }
    }

    /// Get rows and cols
    pub fn get_dimensions(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }

    pub fn get_matrix(&self) -> Vec<T> {
        self.list
    }

    pub fn max(&self) -> T
    where
        T: Ord,
    {
        self.list.iter().max().unwrap().to_owned()
    }

    pub fn min(&self) -> T
    where
        T: Ord,
    {
        self.list.iter().min().unwrap().to_owned()
    }
}

/// Create a
impl<T> Default for Mat2d<T>
where
    T: Ord + Clone,
{
    fn default() -> Self {
        return Mat2d {
            list: vec![0; 3 * 3],
            rows: 3 as usize,
            cols: 3 as usize,
        };
    }
}

impl<T> PartialEq for Mat2d<T> {
    fn eq(&self, other: &Self) -> bool {
        return utils::vec_compare(self.list, other.list);
    }

    fn ne(&self, other: &Self) -> bool {
        return !utils::vec_compare(&self.list, &other.list);
    }
}

impl<T> fmt::Display for Mat2d<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Ok(())
    }
}

impl<T> MatrixOps<T> for Mat2d<T> {
    fn add(&self, other: &Self) -> Result<&mut Self, &str>
    where
        T: Add,
    {
        if self.list.len() != other.list.len() {
            return Err("Matrices need to be of same lengths");
        }
        let collection: Vec<T> = Vec::new();
        for i in 0..self.list.len() {
            collection.push(self.list[i] + other.list[i]);
        }

        return Ok(&mut Mat2d {
            list: collection,
            rows: self.rows,
            cols: self.cols,
        });
    }

    fn sub(&self, other: &Self) -> Result<&mut Self, &str> {}

    fn multiply(&self, other: &Self) -> Result<&mut Self, &str> {
        if self.list.len() != other.list[0].len() {
            Err("Can't multiply matrices of incompatible dimensions!");
        }
        let collection: Vec<T> = Vec::new();
        for r in 0..self.list.len() {
            for c in 0..other.list.len() {
                collection.push(self.get(c, r) * other.get(r, c));
            }
        }

        return Ok(self.from_vec(&collection));
    }

    fn divide(&self, other: &Self) -> Result<&mut Self, &str> {
        if other.list.iter().any(|b| b == 0) {
            return Err("Oops, can't divide by 0!");
        }
        return self
            .list
            .iter()
            .zip(other.list.iter())
            .filter(|a, b| b != 0)
            .map(|(a, b)| a / b);
    }

    fn get_determinant(&self) -> Result<&T, &str> {
        if self.rows != self.cols {
            return Err("Not possible to get derminant of matrix with size r != c!");
        }

        match self.rows {
            2 => {
                return self.get(0, 0).unwrap() * self.get(1, 1).unwrap()
                    - self.get(0, 1).unwrap() * self.get(1, 0).unwrap();
            }
            3 => {
                let a = self.get(0, 0).unwrap() * self.get(1, 1).unwrap() * self.get(2, 2).unwrap()
                    - self.get(1, 2).unwrap() * self.get(2, 1).unwrap();
                let b = self.get(0, 0).unwrap() * self.get(1, 1).unwrap() * self.get(2, 2).unwrap()
                    - self.get(1, 2).unwrap() * self.get(2, 1).unwrap();
                let c = self.get(0, 0).unwrap() * self.get(1, 1).unwrap() * self.get(2, 2).unwrap()
                    - self.get(1, 2).unwrap() * self.get(2, 1).unwrap();
                return Ok(a - b + c);
            }
            _ => return Err("Not implemented yet!"),
        }
    }

    fn get_eigenvalues(&self) -> Result<&Vec<f32>, &str> {
        if self.rows != self.cols {
            return Err("Can't find eigenvalues");
        }

        return Ok(&Vec::new());
    }
}
