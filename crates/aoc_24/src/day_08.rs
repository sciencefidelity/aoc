#![allow(
    clippy::many_single_char_names,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap
)]
use std::collections::{HashMap, HashSet};
use std::path::Path;

use coordinates::{parse_grid, Coordinates};

fn resonant_collinearity_part_1<P>(path: P) -> usize
where
    P: AsRef<Path>,
{
    let antennas = parse_grid(path);
    let (m, n) = (antennas.len(), antennas[0].len());
    let antennas_map = map_antennas(&antennas, m * n);
    let mut antinodes_set = HashSet::new();
    for coords in antennas_map.values() {
        for i in 0..coords.len() {
            for j in i + 1..coords.len() {
                let a = create_antinode(coords[i], coords[j]);
                if is_in_grid(m as i32, n as i32, a) {
                    antinodes_set.insert(a);
                }
                let b = create_antinode(coords[j], coords[i]);
                if is_in_grid(m as i32, n as i32, b) {
                    antinodes_set.insert(b);
                }
            }
        }
    }
    antinodes_set.len()
}

fn resonant_collinearity_part_2<P>(path: P) -> usize
where
    P: AsRef<Path>,
{
    let antennas = parse_grid(path);
    let (m, n) = (antennas.len(), antennas[0].len());
    let antennas_map = map_antennas(&antennas, m * n);
    let mut antinodes_set = HashSet::new();
    for coords in antennas_map.values() {
        for i in 0..coords.len() {
            for j in i + 1..coords.len() {
                let (mut a, mut b) = (coords[i], coords[j]);
                antinodes_set.insert(a);
                antinodes_set.insert(b);
                let mut c = create_antinode(b, a);
                if is_in_grid(m as i32, n as i32, c) {
                    antinodes_set.insert(c);
                }
                while is_in_grid(m as i32, n as i32, c) {
                    (a, b) = (b, c);
                    c = create_antinode(b, a);
                    if is_in_grid(m as i32, n as i32, c) {
                        antinodes_set.insert(c);
                    }
                }
                let (mut a, mut b) = (coords[j], coords[i]);
                let mut c = create_antinode(b, a);
                if is_in_grid(m as i32, n as i32, c) {
                    antinodes_set.insert(c);
                }
                while is_in_grid(m as i32, n as i32, c) {
                    (a, b) = (b, c);
                    c = create_antinode(b, a);
                    if is_in_grid(m as i32, n as i32, c) {
                        antinodes_set.insert(c);
                    }
                }
            }
        }
    }
    antinodes_set.len()
}

const fn create_antinode(a: Coordinates, b: Coordinates) -> Coordinates {
    Coordinates(a.0 - b.0 + a.0, a.1 - b.1 + a.1)
}

fn map_antennas(antennas: &[Vec<u8>], cap: usize) -> HashMap<u8, Vec<Coordinates>> {
    let mut antennas_map: HashMap<u8, Vec<Coordinates>> = HashMap::with_capacity(cap);
    for (y, row) in antennas.iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            if *col != b'.' {
                antennas_map
                    .entry(*col)
                    .or_default()
                    .push(Coordinates(x as i32, y as i32));
            }
        }
    }
    antennas_map
}

const fn is_in_grid(m: i32, n: i32, coords: Coordinates) -> bool {
    coords.0 >= 0 && coords.0 < n && coords.1 >= 0 && coords.1 < m
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resonant_collinearity_part_1_a() {
        assert_eq!(resonant_collinearity_part_1("data/day_08_a.txt"), 14);
    }

    #[test]
    fn test_resonant_collinearity_part_1_b() {
        assert_eq!(resonant_collinearity_part_1("data/day_08_b.txt"), 222);
    }

    #[test]
    fn test_resonant_collinearity_part_2_a() {
        assert_eq!(resonant_collinearity_part_2("data/day_08_a.txt"), 34);
    }

    #[test]
    fn test_resonant_collinearity_part_2_b() {
        assert_eq!(resonant_collinearity_part_2("data/day_08_b.txt"), 884);
    }
}
