use itertools::Itertools;

type Ops<const N: usize> = [fn(usize, usize) -> Option<usize>; N];

fn sat<const N: usize>(ops: Ops<N>, test: usize, numbers: &[usize]) -> bool {
    let (&x, rest) = numbers.split_last().unwrap();
    if rest.is_empty() {
        x == test
    } else {
        ops.into_iter().any(|f| match f(test, x) {
            Some(y) => sat(ops, y, rest),
            None => false,
        })
    }
}

fn solve<const N: usize>(ops: Ops<N>, input: &str) -> usize {
    input
        .lines()
        .filter_map(|line| {
            let (first, rest) = line.split(": ").collect_tuple().unwrap();
            let test: usize = first.parse().unwrap();
            let numbers: Vec<usize> = rest.split(' ').map(|x| x.parse().unwrap()).collect();
            if sat(ops, test, &numbers) {
                Some(test)
            } else {
                None
            }
        })
        .sum()
}

fn add(test: usize, last: usize) -> Option<usize> {
    test.checked_sub(last)
}

fn mul(test: usize, last: usize) -> Option<usize> {
    if last == 0 || test % last != 0 {
        None
    } else {
        Some(test / last)
    }
}

pub fn puzzle1(input: &str) -> usize {
    solve([add, mul], input)
}

fn cat(test: usize, last: usize) -> Option<usize> {
    test.to_string()
        .strip_suffix(&last.to_string())?
        .parse()
        .ok()
}

pub fn puzzle2(input: &str) -> usize {
    solve([add, mul, cat], input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("example.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_puzzle1_example() {
        assert_eq!(puzzle1(EXAMPLE), 3749);
    }

    #[test]
    fn test_puzzle1_input() {
        assert_eq!(puzzle1(INPUT), 4122618559853);
    }

    #[test]
    fn test_puzzle2_example() {
        assert_eq!(puzzle2(EXAMPLE), 11387);
    }

    #[test]
    fn test_puzzle2_input() {
        assert_eq!(puzzle2(INPUT), 227615740238334);
    }
}
