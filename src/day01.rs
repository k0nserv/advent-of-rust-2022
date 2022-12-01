pub fn star_one(input: &str) -> u64 {
    parse(input).max().expect("There should be at least on elf")
}

pub fn star_two(input: &str) -> u64 {
    let mut calories: Vec<u64> = parse(input).collect();

    calories.sort_by(|a, b| b.cmp(a));

    calories.into_iter().take(3).sum()
}

fn parse(input: &str) -> impl Iterator<Item = u64> + '_ {
    input
        .trim()
        .split("\n\n")
        .map(|group| group.lines().flat_map(|l| l.parse::<u64>().ok()).sum())
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two};

    const TEST_INPUT: &'static str = r#"""
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
"""#;

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(TEST_INPUT), 24000);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(TEST_INPUT), 45000);
    }
}
