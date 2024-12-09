use std::iter::repeat_n;

fn digit_usize(c: char) -> Option<usize> {
    c.to_digit(10)?.try_into().ok()
}

type Id = i16;

fn checksum(blocks: &[Id]) -> usize {
    blocks
        .iter()
        .enumerate()
        .map(|(i, &id)| match usize::try_from(id) {
            Ok(x) => i * x,
            Err(_) => 0,
        })
        .sum()
}

pub fn puzzle1(input: &str) -> usize {
    let mut blocks = Vec::<Id>::new();
    for (id, [file, free]) in (0..).zip(input.chars().array_chunks::<2>()) {
        blocks.extend(repeat_n(id, digit_usize(file).unwrap()));
        if let Some(count) = digit_usize(free) {
            blocks.extend(repeat_n(-1, count));
        }
    }
    let mut left = 0;
    let mut right = blocks.len() - 1;
    while let Some(delta) = blocks[left..].iter().position(|&id| id < 0) {
        left += delta;
        if right <= left {
            break;
        }
        blocks[left] = blocks[right];
        blocks[right] = -1;
        right -= 1;
    }
    checksum(&blocks)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("example.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_puzzle1_example() {
        assert_eq!(puzzle1(EXAMPLE), 1928);
    }

    #[test]
    fn test_puzzle1_input() {
        assert_eq!(puzzle1(INPUT), 6398608069280);
    }
}
