use std::collections::HashMap;

use regex::Regex;

fn parse(input: &str) -> (Vec<isize>, Vec<isize>) {
    let re = Regex::new(r"(\d+)\s+(\d+)").unwrap();
    input
        .lines()
        .map(|line| {
            let caps = re.captures(line).unwrap();
            let l: isize = caps[1].parse().unwrap();
            let r: isize = caps[2].parse().unwrap();
            (l, r)
        })
        .unzip()
}

pub fn puzzle1(input: &str) -> isize {
    let (mut left, mut right) = parse(input);
    left.sort();
    right.sort();
    left.into_iter()
        .zip(right)
        .map(|(l, r)| (r - l).abs())
        .sum()
}

pub fn puzzle2(input: &str) -> isize {
    let (left, right) = parse(input);
    let mut frequencies = HashMap::new();
    for x in right {
        *frequencies.entry(x).or_insert(0) += 1;
    }
    left.into_iter()
        .map(|x| x * frequencies.get(&x).unwrap_or(&0))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("example.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_puzzle1_example() {
        assert_eq!(puzzle1(EXAMPLE), 11);
    }

    #[test]
    fn test_puzzle1_input() {
        assert_eq!(puzzle1(INPUT), 3246517);
    }

    #[test]
    fn test_puzzle2_example() {
        assert_eq!(puzzle2(EXAMPLE), 31);
    }

    #[test]
    fn test_puzzle2_input() {
        assert_eq!(puzzle2(INPUT), 29379307);
    }
}
