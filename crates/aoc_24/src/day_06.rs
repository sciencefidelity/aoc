#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss
)]
use std::ops::{Add, AddAssign};
use std::{collections::HashSet, path::Path};

use utils::read_lines;

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
struct Coordinates(i32, i32);

const DIRECTIONS: &[Coordinates] = &[
    Coordinates(0, -1),
    Coordinates(1, 0),
    Coordinates(0, 1),
    Coordinates(-1, 0),
];

fn guard_gallivant_part_1<P>(path: P) -> usize
where
    P: AsRef<Path>,
{
    let mut grid = parse_input(path);
    let guard_pos = find_guard_position(&grid);
    simulate_a(&mut grid, guard_pos);
    count_moves(&grid)
}

fn guard_gallivant_part_2<P>(path: P) -> usize
where
    P: AsRef<Path>,
{
    let mut grid = parse_input(path);
    let mut route = grid.clone();
    let guard_pos = find_guard_position(&grid);
    simulate_a(&mut route, guard_pos);
    let mut count = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] != b'.' || route[i][j] != b'X' {
                continue;
            }
            grid[i][j] = b'#';
            let mut visited = HashSet::new();
            simulate_b(&grid, guard_pos, &mut visited, &mut count);
            grid[i][j] = b'.';
        }
    }
    count
}

fn simulate_a(grid: &mut [Vec<u8>], mut guard_pos: Coordinates) {
    let (m, n) = (grid.len() as i32, grid[0].len() as i32);
    let mut direction = DIRECTIONS.iter().cycle();
    let mut current_direction = direction.next().unwrap();
    while !is_leaving_map(guard_pos, m, n) {
        grid[guard_pos.1 as usize][guard_pos.0 as usize] = b'X';
        loop {
            let next_pos = guard_pos + *current_direction;
            if !is_leaving_map(next_pos, m, n)
                && grid[next_pos.1 as usize][next_pos.0 as usize] == b'#'
            {
                current_direction = direction.next().unwrap();
            } else {
                guard_pos = next_pos;
                break;
            }
        }
    }
}

fn simulate_b(
    grid: &[Vec<u8>],
    mut guard_pos: Coordinates,
    visited: &mut HashSet<(i32, i32, usize)>,
    count: &mut usize,
) {
    let (m, n) = (grid.len() as i32, grid[0].len() as i32);
    let mut direction = DIRECTIONS.iter().enumerate().cycle();
    let mut current_direction = direction.next().unwrap();
    while !is_leaving_map(guard_pos, m, n)
        && !visited.contains(&(guard_pos.0, guard_pos.1, current_direction.0))
    {
        visited.insert((guard_pos.0, guard_pos.1, current_direction.0));
        loop {
            let next_pos = guard_pos + *current_direction.1;
            if !is_leaving_map(next_pos, m, n)
                && grid[next_pos.1 as usize][next_pos.0 as usize] == b'#'
            {
                current_direction = direction.next().unwrap();
            } else {
                guard_pos = next_pos;
                break;
            }
        }
    }
    if visited.contains(&(guard_pos.0, guard_pos.1, current_direction.0)) {
        *count += 1;
    }
}

const fn is_leaving_map(guard_pos: Coordinates, m: i32, n: i32) -> bool {
    guard_pos.0 < 0 || guard_pos.0 >= n || guard_pos.1 < 0 || guard_pos.1 >= m
}

fn count_moves(grid: &[Vec<u8>]) -> usize {
    let mut count = 0;
    for row in grid {
        for col in row {
            if *col == b'X' {
                count += 1;
            }
        }
    }
    count
}

fn find_guard_position(grid: &[Vec<u8>]) -> Coordinates {
    let mut guard_pos = Coordinates(0, 0);
    for (y, row) in grid.iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            if *col == b'^' {
                guard_pos = Coordinates(x as i32, y as i32);
                break;
            }
        }
    }
    guard_pos
}

fn parse_input<P>(path: P) -> Vec<Vec<u8>>
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_guard_gallivant_part_1_a() {
        assert_eq!(guard_gallivant_part_1("data/day_06_a.txt"), 41);
    }

    #[test]
    fn test_guard_gallivant_part_1_b() {
        assert_eq!(guard_gallivant_part_1("data/day_06_b.txt"), 5030);
    }

    #[test]
    fn test_guard_gallivant_part_2_a() {
        assert_eq!(guard_gallivant_part_2("data/day_06_a.txt"), 6);
    }

    #[test]
    fn test_guard_gallivant_part_2_b() {
        assert_eq!(guard_gallivant_part_2("data/day_06_b.txt"), 1928);
    }
}
