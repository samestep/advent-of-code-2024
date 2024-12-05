use std::{cmp::Ordering, collections::HashSet};

use itertools::Itertools;

fn parse(input: &str) -> (HashSet<(usize, usize)>, &str) {
    let (rules, updates) = input.split("\n\n").collect_tuple().unwrap();
    (
        rules
            .lines()
            .map(|line| {
                line.split('|')
                    .map(|x| x.parse().unwrap())
                    .collect_tuple()
                    .unwrap()
            })
            .collect(),
        updates,
    )
}

fn parse_update(rules: &HashSet<(usize, usize)>, update: &str) -> (Vec<usize>, Vec<usize>) {
    let pages: Vec<usize> = update.split(',').map(|x| x.parse().unwrap()).collect();
    let mut sorted = pages.clone();
    sorted.sort_by(
        |&x, &y| match (rules.contains(&(x, y)), rules.contains(&(y, x))) {
            (true, false) => Ordering::Less,
            (false, true) => Ordering::Greater,
            _ => panic!(),
        },
    );
    (pages, sorted)
}

fn middle(pages: &[usize]) -> usize {
    pages[pages.len() / 2]
}

pub fn puzzle1(input: &str) -> usize {
    let (rules, updates) = parse(input);
    updates
        .lines()
        .filter_map(|line| {
            let (pages, sorted) = parse_update(&rules, line);
            if pages == sorted {
                Some(middle(&pages))
            } else {
                None
            }
        })
        .sum()
}

pub fn puzzle2(input: &str) -> usize {
    let (rules, updates) = parse(input);
    updates
        .lines()
        .filter_map(|line| {
            let (pages, sorted) = parse_update(&rules, line);
            if pages == sorted {
                None
            } else {
                Some(middle(&sorted))
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("example.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_puzzle1_example() {
        assert_eq!(puzzle1(EXAMPLE), 143);
    }

    #[test]
    fn test_puzzle1_input() {
        assert_eq!(puzzle1(INPUT), 5166);
    }

    #[test]
    fn test_puzzle2_example() {
        assert_eq!(puzzle2(EXAMPLE), 123);
    }

    #[test]
    fn test_puzzle2_input() {
        assert_eq!(puzzle2(INPUT), 4679);
    }
}
