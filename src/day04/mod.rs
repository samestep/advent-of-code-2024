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
        self.string.len() / self.width
    }

    fn get(&self, x: isize, y: isize) -> Option<&str> {
        let i: usize = y.try_into().ok()?;
        let j: usize = x.try_into().ok()?;
        if j >= self.width {
            return None;
        }
        let k = i * (self.width + 1) + j;
        self.string.get(k..k + 1)
    }

    fn gets<const N: usize>(&self, points: [(isize, isize); N]) -> Option<[&str; N]> {
        let mut strings = [""; N];
        for (i, (x, y)) in points.into_iter().enumerate() {
            strings[i] = self.get(x, y)?;
        }
        Some(strings)
    }
}

pub fn puzzle1(input: &str) -> usize {
    let mut count = 0;
    let grid = Grid::new(input);
    for y in 0..(grid.height() as isize) {
        for x in 0..(grid.width() as isize) {
            for points in [
                [(x, y), (x + 1, y), (x + 2, y), (x + 3, y)],
                [(x, y), (x + 1, y - 1), (x + 2, y - 2), (x + 3, y - 3)],
                [(x, y), (x, y - 1), (x, y - 2), (x, y - 3)],
                [(x, y), (x - 1, y - 1), (x - 2, y - 2), (x - 3, y - 3)],
                [(x, y), (x - 1, y), (x - 2, y), (x - 3, y)],
                [(x, y), (x - 1, y + 1), (x - 2, y + 2), (x - 3, y + 3)],
                [(x, y), (x, y + 1), (x, y + 2), (x, y + 3)],
                [(x, y), (x + 1, y + 1), (x + 2, y + 2), (x + 3, y + 3)],
            ] {
                if let Some(["X", "M", "A", "S"]) = grid.gets(points) {
                    count += 1;
                }
            }
        }
    }
    count
}

fn add((x1, y1): (isize, isize), (x2, y2): (isize, isize)) -> (isize, isize) {
    (x1 + x2, y1 + y2)
}

fn rotate((x, y): (isize, isize)) -> (isize, isize) {
    (y, -x)
}

fn rotations<const N: usize>(
    center: (isize, isize),
    mut points: [(isize, isize); N],
) -> [[(isize, isize); N]; 4] {
    let mut lists = [points; 4];
    for list in lists.iter_mut() {
        for j in 0..N {
            list[j] = add(center, points[j]);
            points[j] = rotate(points[j]);
        }
    }
    lists
}

pub fn puzzle2(input: &str) -> usize {
    let mut count = 0;
    let grid = Grid::new(input);
    for y in 0..(grid.height() as isize) {
        for x in 0..(grid.width() as isize) {
            for points in rotations((x, y), [(-1, -1), (1, -1), (0, 0), (-1, 1), (1, 1)]) {
                if let Some(["M", "S", "A", "M", "S"]) = grid.gets(points) {
                    count += 1;
                }
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("example.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_puzzle1_example() {
        assert_eq!(puzzle1(EXAMPLE), 18);
    }

    #[test]
    fn test_puzzle1_input() {
        assert_eq!(puzzle1(INPUT), 2554);
    }

    #[test]
    fn test_puzzle2_example() {
        assert_eq!(puzzle2(EXAMPLE), 9);
    }

    #[test]
    fn test_puzzle2_input() {
        assert_eq!(puzzle2(INPUT), 1916);
    }
}
