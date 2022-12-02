use std::str::FromStr;

use crate::parse_lines;

pub fn star_one(input: &str) -> u64 {
    score(parse_lines::<Suggestion>(input))
}

pub fn star_two(input: &str) -> u64 {
    score(parse_lines::<Suggestion>(input).map(|s| {
        // reinterpret the input, flipped because we are considering it from the perspective of
        // the opponent
        let desired_outcome = match s.you {
            Action::Rock => Outcome::Win,
            Action::Paper => Outcome::Draw,
            Action::Scissors => Outcome::Lose,
        };

        let action = s.opponent.desired_outcome(desired_outcome);

        Suggestion {
            opponent: s.opponent,
            you: action,
        }
    }))
}

fn score(suggestions: impl IntoIterator<Item = Suggestion>) -> u64 {
    suggestions
        .into_iter()
        .map(|s| s.you.score() + s.you.outcome(s.opponent).score())
        .sum()
}

#[derive(Debug)]
struct Suggestion {
    opponent: Action,
    you: Action,
}

impl FromStr for Suggestion {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let mut parts = value.split_whitespace().map(str::trim);
        let opponent = parts
            .next()
            .ok_or(format!("Expected two parts in {}", value))
            .and_then(str::parse)?;
        let you = parts
            .next()
            .ok_or(format!("Expected two parts in {}", value))
            .and_then(str::parse)?;

        Ok(Suggestion { opponent, you })
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Action {
    Rock,
    Paper,
    Scissors,
}

impl Action {
    fn score(&self) -> u64 {
        match self {
            Action::Rock => 1,
            Action::Paper => 2,
            Action::Scissors => 3,
        }
    }

    fn outcome(&self, opponent_action: Self) -> Outcome {
        match (self, opponent_action) {
            (a @ _, b @ _) if a == &b => Outcome::Draw,
            (Self::Rock, Self::Scissors)
            | (Self::Paper, Self::Rock)
            | (Self::Scissors, Self::Paper) => Outcome::Win,
            _ => Outcome::Lose,
        }
    }

    fn desired_outcome(&self, outcome: Outcome) -> Self {
        match (self, outcome) {
            (_, Outcome::Draw) => *self,
            (Action::Rock, Outcome::Win) | (Action::Paper, Outcome::Lose) => Action::Scissors,
            (Action::Rock, Outcome::Lose) | (Action::Scissors, Outcome::Win) => Action::Paper,
            (Action::Scissors, Outcome::Lose) | (Action::Paper, Outcome::Win) => Action::Rock,
        }
    }
}

impl FromStr for Action {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.trim() {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissors),
            _ => Err(format!("Invalid option {}", value)),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Outcome {
    Win,
    Draw,
    Lose,
}

impl Outcome {
    fn score(&self) -> u64 {
        match self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Lose => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};
    const TEST_INPUT: &'static str = r#"
A Y
B X
C Z
"#;

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(TEST_INPUT), 15);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(TEST_INPUT), 12);
    }
}
