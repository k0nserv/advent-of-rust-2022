use std::{collections::HashSet, fmt, str::FromStr};

use crate::{math::Vector2, parse_lines};

pub fn star_one(input: &str) -> usize {
    solve(input, 1)
}

pub fn star_two(input: &str) -> usize {
    solve(input, 9)
}

fn solve(input: &str, tail_length: usize) -> usize {
    let motions: Vec<_> = parse_lines::<Motion>(input).collect();
    let mut state = State::new(tail_length);

    let mut tail_locations = HashSet::new();
    for motion in motions {
        state.apply(&motion, |s| {
            tail_locations.insert(*s.tail.last().unwrap());
        });
    }

    tail_locations.len()
}

#[derive(Default, Debug)]
struct State {
    head: Vector2<i64>,
    tail: Vec<Vector2<i64>>,
}

impl State {
    fn new(tail_length: usize) -> Self {
        Self {
            head: Vector2::default(),
            tail: vec![Vector2::default(); tail_length],
        }
    }

    fn apply<F>(&mut self, motion: &Motion, mut callback: F)
    where
        F: FnMut(&Self),
    {
        callback(self);

        for _ in 0..motion.steps {
            self.step_once(motion.direction);
            callback(self);
        }
    }

    fn step_once(&mut self, direction: Direction) {
        self.head = self.head + direction.into();

        for i in 0..self.tail.len() {
            let ahead = if i == 0 {
                &self.head
            } else {
                &self.tail[i - 1]
            };
            if ahead.is_adjacent(&self.tail[i]) {
                continue;
            }

            let dir = *ahead - self.tail[i];
            self.tail[i] =
                Vector2::new(self.tail[i].x + clamp(dir.x), self.tail[i].y + clamp(dir.y))
        }
    }
}

fn clamp(v: i64) -> i64 {
    if v < 0 {
        v.clamp(-1, 0)
    } else {
        v.clamp(0, 1)
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl From<Direction> for Vector2<i64> {
    fn from(dir: Direction) -> Self {
        match dir {
            Direction::Left => Vector2::new(-1, 0),
            Direction::Right => Vector2::new(1, 0),
            Direction::Up => Vector2::new(0, 1),
            Direction::Down => Vector2::new(0, -1),
        }
    }
}

impl FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let first = s
            .trim()
            .chars()
            .next()
            .ok_or_else(|| format!("Couldn't parse direction from {}", s))?;

        match first {
            'L' => Ok(Self::Left),
            'R' => Ok(Self::Right),
            'U' => Ok(Self::Up),
            'D' => Ok(Self::Down),
            _ => Err(format!("Invalid direction {} in {}", first, s)),
        }
    }
}

#[derive(Debug)]
struct Motion {
    direction: Direction,
    steps: usize,
}

impl FromStr for Motion {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        let (s1, s2) = s
            .split_once(char::is_whitespace)
            .ok_or_else(|| format!("Invalid motion {}", s))?;
        let direction = s1.parse()?;
        let steps = s2
            .parse()
            .map_err(|e| format!("Failed to parse steps in {}: {}", s, e))?;

        Ok(Self { direction, steps })
    }
}

impl Vector2<i64> {
    fn is_adjacent(&self, to: &Self) -> bool {
        let vec = *self - *to;

        (vec.x.pow(2) + vec.y.pow(2)) <= 2
    }
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};
    const INPUT_STAR_ONE: &'static str = r#"
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"#;

    const INPUT_STAR_TWO: &'static str = r#"
R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"#;

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(INPUT_STAR_ONE), 13);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(INPUT_STAR_ONE), 1);
        assert_eq!(star_two(INPUT_STAR_TWO), 36);
    }
}
