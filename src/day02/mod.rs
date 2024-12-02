use std::cmp::Ordering;

fn parse(line: &str) -> impl Iterator<Item = isize> + '_ {
    line.split_whitespace().map(|level| level.parse().unwrap())
}

fn safe(levels: impl IntoIterator<Item = isize>) -> bool {
    let mut increasing = false;
    let mut decreasing = false;
    let mut previous: Option<isize> = None;
    for curr in levels {
        if let Some(prev) = previous {
            let diff = curr - prev;
            match diff.cmp(&0) {
                Ordering::Less => decreasing = true,
                Ordering::Equal => return false,
                Ordering::Greater => increasing = true,
            }
            if 3 < diff.abs() {
                return false;
            }
        }
        previous = Some(curr);
    }
    increasing != decreasing
}

pub fn puzzle1(input: &str) -> usize {
    input.lines().filter(|line| safe(parse(line))).count()
}

pub fn puzzle2(input: &str) -> usize {
    input
        .lines()
        .filter(|line| {
            let levels: Vec<isize> = parse(line).collect();
            levels.iter().enumerate().any(|(i, _)| {
                safe(levels.iter().enumerate().filter_map(
                    |(j, &level)| {
                        if i == j {
                            None
                        } else {
                            Some(level)
                        }
                    },
                ))
            })
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("example.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_puzzle1_example() {
        assert_eq!(puzzle1(EXAMPLE), 2);
    }

    #[test]
    fn test_puzzle1_input() {
        assert_eq!(puzzle1(INPUT), 421);
    }

    #[test]
    fn test_puzzle2_example() {
        assert_eq!(puzzle2(EXAMPLE), 4);
    }

    #[test]
    fn test_puzzle2_input() {
        assert_eq!(puzzle2(INPUT), 476);
    }
}