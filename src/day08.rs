use std::collections::HashSet;
use std::ops::{ControlFlow, Range};

pub fn star_one(input: &str) -> usize {
    let grid = parse(input);

    let visible: HashSet<_> = (0..grid.height())
        .flat_map(|y| {
            grid.visible_trees(Direction::Left { y })
                .chain(grid.visible_trees(Direction::Right { y }))
        })
        .chain((0..grid.width()).flat_map(|x| {
            grid.visible_trees(Direction::Up { x })
                .chain(grid.visible_trees(Direction::Down { x }))
        }))
        .collect();

    visible.len()
}

pub fn star_two(input: &str) -> usize {
    let grid = parse(input);

    let grid = &grid;
    // BRUUUUUTE FORCE!
    (0..grid.height())
        .flat_map(|y| {
            (0..grid.width()).map(move |x| {
                let height = grid.grid[y][x];

                let do_score = |dir: Direction, dim: usize| {
                    let mut found_edge = true;
                    let count = grid
                        .scan(dir, Some(dim), |other, _| {
                            found_edge = found_edge && other < height;
                            other < height
                        })
                        .count();

                    if count == 0 {
                        0
                    } else {
                        count + usize::from(!found_edge)
                    }
                };

                let up_score = do_score(Direction::Up { x }, y);
                let down_score = do_score(Direction::Down { x }, y);
                let left_score = do_score(Direction::Left { y }, x);
                let right_score = do_score(Direction::Right { y }, x);

                up_score * down_score * left_score * right_score
            })
        })
        .max()
        .unwrap()
}

fn parse(input: &str) -> Grid {
    let grid = input
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .map(|line| {
            line.chars()
                .map(|c| {
                    assert!(c.is_ascii_digit());

                    c.to_digit(10).unwrap() as u8
                })
                .collect()
        })
        .collect();

    Grid { grid }
}

#[derive(Debug)]
struct Grid {
    grid: Vec<Vec<u8>>,
}

impl Grid {
    /// Scan the grid.
    fn scan<'s, 'f: 's, F>(
        &'s self,
        direction: Direction,
        dim_start: Option<usize>,
        mut predicate: F,
    ) -> impl Iterator<Item = (u8, (usize, usize))> + 's
    where
        F: FnMut(u8, (usize, usize)) -> bool + 'f,
    {
        let range: Box<dyn Iterator<Item = (usize, usize)>> = match direction {
            d @ (Direction::Up { x } | Direction::Down { x }) => {
                Box::new(d.range(self.height(), dim_start).map(move |y| (x, y)))
            }
            d @ (Direction::Left { y } | Direction::Right { y }) => {
                Box::new(d.range(self.width(), dim_start).map(move |x| (x, y)))
            }
        };

        range
            .map(move |(x, y)| {
                let height = self.grid[y][x];

                (height, (x, y))
            })
            .take_while(move |(height, loc)| predicate(*height, *loc))
    }

    fn visible_trees(&self, direction: Direction) -> impl Iterator<Item = (usize, usize)> + '_ {
        let iter = self.scan(direction, None, |_, _| true);

        let mut max_height = None;
        iter.filter_map(move |(height, (x, y))| {
            let is_visible = max_height.map(|c| height > c).unwrap_or(true);
            max_height = max_height.map(|c| c.max(height)).or(Some(height));

            is_visible.then_some((x, y))
        })
    }

    fn width(&self) -> usize {
        self.grid[0].len()
    }

    fn height(&self) -> usize {
        self.grid.len()
    }
}

#[derive(Copy, Clone)]
enum Direction {
    Up { x: usize },
    Left { y: usize },
    Right { y: usize },
    Down { x: usize },
}

impl Direction {
    fn range(&self, max_dim: usize, start_dim: Option<usize>) -> Box<dyn Iterator<Item = usize>> {
        match self {
            Direction::Up { .. } => Box::new((0..(start_dim.unwrap_or(max_dim))).rev()),
            Direction::Down { .. } => Box::new(start_dim.map(|d| d + 1).unwrap_or(0)..max_dim),
            Direction::Left { .. } => Box::new((0..(start_dim.unwrap_or(max_dim))).rev()),
            Direction::Right { .. } => Box::new(start_dim.map(|d| d + 1).unwrap_or(0)..max_dim),
        }
    }

    fn is_horizontal(&self) -> bool {
        matches!(self, Self::Left { .. } | Self::Right { .. })
    }

    fn is_vertical(&self) -> bool {
        matches!(self, Self::Up { .. } | Self::Down { .. })
    }
}

#[cfg(test)]
mod tests {
    use crate::day08::Direction;

    use super::{parse, star_one, star_two};
    const INPUT: &'static str = r#"
30373
25512
65332
33549
35390
"#;

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(INPUT), 21);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(INPUT), 8);
    }

    #[test]
    fn test_scan_with_starting_point_up() {
        let grid = parse(INPUT);

        let result: Vec<_> = grid
            .scan(Direction::Up { x: 2 }, Some(4), |_, _| true)
            .collect();
        let expected = vec![(5, (2, 3)), (3, (2, 2)), (5, (2, 1)), (3, (2, 0))];
        assert_eq!(result, expected);

        let result: Vec<_> = grid
            .scan(Direction::Up { x: 2 }, Some(0), |_, _| true)
            .collect();
        let expected = vec![];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_scan_with_starting_point_left() {
        let grid = parse(INPUT);

        let result: Vec<_> = grid
            .scan(Direction::Left { y: 1 }, Some(3), |_, _| true)
            .collect();
        let expected = vec![(5, (2, 1)), (5, (1, 1)), (2, (0, 1))];
        assert_eq!(result, expected);

        let result: Vec<_> = grid
            .scan(Direction::Left { y: 1 }, Some(0), |_, _| true)
            .collect();
        let expected = vec![];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_scan_with_starting_point_down() {
        let grid = parse(INPUT);

        let result: Vec<_> = grid
            .scan(Direction::Down { x: 2 }, Some(3), |_, _| true)
            .collect();
        let expected = vec![(3, (2, 4))];

        assert_eq!(result, expected);

        let result: Vec<_> = grid
            .scan(Direction::Down { x: 2 }, Some(4), |_, _| true)
            .collect();
        let expected = vec![];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_scan_with_starting_point_right() {
        let grid = parse(INPUT);

        let result: Vec<_> = grid
            .scan(Direction::Right { y: 1 }, Some(3), |_, _| true)
            .collect();
        let expected = vec![(2, (4, 1))];

        assert_eq!(result, expected);

        let result: Vec<_> = grid
            .scan(Direction::Right { y: 1 }, Some(4), |_, _| true)
            .collect();
        let expected = vec![];

        assert_eq!(result, expected);
    }
}
