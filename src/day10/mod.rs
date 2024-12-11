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

    fn table<T: Clone>(&self, fill: T) -> Table<T> {
        Table::new(self.width(), self.height(), fill)
    }

    fn index(&self, i: usize) -> (isize, isize) {
        let x = (i % (self.width() + 1)) as isize;
        let y = (i / (self.width() + 1)) as isize;
        (x, y)
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

struct Table<T> {
    width: usize,
    elems: Vec<T>,
}

impl<T: Clone> Table<T> {
    fn new(width: usize, height: usize, fill: T) -> Self {
        let elems = vec![fill; width * height];
        Self { width, elems }
    }
}

impl<T> Table<T> {
    fn get_mut(&mut self, x: isize, y: isize) -> Option<&mut T> {
        let i: usize = y.try_into().ok()?;
        let j: usize = x.try_into().ok()?;
        if j >= self.width {
            return None;
        }
        self.elems.get_mut(i * self.width + j)
    }
}

pub fn puzzle1(input: &str) -> usize {
    let grid = Grid::new(input);
    input
        .chars()
        .enumerate()
        .filter(|&(_, c)| c == '0')
        .map(|(i, _)| {
            let mut visited = grid.table(false);
            let mut stack = vec![(0, grid.index(i))];
            while let Some((n, (x, y))) = stack.pop() {
                if grid.get(x, y).and_then(|s| s.parse().ok()) != Some(n) {
                    continue;
                }
                let here = visited.get_mut(x, y).unwrap();
                if *here {
                    continue;
                }
                *here = true;
                stack.push((n + 1, (x + 1, y)));
                stack.push((n + 1, (x, y - 1)));
                stack.push((n + 1, (x - 1, y)));
                stack.push((n + 1, (x, y + 1)));
            }
            let mut score = 0;
            for y in 0..(grid.height() as isize) {
                for x in 0..(grid.width() as isize) {
                    if grid.get(x, y) == Some("9") && *visited.get_mut(x, y).unwrap() {
                        score += 1;
                    }
                }
            }
            score
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("example.txt");
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_puzzle1_example() {
        assert_eq!(puzzle1(EXAMPLE), 36);
    }

    #[test]
    fn test_puzzle1_input() {
        assert_eq!(puzzle1(INPUT), 667);
    }
}
