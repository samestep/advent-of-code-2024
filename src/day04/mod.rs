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

    fn gets<const N: usize>(&self, points: [(isize, isize); N]) -> Option<[&str; N]> {
        let mut strings = [""; N];
        for (i, (x, y)) in points.into_iter().enumerate() {
            strings[i] = self.get(x, y)?;
        }
        Some(strings)
    }
}

fn solve<const M: usize, const N: usize>(
    needle: [&str; M],
    matrix: [[(isize, isize); M]; N],
    input: &str,
) -> usize {
    let mut count = 0;
    let grid = Grid::new(input);
    for y in 0..(grid.height() as isize) {
        for x in 0..(grid.width() as isize) {
            for points in matrix.map(|row| row.map(|(x0, y0)| (x + x0, y + y0))) {
                match grid.gets(points) {
                    Some(strings) if strings == needle => count += 1,
                    _ => {}
                }
            }
        }
    }
    count
}

pub fn puzzle1(input: &str) -> usize {
    solve(
        ["X", "M", "A", "S"],
        [
            [(0, 0), (1, 0), (2, 0), (3, 0)],
            [(0, 0), (1, -1), (2, -2), (3, -3)],
            [(0, 0), (0, -1), (0, -2), (0, -3)],
            [(0, 0), (-1, -1), (-2, -2), (-3, -3)],
            [(0, 0), (-1, 0), (-2, 0), (-3, 0)],
            [(0, 0), (-1, 1), (-2, 2), (-3, 3)],
            [(0, 0), (0, 1), (0, 2), (0, 3)],
            [(0, 0), (1, 1), (2, 2), (3, 3)],
        ],
        input,
    )
}

pub fn puzzle2(input: &str) -> usize {
    solve(
        ["M", "S", "A", "M", "S"],
        [
            [(-1, -1), (1, -1), (0, 0), (-1, 1), (1, 1)],
            [(-1, 1), (-1, -1), (0, 0), (1, 1), (1, -1)],
            [(1, 1), (-1, 1), (0, 0), (1, -1), (-1, -1)],
            [(1, -1), (1, 1), (0, 0), (-1, -1), (-1, 1)],
        ],
        input,
    )
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
