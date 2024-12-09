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

struct Span {
    start: usize,
    len: usize,
}

pub fn puzzle2(input: &str) -> usize {
    let mut blocks = Vec::<Id>::new();
    let files: Vec<Span> = (0..)
        .zip(input.chars().array_chunks::<2>())
        .map(|(id, [file, free])| {
            let len = digit_usize(file).unwrap();
            let start = blocks.len();
            blocks.extend(repeat_n(id, len));
            if let Some(len) = digit_usize(free) {
                blocks.extend(repeat_n(-1, len));
            }
            Span { start, len }
        })
        .collect();
    for (i, file) in files.into_iter().enumerate().rev() {
        for start in 0..file.start {
            let span = &mut blocks[start..start + file.len];
            if span.iter().all(|&id| id < 0) {
                span.fill(i as Id);
                blocks[file.start..file.start + file.len].fill(-1);
                break;
            }
        }
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

    #[test]
    fn test_puzzle2_example() {
        assert_eq!(puzzle2(EXAMPLE), 2858);
    }

    #[test]
    fn test_puzzle2_input() {
        assert_eq!(puzzle2(INPUT), 6427437134372);
    }
}
