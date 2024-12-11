pub fn puzzle1(input: &str) -> usize {
    let mut stones: Vec<usize> = input
        .split_whitespace()
        .map(|stone| stone.parse().unwrap())
        .collect();
    for _ in 0..25 {
        stones = stones
            .into_iter()
            .flat_map(|stone| {
                if stone == 0 {
                    return vec![1];
                }
                let digits = stone.to_string();
                if digits.len() % 2 == 0 {
                    let half = digits.len() / 2;
                    vec![
                        digits[..half].parse().unwrap(),
                        digits[half..].parse().unwrap(),
                    ]
                } else {
                    vec![stone * 2024]
                }
            })
            .collect();
    }
    stones.len()
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
}
