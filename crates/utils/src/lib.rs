#![allow(
    dead_code,
    clippy::must_use_candidate,
    clippy::unwrap_used,
    clippy::missing_panics_doc
)]
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

/// Read lines from a file and return an iterator
pub fn read_lines<P>(file: P) -> io::Lines<io::BufReader<File>>
where
    P: AsRef<Path>,
{
    let file = File::open(file).expect("failed to open file");
    io::BufReader::new(file).lines()
}

/// Print a grid or bytes
pub fn print_grid(grid: &[Vec<u8>]) {
    for row in grid {
        println!("{}", String::from_utf8(row.clone()).unwrap());
    }
    println!();
}
