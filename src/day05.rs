use std::collections::{HashMap, VecDeque};
use std::str::FromStr;

use crate::parse_lines;

pub fn star_one(input: &str) -> String {
    solve(input, Kind::CrateMover9000)
}

pub fn star_two(input: &str) -> String {
    solve(input, Kind::CrateMover9001)
}

fn solve(input: &str, kind: Kind) -> String {
    let (mut state, commands) = parse(input, kind);

    for command in commands {
        state.apply(command);
    }

    state.top_of_stacks()
}

fn parse(input: &str, kind: Kind) -> (State, Vec<Command>) {
    let (initial_state, commands) = input
        .split_once("\n\n")
        .expect("Input should contain two groups separated by two new lines");
    let mut state: State = initial_state
        .parse()
        .expect("Failed to create state from input");
    state.kind = kind;
    let commands = parse_lines(commands).collect();

    (state, commands)
}

#[derive(Debug, PartialEq, Eq)]
enum Kind {
    CrateMover9000,
    CrateMover9001,
}

#[derive(Debug)]
struct State {
    stacks: HashMap<usize, VecDeque<char>>,
    kind: Kind,
}

impl State {
    fn apply(&mut self, command: Command) {
        let stack = self.stacks.get_mut(&command.source).unwrap();
        let items = stack.split_off(stack.len() - command.count);
        match self.kind {
            Kind::CrateMover9000 => self
                .stacks
                .get_mut(&command.destination)
                .unwrap()
                .extend(items.into_iter().rev()),
            Kind::CrateMover9001 => self
                .stacks
                .get_mut(&command.destination)
                .unwrap()
                .extend(items.into_iter()),
        };
    }

    fn top_of_stacks(&self) -> String {
        (1..=9)
            .flat_map(|idx| self.stacks.get(&idx).and_then(|s| s.back()))
            .collect()
    }
}

impl FromStr for State {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let last_line = s
            .lines()
            .rev()
            .next()
            .ok_or_else(|| String::from("State definition has no lines"))?;
        let indices: HashMap<usize, usize> = last_line
            .chars()
            .enumerate()
            .filter(|(_, i)| i.is_ascii_digit())
            .map(|(idx, id)| (idx, id.to_digit(10).unwrap() as usize))
            .collect();

        let iter = s.lines().rev().skip(1).filter(|l| !l.is_empty()).map(|l| {
            l.chars()
                .enumerate()
                .filter(|(_, c)| c.is_ascii_alphabetic())
                .map(|(idx, c)| (indices[&idx], c))
        });

        let stacks = iter.fold(HashMap::<usize, VecDeque<char>>::new(), |mut acc, row| {
            for (idx, item) in row {
                let entry = acc.entry(idx).or_default();
                entry.push_back(item);
            }

            acc
        });

        Ok(Self {
            stacks,
            kind: Kind::CrateMover9000,
        })
    }
}

#[derive(Debug, Copy, Clone)]
struct Command {
    destination: usize,
    source: usize,
    count: usize,
}

impl FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Who needs regex, pfft
        let rest = s.strip_prefix("move").ok_or_else(|| make_error(s))?;

        let (count, rest) = rest
            .trim_start()
            .split_once(char::is_whitespace)
            .ok_or_else(|| make_error(rest))?;
        let count: usize = count
            .parse()
            .map_err(|e| format!("Invalid count {} in {}, err: {}", count, s, e))?;

        let rest = rest
            .trim_start()
            .strip_prefix("from")
            .ok_or_else(|| make_error(s))?;

        let (source, rest) = rest
            .trim_start()
            .split_once(char::is_whitespace)
            .ok_or_else(|| make_error(s))?;
        let source: usize = source
            .parse()
            .map_err(|e| format!("Invalid source {} in {}, err: {}", source, s, e))?;

        let rest = rest
            .trim_start()
            .strip_prefix("to")
            .ok_or_else(|| make_error(s))?;

        let destination: usize = rest
            .trim()
            .parse()
            .map_err(|e| format!("Invalid destination {} in {}, err: {}", source, s, e))?;

        Ok(Self {
            source,
            destination,
            count,
        })
    }
}

fn make_error(s: &str) -> String {
    format!("Invalid command `{}`", s)
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};

    const INPUT: &'static str = r#"
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
"#;

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(INPUT), "CMZ");
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(INPUT), "MCD");
    }
}
