use std::collections::HashMap;
use std::ops::RangeInclusive;

pub fn star_one(input: &str) -> usize {
    parse(input)
        .filter(|parsed: &Result<_, String>| {
            let (lhs, rhs) = parsed.as_ref().expect("To be able to parse all lines");

            either_fully_contains(lhs, rhs)
        })
        .count()
}

pub fn star_two(input: &str) -> usize {
    parse(input)
        .map(|parsed| {
            let (lhs, rhs) = parsed.expect("To be able to parse all lines");
            let counts = lhs.into_iter().chain(rhs.into_iter()).fold(
                HashMap::<u64, usize>::new(),
                |mut acc, v| {
                    *(acc.entry(v).or_default()) += 1_usize;
                    acc
                },
            );

            usize::from(counts.into_iter().any(|(_, c)| c > 1))
        })
        .sum()
}

fn parse(
    input: &str,
) -> impl Iterator<Item = Result<(RangeInclusive<u64>, RangeInclusive<u64>), String>> + '_ {
    input
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .map(|l| {
            let mut parts = l.split(',');
            let left = parts
                .next()
                .ok_or_else(|| format!("Invalid assignment, missing first part in: `{}`", l))?;
            let right = parts
                .next()
                .ok_or_else(|| format!("Invalid assignment, missing second part in: `{}`", l))?;

            let left_range = parse_range(left)?;
            let right_range = parse_range(right)?;

            Ok((left_range, right_range))
        })
}

fn parse_range(s: &str) -> Result<RangeInclusive<u64>, String> {
    let mut parts = s.split('-');
    let lower_bound = parts
        .next()
        .ok_or_else(|| format!("Invalid range {} missing first part", s))?;
    let upper_bound = parts
        .next()
        .ok_or_else(|| format!("Invalid range {} missing second part", s))?;

    let lower = lower_bound
        .trim()
        .parse()
        .map_err(|e| format!("Failed to parse lower bound: {}", e))?;
    let upper = upper_bound
        .trim()
        .parse()
        .map_err(|e| format!("Failed to parse lower bound: {}", e))?;

    Ok(RangeInclusive::new(lower, upper))
}

trait RangeInclusiveExt {
    fn fully_contains(&self, other: &Self) -> bool;
}
impl<Idx> RangeInclusiveExt for RangeInclusive<Idx>
where
    Idx: PartialOrd,
{
    fn fully_contains(&self, other: &Self) -> bool {
        self.start() <= other.start() && self.end() >= other.end()
    }
}

fn either_fully_contains<Idx: PartialOrd>(
    lhs: &RangeInclusive<Idx>,
    rhs: &RangeInclusive<Idx>,
) -> bool {
    lhs.fully_contains(rhs) || rhs.fully_contains(lhs)
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};
    const TEST: &'static str = r#"
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
"#;

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(TEST), 2);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(TEST), 4);
    }
}
