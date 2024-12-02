use std::{collections::HashMap, path::Path};

pub use utils::read_lines;

pub fn historian_hysteria_part_1<P>(path: P) -> i32
where
    P: AsRef<Path>,
{
    let mut left = Vec::new();
    let mut right = Vec::new();
    for line in read_lines(path).map_while(Result::ok) {
        let line: Vec<i32> = line
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        left.push(line[0]);
        right.push(line[1]);
    }
    let mut total_distance = 0;
    left.sort_unstable();
    right.sort_unstable();
    for i in 0..left.len() {
        total_distance += (left[i] - right[i]).abs();
    }
    total_distance
}

pub fn historian_hysteria_part_2<P>(path: P) -> i32
where
    P: AsRef<Path>,
{
    let mut left = Vec::new();
    let mut right = HashMap::new();
    for line in read_lines(path).map_while(Result::ok) {
        let line: Vec<i32> = line
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        left.push(line[0]);
        right
            .entry(line[1])
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }
    let mut similarity_score = 0;
    for num in left {
        similarity_score += right.get(&num).unwrap_or(&0) * num;
    }
    similarity_score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_historian_hysteria_part_1_a() {
        assert_eq!(historian_hysteria_part_1("data/day_01_a.txt"), 11);
    }

    #[test]
    fn test_historian_hysteria_part_1_b() {
        assert_eq!(historian_hysteria_part_1("data/day_01_b.txt"), 2_367_773);
    }

    #[test]
    fn test_historian_hysteria_part_2_a() {
        assert_eq!(historian_hysteria_part_2("data/day_01_a.txt"), 31);
    }

    #[test]
    fn test_historian_hysteria_part_2_b() {
        assert_eq!(historian_hysteria_part_2("data/day_01_b.txt"), 21_271_939);
    }
}
