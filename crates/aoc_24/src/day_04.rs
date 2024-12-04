#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss
)]
use std::path::Path;

use utils::read_lines;

const DIRECTIONS: &[(i32, i32); 8] = &[
    (-1, -1),
    (0, -1),
    (1, -1),
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
];

fn ceres_search_part_1<P>(path: P) -> i32
where
    P: AsRef<Path>,
{
    let grid = grid_from_lines(path);
    let mut result = 0;
    let pattern = b"XMAS";
    let (m, n) = (grid.len(), grid[0].len());
    for i in 0..m {
        for j in 0..n {
            if grid[i][j] == b'X' {
                for d in DIRECTIONS {
                    let mut count = 1;
                    let (mut row, mut col) = (i as i32, j as i32);
                    for b in pattern.iter().skip(1) {
                        (row, col) = (row + d.0, col + d.1);
                        if row < 0
                            || row >= m as i32
                            || col < 0
                            || col >= n as i32
                            || *b != grid[row as usize][col as usize]
                        {
                            break;
                        }
                        count += 1;
                    }
                    if count == 4 {
                        result += 1;
                    }
                }
            }
        }
    }
    result
}

const CORNERS: &[(i32, i32); 4] = &[(-1, -1), (1, 1), (1, -1), (-1, 1)];

fn ceres_search_part_2<P>(path: P) -> i32
where
    P: AsRef<Path>,
{
    let patterns = Vec::from([
        "SMMS".to_owned(),
        "MSSM".to_owned(),
        "MSMS".to_owned(),
        "SMSM".to_owned(),
    ]);
    let grid = grid_from_lines(path);
    let mut result = 0;
    let (m, n) = (grid.len(), grid[0].len());
    for i in 1..m - 1 {
        for j in 1..n - 1 {
            if grid[i][j] == b'A' {
                let pattern = &mut [0; 4];
                for (k, c) in CORNERS.iter().enumerate() {
                    pattern[k] = grid[(i as i32 + c.0) as usize][(j as i32 + c.1) as usize];
                }
                let pattern = String::from_utf8(pattern.to_vec()).unwrap();
                if patterns.contains(&pattern) {
                    result += 1;
                }
            }
        }
    }
    result
}

fn grid_from_lines<P>(path: P) -> Vec<Vec<u8>>
where
    P: AsRef<Path>,
{
    let mut grid = Vec::new();
    for line in read_lines(path).map_while(Result::ok) {
        grid.push(line.into_bytes());
    }
    grid
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ceres_search_part_1_a() {
        assert_eq!(ceres_search_part_1("data/day_04_a.txt"), 18);
    }

    #[test]
    fn test_ceres_search_part_1_b() {
        assert_eq!(ceres_search_part_1("data/day_04_b.txt"), 2406);
    }

    #[test]
    fn test_ceres_search_part_2_a() {
        assert_eq!(ceres_search_part_2("data/day_04_a.txt"), 9);
    }

    #[test]
    fn test_ceres_search_part_2_b() {
        assert_eq!(ceres_search_part_2("data/day_04_b.txt"), 1807);
    }
}
