use std::{
    collections::{BinaryHeap, HashMap},
    ops::Index,
    str::FromStr,
};

pub fn star_one(input: &str) -> usize {
    let map: HeightMap = input.parse().expect("Failed to parse height map");

    map.shortest_path(map.start, map.end)
        .expect("No shortest path found")
        .len()
        - 1
}

pub fn star_two(input: &str) -> usize {
    let map: HeightMap = input.parse().expect("Failed to parse height map");
    let came_from = map.djikstra(map.end);

    let map = &map;
    let came_from = &came_from;
    let (loc, path) = (0..map.width())
        .flat_map(|x| {
            (0..map.height()).filter_map(move |y| {
                let loc = (x, y);
                if map[loc] != 0 {
                    return None;
                }

                let path = reconstruct_path(came_from, loc);

                (path[0] == map.end).then_some((loc, path))
            })
        })
        .inspect(|(loc, path)| println!("Found path with length {} at {loc:?}", path.len()))
        .min_by_key(|(_, path)| path.len())
        .expect("There should be at least one point at the lowest level");

    dbg!(loc);
    path.len() - 1
}

#[derive(Debug)]
struct HeightMap {
    map: Vec<Vec<u8>>,
    start: (usize, usize),
    end: (usize, usize),
}

impl HeightMap {
    fn shortest_path(
        &self,
        from: (usize, usize),
        to: (usize, usize),
    ) -> Option<Vec<(usize, usize)>> {
        let came_from = self.djikstra(to);

        Some(reconstruct_path(&came_from, from))
    }

    fn valid_neighbors_backwards(&self, from: (usize, usize)) -> Vec<(usize, usize)> {
        let current_height = self[from] as i16;
        let (x, y) = from;
        let mut result = Vec::with_capacity(4);

        if x > 0 && current_height - (self[(x - 1, y)] as i16) <= 1 {
            result.push((x - 1, y));
        }

        if y > 0 && current_height - (self[(x, y - 1)] as i16) <= 1 {
            result.push((x, y - 1));
        }

        if x < self.width() - 1 && current_height - (self[(x + 1, y)] as i16) <= 1 {
            result.push((x + 1, y));
        }

        if y < self.height() - 1 && current_height - (self[(x, y + 1)] as i16) <= 1 {
            result.push((x, y + 1));
        }

        result
    }

    fn width(&self) -> usize {
        self.map[0].len()
    }

    fn height(&self) -> usize {
        self.map.len()
    }

    fn djikstra(&self, from: (usize, usize)) -> HashMap<(usize, usize), (usize, usize)> {
        let max = (self.width(), self.height());

        let mut open = BinaryHeap::new();
        for x in 0..max.0 {
            for y in 0..max.1 {
                let p = (x, y);

                if p != from {
                    open.push(HeapEntry::new((x, y), usize::MAX));
                }
            }
        }
        open.push(HeapEntry::new(from, 0));

        let mut came_from: HashMap<(usize, usize), (usize, usize)> = Default::default();

        let mut distance: HashMap<(usize, usize), usize> = Default::default();
        distance.insert(from, 0);

        while !open.is_empty() {
            let current = open.pop().unwrap();

            for neighbor in self.valid_neighbors_backwards(current.location) {
                let alt = distance
                    .get(&current.location)
                    .map(|d| d + 1)
                    .unwrap_or(usize::MAX);

                if alt < *distance.get(&neighbor).unwrap_or(&usize::MAX) {
                    distance.insert(neighbor, alt);
                    came_from.insert(neighbor, current.location);
                    open.push(HeapEntry::new(neighbor, alt));
                }
            }
        }

        came_from
    }
}

fn reconstruct_path(
    came_from: &HashMap<(usize, usize), (usize, usize)>,
    end: (usize, usize),
) -> Vec<(usize, usize)> {
    let mut path = vec![end];
    let mut current = &end;

    while let Some(prev) = came_from.get(current) {
        path.push(*prev);
        current = prev;
    }

    path.reverse();

    path
}

impl Index<(usize, usize)> for HeightMap {
    type Output = u8;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.map[index.1][index.0]
    }
}

impl FromStr for HeightMap {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut start: Option<(usize, usize)> = None;
        let mut end: Option<(usize, usize)> = None;

        let map: Result<Vec<Vec<u8>>, String> = s
            .trim()
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.trim()
                    .chars()
                    .enumerate()
                    .map(|(x, c)| match c {
                        'a'..='z' => Ok((c as u8) - 97),
                        'S' => {
                            start = Some((x, y));

                            Ok(b'a' - 97)
                        }
                        'E' => {
                            end = Some((x, y));

                            Ok(b'z' - 97)
                        }
                        _ => Err(format!("Unexpected character: '{c}' in {line}")),
                    })
                    .collect::<Result<Vec<u8>, String>>()
            })
            .collect();

        let start = start.ok_or_else(|| "No start location found".to_owned())?;
        let end = end.ok_or_else(|| "No end location found".to_owned())?;

        Ok(Self {
            map: map?,
            start,
            end,
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
struct HeapEntry<T> {
    location: (usize, usize),
    value: T,
}

impl<T: Ord> HeapEntry<T> {
    fn new(location: (usize, usize), value: T) -> Self {
        Self { location, value }
    }
}

impl<T: Ord> Ord for HeapEntry<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .value
            .cmp(&self.value)
            .then_with(|| self.location.cmp(&other.location))
    }
}
impl<T: Ord> PartialOrd for HeapEntry<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BinaryHeap;

    use super::{star_one, star_two, HeapEntry};
    static INPUT: &'static str = r#"
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi"#;

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(INPUT), 31);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(INPUT), 29);
    }
}
