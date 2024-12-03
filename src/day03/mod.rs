use regex::{Captures, Regex};

fn parse_mul(caps: Captures) -> usize {
    caps[1].parse::<usize>().unwrap() * caps[2].parse::<usize>().unwrap()
}

pub fn puzzle1(input: &str) -> usize {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    re.captures_iter(input).map(parse_mul).sum()
}

pub fn puzzle2(input: &str) -> usize {
    let mut sum = 0;
    let mut enabled = true;
    let re = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();
    for caps in re.captures_iter(input) {
        match &caps[0] {
            "do()" => enabled = true,
            "don't()" => enabled = false,
            _ => {
                if enabled {
                    sum += parse_mul(caps);
                }
            }
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = include_str!("example1.txt");
    const EXAMPLE2: &str = include_str!("example2.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_puzzle1_example1() {
        assert_eq!(puzzle1(EXAMPLE1), 161);
    }

    #[test]
    fn test_puzzle1_input() {
        assert_eq!(puzzle1(INPUT), 174103751);
    }

    #[test]
    fn test_puzzle2_example2() {
        assert_eq!(puzzle2(EXAMPLE2), 48);
    }

    #[test]
    fn test_puzzle2_input() {
        assert_eq!(puzzle2(INPUT), 100411201);
    }
}
