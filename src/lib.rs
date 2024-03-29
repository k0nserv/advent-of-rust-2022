#![allow(dead_code, unused)]
use std::fs::File;
use std::io::Read;
use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
#[macro_use]
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod math;

#[derive(Debug, Copy, Clone)]
pub struct DigitIterator {
    initial_value_is_zero: bool,
    number: f64,
}

impl DigitIterator {
    fn new(number: usize) -> Self {
        Self {
            initial_value_is_zero: number == 0,
            number: number as f64,
        }
    }
}

impl Iterator for DigitIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.number < 1.0 && !self.initial_value_is_zero {
            return None;
        }

        if self.initial_value_is_zero {
            self.initial_value_is_zero = false;

            Some(0)
        } else {
            let digit = self.number % 10_f64;
            self.number = (self.number / 10_f64).floor();

            Some(digit as usize)
        }
    }
}

fn time<F>(label: &str, closure: F)
where
    F: Fn(),
{
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    closure();
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let time = end - start;
    println!(
        "Time taken for {}: {}s and {}ns",
        label,
        time.as_secs(),
        time.subsec_nanos()
    );
}

/// Parse lines of text into custom types.
///
/// Each line is treated as parsable after trimming.
///
/// **Note:** Panics if any parsing fails
pub fn parse_lines<T>(input: &str) -> impl Iterator<Item = T> + '_
where
    T: FromStr + std::fmt::Debug,
    <T as FromStr>::Err: std::fmt::Debug,
{
    input
        .lines()
        .map(str::trim)
        .filter(|l| l.len() > 0)
        .map(|l| {
            l.parse().expect(&format!(
                "Expected to be able to parse `{:?}` as `{:?}`",
                l,
                std::any::type_name::<T>()
            ))
        })
}

/// Parse whitespace separated custom types.
///
/// Each unit separated by whitespace is treated as parsable after trimming.
///
/// **Note:** Panics if any parsing fails
pub fn parse_whitespace_separated<T>(input: &str) -> impl Iterator<Item = T> + '_
where
    T: FromStr + std::fmt::Debug,
    <T as FromStr>::Err: std::fmt::Debug,
{
    input
        .split_whitespace()
        .map(str::trim)
        .filter(|l| l.len() > 0)
        .map(|l| {
            l.parse().expect(&format!(
                "Expected to be able to parse `{:?}` as `{:?}`",
                l,
                std::any::type_name::<T>()
            ))
        })
}

/// Parse custom separator separated custom types.
///
/// Each unit separated by a specific separator is treated as parsable after trimming.
///
/// **Note:** Panics if any parsing fails
pub fn parse_custom_separated<'a, T>(
    input: &'a str,
    separator: &'a str,
) -> impl Iterator<Item = T> + 'a
where
    T: FromStr + std::fmt::Debug,
    <T as FromStr>::Err: std::fmt::Debug,
{
    input
        .split(separator)
        .map(str::trim)
        .filter(|l| l.len() > 0)
        .map(|l| {
            l.parse().expect(&format!(
                "Expected to be able to parse `{:?}` as `{:?}`",
                l,
                std::any::type_name::<T>()
            ))
        })
}

