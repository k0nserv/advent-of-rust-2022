use lazy_static::lazy_static;
use std::collections::{HashMap, HashSet};

lazy_static! {
    static ref PRIORITIES: HashMap<char, u64> = {
        let lowercase = ('a'..='z').enumerate().map(|(i, c)| (c, i as u64 + 1));
        let uppercase = ('A'..='Z').enumerate().map(|(i, c)| (c, i as u64 + 27));

        lowercase.chain(uppercase).collect()
    };
}

pub fn star_one(input: &str) -> u64 {
    input
        .lines()
        .map(str::trim)
        .filter(|l| l.len() > 0)
        .map(|l| {
            let (left, right) = l.split_at(l.len() / 2);
            let left_chars = to_chars(left);
            let right_chars = to_chars(right);
            let intersection = left_chars.intersection(&right_chars);

            intersection.map(priority).sum::<u64>()
        })
        .sum()
}

pub fn star_two(input: &str) -> u64 {
    let sacks: Vec<_> = input
        .lines()
        .map(str::trim)
        .filter(|l| l.len() > 0)
        .map(to_chars)
        .collect();

    sacks
        .chunks(3)
        .map(|group| {
            let mut in_common = group[0].clone();

            for g in &group[1..] {
                in_common = in_common.intersection(g).copied().collect();
            }

            assert!(in_common.len() == 1);
            let in_common = in_common.drain().next().unwrap();

            priority(&in_common)
        })
        .sum()
}

fn to_chars(s: &str) -> HashSet<char> {
    s.chars().collect()
}

fn priority(c: &char) -> u64 {
    *PRIORITIES
        .get(&c)
        .unwrap_or_else(|| panic!("No priority for {}", c))
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};

    const TEST: &'static str = r#"
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
"#;

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(TEST), 157);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(TEST), 70);
    }
}
