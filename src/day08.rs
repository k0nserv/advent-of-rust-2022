use std::collections::HashSet;

use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;

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
                    grid.scan(dir, Some(dim))
                        .fold_while(0, |acc, (other, _)| {
                            if other < height {
                                Continue(acc + 1)
                            } else {
                                Done(acc + 1)
                            }
                        })
                        .into_inner()
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
                    c.to_digit(10)
                        .expect("All trees should be represented by numbers between 0 and 9")
                        as u8
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
    fn scan(
        &self,
        direction: Direction,
        dim_start: Option<usize>,
    ) -> impl Iterator<Item = (u8, (usize, usize))> + '_ {
        let range: Box<dyn Iterator<Item = (usize, usize)>> = match direction {
            d @ (Direction::Up { x } | Direction::Down { x }) => {
                Box::new(d.range(self.height(), dim_start).map(move |y| (x, y)))
            }
            d @ (Direction::Left { y } | Direction::Right { y }) => {
                Box::new(d.range(self.width(), dim_start).map(move |x| (x, y)))
            }
        };

        range.map(move |(x, y)| {
            let height = self.grid[y][x];

            (height, (x, y))
        })
    }

    fn visible_trees(&self, direction: Direction) -> impl Iterator<Item = (usize, usize)> + '_ {
        let mut max_height = None;
        self.scan(direction, None)
            .filter_map(move |(height, (x, y))| {
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

        let result: Vec<_> = grid.scan(Direction::Up { x: 2 }, Some(4)).collect();
        let expected = vec![(5, (2, 3)), (3, (2, 2)), (5, (2, 1)), (3, (2, 0))];
        assert_eq!(result, expected);

        let result: Vec<_> = grid.scan(Direction::Up { x: 2 }, Some(0)).collect();
        let expected = vec![];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_scan_with_starting_point_left() {
        let grid = parse(INPUT);

        let result: Vec<_> = grid.scan(Direction::Left { y: 1 }, Some(3)).collect();
        let expected = vec![(5, (2, 1)), (5, (1, 1)), (2, (0, 1))];
        assert_eq!(result, expected);

        let result: Vec<_> = grid.scan(Direction::Left { y: 1 }, Some(0)).collect();
        let expected = vec![];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_scan_with_starting_point_down() {
        let grid = parse(INPUT);

        let result: Vec<_> = grid.scan(Direction::Down { x: 2 }, Some(3)).collect();
        let expected = vec![(3, (2, 4))];

        assert_eq!(result, expected);

        let result: Vec<_> = grid.scan(Direction::Down { x: 2 }, Some(4)).collect();
        let expected = vec![];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_scan_with_starting_point_right() {
        let grid = parse(INPUT);

        let result: Vec<_> = grid.scan(Direction::Right { y: 1 }, Some(3)).collect();
        let expected = vec![(2, (4, 1))];

        assert_eq!(result, expected);

        let result: Vec<_> = grid.scan(Direction::Right { y: 1 }, Some(4)).collect();
        let expected = vec![];

        assert_eq!(result, expected);
    }
}
