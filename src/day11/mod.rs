use std::collections::HashMap;

type Stones = HashMap<usize, usize>;

fn parse(input: &str) -> Stones {
    let mut stones = Stones::new();
    for stone in input.split_whitespace() {
        let x = stone.parse().unwrap();
        *stones.entry(x).or_default() += 1;
    }
    stones
}

fn rules(stone: usize, mut f: impl FnMut(usize)) {
    let digits = stone.to_string();
    if stone == 0 {
        f(1);
    } else if digits.len() % 2 == 0 {
        let half = digits.len() / 2;
        f(digits[..half].parse().unwrap());
        f(digits[half..].parse().unwrap());
    } else {
        f(stone * 2024);
    }
}

fn blink(stones: &Stones) -> Stones {
    let mut after = Stones::new();
    for (&x, &n) in stones {
        rules(x, |y| *after.entry(y).or_default() += n);
    }
    after
}

fn count(stones: &Stones) -> usize {
    stones.values().sum()
}

pub fn puzzle1(input: &str) -> usize {
    let mut stones = parse(input);
    for _ in 0..25 {
        stones = blink(&stones);
    }
    count(&stones)
}

pub fn puzzle2(input: &str) -> usize {
    let mut stones = parse(input);
    for _ in 0..75 {
        stones = blink(&stones);
    }
    count(&stones)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("example.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_puzzle1_example() {
        assert_eq!(puzzle1(EXAMPLE), 55312);
    }

    #[test]
    fn test_puzzle1_input() {
        assert_eq!(puzzle1(INPUT), 198089);
    }

    #[test]
    fn test_puzzle2_example() {
        assert_eq!(puzzle2(EXAMPLE), 65601038650482);
    }

    #[test]
    fn test_puzzle2_input() {
        assert_eq!(puzzle2(INPUT), 236302670835517);
    }
}
