use std::{collections::HashMap, path::Path};

use utils::read_lines;

fn print_queue<P>(path: P) -> (usize, usize)
where
    P: AsRef<Path>,
{
    let (rules, mut updates) = parse_file(path);
    let (mut result_a, mut result_b) = (0, 0);
    for row in &mut updates {
        let mut is_correct = true;
        for i in 0..row.len() {
            for j in i + 1..row.len() {
                if let Some(v) = rules.get(&row[i]) {
                    if !v.contains(&row[j]) {
                        is_correct = false;
                        break;
                    }
                } else {
                    is_correct = false;
                    break;
                }
            }
        }
        if is_correct {
            result_a += row[row.len() / 2];
        } else {
            row.sort_unstable_by(|a, b| {
                if rules.get(a).map_or(false, |values| values.contains(b)) {
                    std::cmp::Ordering::Less
                } else if rules.get(b).map_or(false, |values| values.contains(a)) {
                    std::cmp::Ordering::Greater
                } else {
                    std::cmp::Ordering::Equal
                }
            });
            result_b += row[row.len() / 2];
        }
    }
    (result_a, result_b)
}

fn parse_file<P>(path: P) -> (HashMap<usize, Vec<usize>>, Vec<Vec<usize>>)
where
    P: AsRef<Path>,
{
    let (mut rules, mut updates) = (HashMap::new(), Vec::new());
    let mut is_first_section = true;
    for line in read_lines(path).map_while(Result::ok) {
        if line.is_empty() {
            is_first_section = false;
            continue;
        }
        if is_first_section {
            let mut iter = line.split('|');
            let a = iter.next().unwrap().parse::<usize>().unwrap();
            let b = iter.next().unwrap().parse::<usize>().unwrap();
            rules.entry(a).or_insert(Vec::new()).push(b);
        } else {
            updates.push(
                line.split(',')
                    .map(|n| n.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>(),
            );
        }
    }
    (rules, updates)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_queue_a() {
        assert_eq!(print_queue("data/day_05_a.txt"), (143, 123));
    }

    #[test]
    fn test_print_queue_b() {
        assert_eq!(print_queue("data/day_05_b.txt"), (4662, 5900));
    }
}
