use std::path::Path;

use utils::read_lines;

#[derive(Debug, Clone, Copy)]
enum Ops {
    Add,
    Mul,
    Concat,
}

const ELEPHANTS_A: &[Ops; 2] = &[Ops::Add, Ops::Mul];
const ELEPHANTS_B: &[Ops; 3] = &[Ops::Add, Ops::Mul, Ops::Concat];

fn bridge_repair<P>(path: P, operators: &[Ops]) -> u64
where
    P: AsRef<Path>,
{
    let equations = parse_data(path);
    let mut result = 0;
    for equation in &equations {
        let combinations = generate_combinations(equation.1.len() - 1, operators);
        if check(equation.0, &equation.1, &combinations) {
            result += equation.0;
        }
    }
    result
}

fn generate_combinations(n: usize, operators: &[Ops]) -> Vec<Vec<Ops>> {
    if n == 0 {
        return vec![vec![]];
    }
    let smaller_combinations = generate_combinations(n - 1, operators);
    let mut result = Vec::new();
    for combination in smaller_combinations {
        for &op in operators {
            let mut new_combination = combination.clone();
            new_combination.push(op);
            result.push(new_combination);
        }
    }
    result
}

fn check(value: u64, nums: &[u64], combinations: &[Vec<Ops>]) -> bool {
    for combination in combinations {
        let test = calculate(nums, combination);
        if test == value {
            return true;
        }
    }
    false
}

fn calculate(nums: &[u64], operators: &[Ops]) -> u64 {
    let mut result = nums[0];
    for (i, op) in operators.iter().enumerate() {
        match op {
            Ops::Add => result += nums[i + 1],
            Ops::Mul => result *= nums[i + 1],
            Ops::Concat => result = concat(result, nums[i + 1]),
        }
    }
    result
}

const fn concat(a: u64, b: u64) -> u64 {
    a * 10u64.pow(b.ilog(10) + 1) + b
}

fn parse_data<P>(path: P) -> Vec<(u64, Vec<u64>)>
where
    P: AsRef<Path>,
{
    let mut equations = Vec::new();
    for line in read_lines(path).map_while(Result::ok) {
        let mut iter = line.split(':');
        let value: u64 = iter.next().unwrap().parse().unwrap();
        let numbers: Vec<u64> = iter
            .next()
            .unwrap()
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        equations.push((value, numbers));
    }
    equations
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bridge_repair_part_1_a() {
        assert_eq!(bridge_repair("data/day_07_a.txt", ELEPHANTS_A), 3749);
    }

    #[test]
    fn test_bridge_repair_part_1_b() {
        assert_eq!(
            bridge_repair("data/day_07_b.txt", ELEPHANTS_A),
            1_038_838_357_795
        );
    }

    #[test]
    fn test_bridge_repair_part_2_a() {
        assert_eq!(bridge_repair("data/day_07_a.txt", ELEPHANTS_B), 11_387);
    }

    #[test]
    fn test_bridge_repair_part_2_b() {
        assert_eq!(
            bridge_repair("data/day_07_b.txt", ELEPHANTS_B),
            254_136_560_217_241
        );
    }
}
