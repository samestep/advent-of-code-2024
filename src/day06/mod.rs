use enumset::{EnumSet, EnumSetType};

struct Grid<'a> {
    string: &'a str,
    width: usize,
    start: (isize, isize),
}

impl<'a> Grid<'a> {
    fn new(string: &'a str) -> Self {
        let width = string.find('\n').unwrap();
        let start = string.find('^').unwrap();
        let x = start % (width + 1);
        let y = start / (width + 1);
        Self {
            string,
            width,
            start: (x as isize, y as isize),
        }
    }

    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.string.len() / (self.width + 1)
    }

    fn start(&self) -> (isize, isize) {
        self.start
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
}

#[derive(EnumSetType)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn vector(self) -> (isize, isize) {
        match self {
            Self::North => (0, -1),
            Self::East => (1, 0),
            Self::South => (0, 1),
            Self::West => (-1, 0),
        }
    }

    fn rotate(self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }
}

struct Visits {
    width: usize,
    positions: Vec<EnumSet<Direction>>,
}

impl Visits {
    fn new(grid: &Grid) -> Self {
        let width = grid.width();
        let positions = vec![EnumSet::empty(); width * grid.height()];
        Self { width, positions }
    }

    fn visit(&mut self, x: isize, y: isize, direction: Direction) -> bool {
        self.positions[y as usize * self.width + x as usize].insert(direction)
    }

    fn iter(&self) -> impl Iterator<Item = EnumSet<Direction>> + '_ {
        self.positions.iter().copied()
    }
}

fn patrol(grid: &Grid, obstacle: Option<(isize, isize)>) -> Option<usize> {
    let mut visited = Visits::new(grid);
    let (mut x, mut y) = grid.start();
    let mut direction = Direction::North;
    loop {
        match grid.get(x, y) {
            None => break,
            Some("^" | ".") => {
                if !visited.visit(x, y, direction) {
                    return None;
                }
                let (dx, dy) = direction.vector();
                x += dx;
                y += dy;
                if grid.get(x, y) == Some("#") || Some((x, y)) == obstacle {
                    x -= dx;
                    y -= dy;
                    direction = direction.rotate();
                }
            }
            Some(_) => panic!(),
        }
    }
    Some(
        visited
            .iter()
            .filter(|directions| !directions.is_empty())
            .count(),
    )
}

pub fn puzzle1(input: &str) -> usize {
    patrol(&Grid::new(input), None).unwrap()
}

pub fn puzzle2(input: &str) -> usize {
    let mut positions = 0;
    let grid = Grid::new(input);
    for y in 0..(grid.height() as isize) {
        for x in 0..(grid.width() as isize) {
            if (x, y) != grid.start() && patrol(&grid, Some((x, y))).is_none() {
                positions += 1;
            }
        }
    }
    positions
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("example.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_puzzle1_example() {
        assert_eq!(puzzle1(EXAMPLE), 41);
    }

    #[test]
    fn test_puzzle1_input() {
        assert_eq!(puzzle1(INPUT), 5145);
    }

    #[test]
    fn test_puzzle2_example() {
        assert_eq!(puzzle2(EXAMPLE), 6);
    }

    #[test]
    fn test_puzzle2_input() {
        assert_eq!(puzzle2(INPUT), 1523);
    }
}
