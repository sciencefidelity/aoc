use std::path::Path;

use utils::read_lines;

pub fn red_nosed_reports_part_1<P>(path: P) -> i32
where
    P: AsRef<Path>,
{
    let mut safe_count = 0;
    for line in read_lines(path).map_while(Result::ok) {
        let line: Vec<i32> = line
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        safe_count += i32::from(is_safe(&line));
    }
    safe_count
}

pub fn red_nosed_reports_part_2<P>(path: P) -> i32
where
    P: AsRef<Path>,
{
    let mut safe_count = 0;
    for line in read_lines(path).map_while(Result::ok) {
        let line: Vec<i32> = line
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        if is_safe(&line) {
            safe_count += 1;
        } else {
            for i in 0..line.len() {
                let mut dampened = Vec::new();
                for (j, num) in line.iter().enumerate() {
                    if i != j {
                        dampened.push(*num);
                    }
                }
                if is_safe(&dampened) {
                    safe_count += 1;
                    break;
                }
                println!("{dampened:?}");
            }
        }
    }
    safe_count
}

fn is_safe(line: &[i32]) -> bool {
    if line[0] == line[1] {
        return false;
    }
    let is_increasing = line[1] > line[0];
    let mut last_seen = line[0];
    for num in line.iter().skip(1) {
        if is_increasing && !(*num > last_seen && num - last_seen <= 3) {
            return false;
        }
        if !(is_increasing || last_seen > *num && last_seen - num <= 3) {
            return false;
        }
        last_seen = *num;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_red_nosed_reports_part_1_a() {
        assert_eq!(red_nosed_reports_part_1("data/day_02_a.txt"), 2);
    }

    #[test]
    fn test_red_nosed_reports_part_1_b() {
        assert_eq!(red_nosed_reports_part_1("data/day_02_b.txt"), 334);
    }

    #[test]
    fn test_red_nosed_reports_part_2_a() {
        assert_eq!(red_nosed_reports_part_2("data/day_02_a.txt"), 4);
    }

    #[test]
    fn test_red_nosed_reports_part_2_b() {
        assert_eq!(red_nosed_reports_part_2("data/day_02_b.txt"), 400);
    }
}
