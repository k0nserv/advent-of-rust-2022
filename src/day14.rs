use core::fmt;
use std::collections::HashMap;
use std::ops::RangeInclusive;
use std::str::FromStr;

use itertools::Itertools;

use crate::math::Vector2;

pub fn star_one(input: &str) -> usize {
    let mut map: Map = input.parse().expect("Should be able to parse the map");

    while map.tick() {}

    map.sand_at_rest()
}

pub fn star_two(input: &str) -> usize {
    let mut map: Map = input.parse().expect("Should be able to parse the map");
    map.mode = Mode::Floor;

    while map.tick() {}

    map.sand_at_rest()
}

#[derive(Debug, PartialEq, Eq)]
enum Mode {
    Abyss,
    Floor,
}

struct Map {
    /// The locations occupied by something meaningful.
    /// This is sparse, air is absent.
    locations: HashMap<Vector2<isize>, Location>,
    /// The point in the y axis at which sand just falls into the abyss
    max_y: isize,
    spawn_location: Vector2<isize>,
    mode: Mode,
}

impl Map {
    fn new(
        locations: HashMap<Vector2<isize>, Location>,
        spawn_location: Vector2<isize>,
        mode: Mode,
    ) -> Self {
        let max_y = locations
            .keys()
            .max_by_key(|p| p.y)
            .map(|p| p.y)
            .unwrap_or(0);

        Self {
            locations,
            max_y,
            spawn_location,
            mode,
        }
    }

    /// Simulate a single unit of sand being spawned.
    ///
    /// Returns [`true`] if we should continue ticking, [`false`] if we are done per the rules of
    /// the mode.
    fn tick(&mut self) -> bool {
        let mut location = self.spawn_location;
        loop {
            if self.mode == Mode::Abyss && location.y >= self.max_y {
                return false;
            }

            let Some(fall_direction) = self.fall_location(location) else {
                // Came to rest
                break
            };

            location = fall_direction;
        }

        self.locations.insert(location, Location::Sand);

        if self.mode == Mode::Floor {
            self.locations.get(&self.spawn_location).is_none()
        } else {
            true
        }
    }

    /// Where will a given unit of sand fall to.
    fn fall_location(&self, location: Vector2<isize>) -> Option<Vector2<isize>> {
        let floor = self.max_y + 2;

        // Down
        let candidate = Vector2::new(location.x, location.y + 1);
        if !self.locations.contains_key(&candidate)
            && (self.mode == Mode::Abyss || floor != candidate.y)
        {
            return Some(candidate);
        }

        // Left
        let candidate = Vector2::new(location.x - 1, location.y + 1);
        if !self.locations.contains_key(&candidate)
            && (self.mode == Mode::Abyss || floor != candidate.y)
        {
            return Some(candidate);
        }

        // Right
        let candidate = Vector2::new(location.x + 1, location.y + 1);
        if !self.locations.contains_key(&candidate)
            && (self.mode == Mode::Abyss || floor != candidate.y)
        {
            return Some(candidate);
        }

        None
    }

    fn sand_at_rest(&self) -> usize {
        self.locations.values().filter(|l| l.is_sand()).count()
    }
}

impl FromStr for Map {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let locations = s
            .lines()
            .map(str::trim)
            .filter(|l| !l.is_empty())
            .flat_map(parse_rocks_in_line)
            .collect::<Result<HashMap<_, _>, String>>()?;

        Ok(Self::new(locations, Vector2::new(500, 0), Mode::Abyss))
    }
}

fn parse_rocks_in_line(
    line: &str,
) -> impl Iterator<Item = Result<(Vector2<isize>, Location), String>> + '_ {
    let mut last = None;
    line.split("->")
        .map(str::trim)
        .map(move |part| {
            let (x, y) = part
                .split_once(',')
                .ok_or_else(|| format!("Could not parse line from `{part}` in `{line}`"))?;
            let x: isize = x.parse().map_err(|e| {
                format!("Failed to parse x component in `{part}` within `{line}`: {e}")
            })?;
            let y: isize = y.parse().map_err(|e| {
                format!("Failed to parse y component in `{part}` within `{line}`: {e}")
            })?;

            let Some(last_point) = last else {
                last = Some(Vector2::new(x, y));
                return Ok(Box::new(std::iter::empty()) as Box<dyn Iterator<Item = _>>);
            };
            last = Some(Vector2::new(x, y));

            if last_point.x == x && last_point.y != y {
                Ok(Box::new(
                    make_range(last_point.y, y).map(move |y| (Vector2::new(x, y), Location::Rock)),
                ))
            } else if last_point.y == y && last_point.x != x {
                Ok(Box::new(
                    make_range(last_point.x, x).map(move |x| (Vector2::new(x, y), Location::Rock)),
                ))
            } else {
                unreachable!("Line is neither horizontal nor vertical");
            }
        })
        .flatten_ok()
}

fn make_range(a: isize, b: isize) -> RangeInclusive<isize> {
    if a <= b {
        a..=b
    } else {
        b..=a
    }
}

#[derive(Debug, Copy, Clone)]
enum Location {
    Rock,
    Sand,
}

impl Location {
    fn is_sand(&self) -> bool {
        matches!(self, Location::Sand)
    }
}

const DEBUG_PADDING: isize = 2;
impl fmt::Debug for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let min = Vector2::new(
            self.locations.keys().map(|p| p.x).min().unwrap_or(0) - DEBUG_PADDING,
            self.locations.keys().map(|p| p.y).min().unwrap_or(0) - DEBUG_PADDING,
        );
        let max = Vector2::new(
            self.locations.keys().map(|p| p.x).max().unwrap_or(0) + DEBUG_PADDING,
            self.locations.keys().map(|p| p.y).max().unwrap_or(0)
                + DEBUG_PADDING
                + isize::from(self.mode == Mode::Floor) * 2,
        );

        for y in min.y..=max.y {
            for x in min.x..=max.x {
                if self.mode == Mode::Floor && y == self.max_y + 2 {
                    write!(f, "#")?;
                    continue;
                }

                match self.locations.get(&Vector2::new(x, y)) {
                    Some(Location::Rock) => write!(f, "#")?,
                    Some(Location::Sand) => write!(f, "O")?,
                    None => write!(f, ".")?,
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};
    static INPUT: &'static str = r#"
498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
"#;

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(INPUT), 24);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(INPUT), 93);
    }
}
