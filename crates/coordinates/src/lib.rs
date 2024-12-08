use std::ops::{Add, AddAssign};
use std::path::Path;

use utils::read_lines;

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub struct Coordinates(pub i32, pub i32);

pub fn parse_grid<P>(path: P) -> Vec<Vec<u8>>
where
    P: AsRef<Path>,
{
    read_lines(path)
        .map_while(Result::ok)
        .map(String::into_bytes)
        .collect()
}

impl Add for Coordinates {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl AddAssign for Coordinates {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}
