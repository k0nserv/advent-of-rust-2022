pub fn star_one(input: &str) -> usize {
    let mat = input
        .trim()
        .as_bytes() // Assume ascii
        .windows(4)
        .enumerate()
        .find(|(_, w)| all_different(&w))
        .unwrap();

    mat.0 + 4
}

pub fn star_two(input: &str) -> usize {
    let mat = input
        .trim()
        .as_bytes() // Assume ascii
        .windows(14)
        .enumerate()
        .find(|(_, w)| all_different(&w))
        .unwrap();

    mat.0 + 14
}

fn all_different<T: Eq>(values: &[T]) -> bool {
    for (i, v) in values.iter().enumerate() {
        for u in values[i + 1..].iter() {
            if v == u {
                return false;
            }
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::{all_different, star_one, star_two};

    const STAR_ONE_TEST_CASES: &[(&str, usize)] = &[
        ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7),
        ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5),
        ("nppdvjthqldpwncqszvftbrmjlhg", 6),
        ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10),
        ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11),
    ];

    #[test]
    fn test_star_one() {
        for (input, expected) in STAR_ONE_TEST_CASES {
            assert_eq!(star_one(input), *expected, "Wrong output for {}", input);
        }
    }

    const STAR_TWO_TEST_CASES: &[(&str, usize)] = &[
        ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19),
        ("bvwbjplbgvbhsrlpgdmjqwftvncz", 23),
        ("nppdvjthqldpwncqszvftbrmjlhg", 23),
        ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29),
        ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26),
    ];

    #[test]
    fn test_star_two() {
        for (input, expected) in STAR_TWO_TEST_CASES {
            assert_eq!(star_two(input), *expected, "Wrong output for {}", input);
        }
    }

    #[test]
    fn test_all_different() {
        assert!(!all_different(b"bvwb"));
        assert!(all_different(b"jplb"));
    }
}
