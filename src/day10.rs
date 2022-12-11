use std::{collections::HashMap, fmt::Write, str::FromStr};

use itertools::Itertools;

use crate::parse_lines;

pub fn star_one(input: &str) -> i64 {
    let operations: Vec<Op> = parse_lines(input).collect();
    let mut machine = Machine::new(operations);
    let target_cycles = vec![20, 60, 100, 140, 180, 220];

    let mut result = 0;
    for c in 1..=220 {
        if target_cycles.contains(&c) {
            let x = machine.register(Register::X);
            result += x * c;
        }
        machine.tick();
    }

    result
}

pub fn star_two(input: &str) -> String {
    let operations: Vec<Op> = parse_lines(input).collect();
    let mut machine = Machine::new(operations);
    let display = (0..240).map(|c| {
        let coord = c % 40;
        let pixel = match machine.register(Register::X) {
            x if coord >= (x - 1) && coord <= x + 1 => '#',
            _ => '.',
        };

        machine.tick();

        pixel
    });

    let mut s = String::with_capacity(300);
    for line in display
        .chunks(40)
        .into_iter()
        .map(|chunk| chunk.collect::<String>())
    {
        writeln!(&mut s, "{}", line).expect("Failed to write to string");
    }

    println!("{s}");

    s
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Register {
    X,
}

#[derive(Debug)]
enum Op {
    NOOP,
    Add(Register, i64),
}

impl Op {
    fn cycles(&self) -> u64 {
        use Op::*;
        match self {
            NOOP => 1,
            Add(..) => 2,
        }
    }
}

impl FromStr for Op {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();

        let op = parts
            .next()
            .map(str::trim)
            .ok_or_else(|| format!(r#"No operation in "{s}""#))?;

        match op {
            "noop" => Ok(Self::NOOP),
            "addx" => {
                let arg = parts
                    .next()
                    .map(str::trim)
                    .ok_or_else(|| format!(r#"Missing argument for addx in "{s}"#))?;
                let parsed: i64 = arg
                    .parse()
                    .map_err(|e| format!(r#"Failed to parse addx argument in "{s}""#))?;

                Ok(Self::Add(Register::X, parsed))
            }
            _ => Err(format!(r#"Unknown op code `{op}` in "{s}""#)),
        }
    }
}

struct Machine {
    ip: usize,
    cycle_count: u64,
    op_cycle: u64,
    operations: Vec<Op>,
    registers: HashMap<Register, i64>,
}

impl Machine {
    fn new(operations: Vec<Op>) -> Self {
        Self {
            ip: 0,
            cycle_count: 0,
            op_cycle: 0,
            operations,
            registers: [(Register::X, 1)].into_iter().collect(),
        }
    }

    // Advance on cycle
    fn tick(&mut self) {
        self.op_cycle += 1;
        self.cycle_count += 1;

        if self.op_cycle == self.operations[self.ip].cycles() {
            self.apply();
            self.ip += 1;
            self.op_cycle = 0;
        }
    }

    /// Apply the current operation
    fn apply(&mut self) {
        use Op::*;
        match self.operations[self.ip] {
            NOOP => {}
            Add(reg, value) => *self.register_mut(reg) += value,
        }
    }

    fn register_mut(&mut self, reg: Register) -> &mut i64 {
        self.registers
            .get_mut(&reg)
            .unwrap_or_else(|| panic!("Unknown reqgister {:?}", reg))
    }

    fn register(&mut self, reg: Register) -> i64 {
        *self
            .registers
            .get(&reg)
            .unwrap_or_else(|| panic!("Unknown reqgister {:?}", reg))
    }
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};

    const INPUT: &'static str = include_str!("day_10_test.txt");

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(INPUT), 13140);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(
            star_two(INPUT).trim(),
            r#"
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."#
                .trim()
        );
    }
}