pub fn load_file(path: &str) -> String {
    let mut input = String::new();
    let mut f = File::open(path).expect("Unable to open file");
    f.read_to_string(&mut input).expect("Unable to read string");

    input
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Read;

    fn load_file(path: &str) -> String {
        let mut input = String::new();
        let mut f = File::open(path).expect("Unable to open file");
        f.read_to_string(&mut input).expect("Unable to read string");

        input
    }

    #[test]
    fn solve_day01() {
        use crate::day01::{star_one, star_two};

        let input = load_file("day01.txt");

        assert_eq!(star_one(&input), 66616);
        assert_eq!(star_two(&input), 199172);
    }

    #[test]
    fn solve_day02() {
        use crate::day02::{star_one, star_two};

        let input = load_file("day02.txt");

        assert_eq!(star_one(&input), 13052);
        assert_eq!(star_two(&input), 13693);
    }

    #[test]
    fn solve_day03() {
        use crate::day03::{star_one, star_two};

        let input = load_file("day03.txt");

        assert_eq!(star_one(&input), 7746);
        assert_eq!(star_two(&input), 2604);
    }

    #[test]
    fn solve_day04() {
        use super::time;
        use crate::day04::{star_one, star_two};

        let input = load_file("day04.txt");

        time("Day 04 start one", || {
            assert_eq!(star_one(&input), 413);
        });
        time("Day 04 start one", || {
            assert_eq!(star_two(&input), 806);
        });
    }

    #[test]
    fn solve_day05() {
        use crate::day05::{star_one, star_two};

        let input = load_file("day05.txt");

        assert_eq!(star_one(&input), "QGTHFZBHV");
        assert_eq!(star_two(&input), "MGDMPSZTM");
    }

    #[test]
    fn solve_day06() {
        use crate::day06::{star_one, star_two};

        let input = load_file("day06.txt");

        assert_eq!(star_one(&input), 1343);
        assert_eq!(star_two(&input), 2193);
    }

    #[test]
    fn solve_day07() {
        use crate::day07::{star_one, star_two};

        let input = load_file("day07.txt");

        assert_eq!(star_one(&input), 1743217);
        assert_eq!(star_two(&input), 8319096);
    }

    #[test]
    fn solve_day08() {
        use crate::day08::{star_one, star_two};

        let input = load_file("day08.txt");

        assert_eq!(star_one(&input), 1782);
        assert_eq!(star_two(&input), 474606);
    }

    #[test]
    fn solve_day09() {
        use crate::day09::{star_one, star_two};

        let input = load_file("day09.txt");

        assert_eq!(star_one(&input), 6642);
        assert_eq!(star_two(&input), 2765);
    }

    #[test]
    fn solve_day10() {
        use crate::day10::{star_one, star_two};

        let input = load_file("day10.txt");

        assert_eq!(star_one(&input), 11780);
        assert_eq!(
            star_two(&input).trim(),
            "
###..####.#..#.#....###...##..#..#..##..
#..#....#.#..#.#....#..#.#..#.#..#.#..#.
#..#...#..#..#.#....###..#..#.#..#.#..#.
###...#...#..#.#....#..#.####.#..#.####.
#....#....#..#.#....#..#.#..#.#..#.#..#.
#....####..##..####.###..#..#..##..#..#.
"
            .trim()
        );
    }

    #[test]
    fn solve_day11() {
        use crate::day11::prelude::*;

        let monkeys = monkeys!(
            {
            Monkey 0:
              Starting items: 84, 66, 62, 69, 88, 91, 91
              Operation: new = old * 11
              Test: divisible by 2
                If true: throw to monkey 4
                If false: throw to monkey 7
            }

            {
            Monkey 1:
              Starting items: 98, 50, 76, 99
              Operation: new = old * old
              Test: divisible by 7
                If true: throw to monkey 3
                If false: throw to monkey 6
            }

            {
            Monkey 2:
              Starting items: 72, 56, 94
              Operation: new = old + 1
              Test: divisible by 13
                If true: throw to monkey 4
                If false: throw to monkey 0
            }

            {
            Monkey 3:
              Starting items: 55, 88, 90, 77, 60, 67
              Operation: new = old + 2
              Test: divisible by 3
                If true: throw to monkey 6
                If false: throw to monkey 5
            }

            {
            Monkey 4:
              Starting items: 69, 72, 63, 60, 72, 52, 63, 78
              Operation: new = old * 13
              Test: divisible by 19
                If true: throw to monkey 1
                If false: throw to monkey 7
            }

            {
            Monkey 5:
              Starting items: 89, 73
              Operation: new = old + 5
              Test: divisible by 17
                If true: throw to monkey 2
                If false: throw to monkey 0
            }

            {
            Monkey 6:
              Starting items: 78, 68, 98, 88, 66
              Operation: new = old + 6
              Test: divisible by 11
                If true: throw to monkey 2
                If false: throw to monkey 5
            }

            {
            Monkey 7:
              Starting items: 70
              Operation: new = old + 7
              Test: divisible by 5
                If true: throw to monkey 1
                If false: throw to monkey 3
            }
        );

        assert_eq!(star_one(monkeys.clone()), 99840);
        assert_eq!(star_two(monkeys), 20683044837);
    }

    #[test]
    fn solve_day12() {
        use crate::day12::{star_one, star_two};

        let input = load_file("day12.txt");

        assert_eq!(star_one(&input), 528);
        assert_eq!(star_two(&input), 522);
    }

    #[test]
    fn solve_day13() {
        use crate::day13::{star_one, star_two};

        let input = load_file("day13.txt");

        assert_eq!(star_one(&input), 5529);
        assert_eq!(star_two(&input), 27690);
    }

    #[test]
    fn solve_day14() {
        use crate::day14::{star_one, star_two};

        let input = load_file("day14.txt");

        assert_eq!(star_one(&input), 1003);
        assert_eq!(star_two(&input), 25771);
    }

    #[test]
    fn solve_day15() {
        use crate::day15::{star_one, star_two};

        let input = load_file("day15.txt");

        assert_eq!(star_one(&input), 1);
        assert_eq!(star_two(&input), 1);
    }

    #[test]
    fn solve_day16() {
        use crate::day16::{star_one, star_two};

        let input = load_file("day16.txt");

        assert_eq!(star_one(&input), 1);
        assert_eq!(star_two(&input), 1);
    }

    #[test]
    fn solve_day17() {
        use crate::day17::{star_one, star_two};

        let input = load_file("day17.txt");

        assert_eq!(star_one(&input), 1);
        assert_eq!(star_two(&input), 1);
    }

    #[test]
    fn solve_day18() {
        use crate::day18::{star_one, star_two};

        let input = load_file("day18.txt");

        assert_eq!(star_one(&input), 1);
        assert_eq!(star_two(&input), 1);
    }

    #[test]
    fn solve_day19() {
        use crate::day19::{star_one, star_two};

        let input = load_file("day19.txt");

        assert_eq!(star_one(&input), 1);
        assert_eq!(star_two(&input), 1);
    }

    #[test]
    fn solve_day20() {
        use crate::day20::{star_one, star_two};

        let input = load_file("day20.txt");

        assert_eq!(star_one(&input), 1);
        assert_eq!(star_two(&input), 1);
    }

    #[test]
    fn solve_day21() {
        use crate::day21::{star_one, star_two};

        let input = load_file("day21.txt");

        assert_eq!(star_one(&input), 1);
        assert_eq!(star_two(&input), 1);
    }

    #[test]
    fn solve_day22() {
        use crate::day22::{star_one, star_two};

        let input = load_file("day22.txt");

        assert_eq!(star_one(&input), 1);
        assert_eq!(star_two(&input), 1);
    }

    #[test]
    fn solve_day23() {
        use crate::day23::{star_one, star_two};

        let input = load_file("day23.txt");

        assert_eq!(star_one(&input), 1);
        assert_eq!(star_two(&input), 1);
    }

    #[test]
    fn solve_day24() {
        use crate::day24::{star_one, star_two};

        let input = load_file("day24.txt");

        assert_eq!(star_one(&input), 1);
        assert_eq!(star_two(&input), 1);
    }
}
