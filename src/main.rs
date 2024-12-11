use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;

struct Matrix {
    matrix: Vec<Vec<char>>,
}

impl fmt::Debug for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let height = self.matrix.len();
        let width = self.matrix[0].len();
        for i in 0..height {
            for j in 0..width {
                let val = self.matrix[i][j];
                write!(f, "{val} ");
            }
            writeln!(f);
        }
        Ok(())
    }
}

trait Get {
    fn get(&self, i: i32, j: i32) -> Option<&char>;
}

impl Get for Matrix {
    fn get(&self, i: i32, j: i32) -> Option<&char> {
        if i < 0 || j < 0 {
            return None;
        }
        let mut x: Option<&char> = None;
        let row = self.matrix.get(i as usize);
        if row.is_some() {
            x = row.unwrap().get(j as usize);
        }
        return x;
    }
}

trait CountXMAS {
    fn count_at_pos(&self, i: usize, j: usize) -> u64;
}

impl CountXMAS for Matrix {
    fn count_at_pos( &self, i: usize, j: usize) -> u64 {

        fn check_with_delta( matrix : &Matrix, i: i32, j: i32, deltai: i32, deltaj: i32 ) -> bool {
            if let Some(c) = matrix.get(i+deltai, j-deltaj) {
                if *c == 'M' {
                    if let Some(c) = matrix.get(i-deltai, j+deltaj) {
                        if *c == 'S' {
                            return true;
                        }
                    }
                }
            }
            return false;
        }


        let mut count: u64 = 0;

        let left = check_with_delta(&self, i as i32,j as i32, 1, 1) || check_with_delta(&self, i as i32,j as i32, -1, -1);

        let right = check_with_delta(&self, i as i32,j as i32, 1, -1) || check_with_delta(&self, i as i32,j as i32, -1, 1);

        if left && right {
            count += 1;
        }

        return count;
    }
}

fn main() {
    println!("Hello, aoc_2024_4!");

    if let Ok(lines) = read_lines("./src/input.txt") {

        let mut input_matrix = Matrix {
            matrix : Vec::new(),
        };

        // Consumes the iterator, returns an ( Optional) String
        for line in lines.flatten() {
            let characters:Vec<char> = line.chars().collect();
            input_matrix.matrix.push(characters);
        }

        let height = input_matrix.matrix.len();
        println!("height {height}");

        // We assume every row has the same number of columns.
        let width = input_matrix.matrix[0].len();
        println!("array width {width}");

        println!("input_matrix is:\n{input_matrix:?}");

        let mut count_found: u64 = 0;

        for i in 0..height {
            for j in 0..width {
                if let Some(val) = input_matrix.get(i as i32,j as i32) {
                    if *val == 'A' {
                        count_found += input_matrix.count_at_pos(i,j);
                    }
                }
            }
        }

        println!("count_found {count_found}");

    } else {
        if let Ok(path) = env::current_dir() {
            println!("Error reading lines, the current directory is {}", path.display());
        } else {
            println!("Error reading lines, and can't print the current directory");

        }
    }
}

// Thanks to https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}