use std::collections::{HashMap, HashSet};

use itertools::Itertools;

struct Grid<'a> {
    string: &'a str,
    width: usize,
}

impl<'a> Grid<'a> {
    fn new(string: &'a str) -> Self {
        let width = string.find('\n').unwrap();
        Self { string, width }
    }

    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.string.len() / (self.width() + 1)
    }

    fn get(&self, x: isize, y: isize) -> Option<&str> {
        let i: usize = y.try_into().ok()?;
        let j: usize = x.try_into().ok()?;
        if j >= self.width() {
            return None;
        }
        let k = i * (self.width() + 1) + j;
        self.string.get(k..k + 1)
    }
}

fn singleton(s: &str) -> char {
    let (c,) = s.chars().collect_tuple().unwrap();
    c
}

type Frequencies = HashMap<char, Vec<(isize, isize)>>;

fn gather(input: &str) -> (Grid, Frequencies) {
    let grid = Grid::new(input);
    let mut frequencies: HashMap<char, Vec<(isize, isize)>> = HashMap::new();
    for y in 0..(grid.height() as isize) {
        for x in 0..(grid.width() as isize) {
            let freq = singleton(grid.get(x, y).unwrap());
            if freq != '.' {
                frequencies.entry(freq).or_default().push((x, y));
            }
        }
    }
    (grid, frequencies)
}

pub fn puzzle1(input: &str) -> usize {
    let (grid, frequencies) = gather(input);
    let mut antinodes = HashSet::new();
    for (_, coords) in frequencies {
        for (i, &(x0, y0)) in coords.iter().enumerate() {
            for (j, &(x1, y1)) in coords.iter().enumerate() {
                if i != j {
                    let dx = x1 - x0;
                    let dy = y1 - y0;
                    for (x, y) in [(x0 - dx, y0 - dy), (x1 + dx, y1 + dy)] {
                        if grid.get(x, y).is_some() {
                            antinodes.insert((x, y));
                        }
                    }
                }
            }
        }
    }
    antinodes.len()
}

pub fn puzzle2(input: &str) -> usize {
    let (grid, frequencies) = gather(input);
    let mut antinodes = HashSet::new();
    for (_, coords) in frequencies {
        for (i, &(x0, y0)) in coords.iter().enumerate() {
            for (j, &(x1, y1)) in coords.iter().enumerate() {
                if i != j {
                    let dx = x1 - x0;
                    let dy = y1 - y0;
                    for k in 0.. {
                        let x = x0 - k * dx;
                        let y = y0 - k * dy;
                        if grid.get(x, y).is_some() {
                            antinodes.insert((x, y));
                        } else {
                            break;
                        }
                    }
                    for k in 0.. {
                        let x = x1 + k * dx;
                        let y = y1 + k * dy;
                        if grid.get(x, y).is_some() {
                            antinodes.insert((x, y));
                        } else {
                            break;
                        }
                    }
                }
            }
        }
    }
    antinodes.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("example.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_puzzle1_example() {
        assert_eq!(puzzle1(EXAMPLE), 14);
    }

    #[test]
    fn test_puzzle1_input() {
        assert_eq!(puzzle1(INPUT), 409);
    }

    #[test]
    fn test_puzzle2_example() {
        assert_eq!(puzzle2(EXAMPLE), 34);
    }

    #[test]
    fn test_puzzle2_input() {
        assert_eq!(puzzle2(INPUT), 1308);
    }
}
