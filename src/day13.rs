use itertools::{EitherOrBoth, Itertools};
use std::cmp::Ordering;
use std::fmt::{self, Write};
use std::ops::ControlFlow;
use std::str::FromStr;

pub fn star_one(input: &str) -> usize {
    let pairs = parse(input);

    pairs
        .into_iter()
        .enumerate()
        .filter_map(|(i, p)| p.in_order().then_some((p, i + 1)))
        .map(|(_, i)| i)
        .sum()
}

pub fn star_two(input: &str) -> usize {
    let dividers = [
        Item::List(Item::parse_items("[[2]]").unwrap()),
        Item::List(Item::parse_items("[[6]]").unwrap()),
    ];
    let ordered = {
        let pairs = parse(input);
        let mut list: Vec<_> = pairs
            .into_iter()
            .flat_map(|p| [p.first, p.second].into_iter())
            .collect();
        list.extend_from_slice(&dividers);

        list.sort_by(|a, b| {
            if in_order(a, b) {
                Ordering::Less
            } else if in_order(b, a) {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        });

        list
    };

    let mut iter = ordered
        .into_iter()
        .enumerate()
        .filter_map(|(i, it)| (it == dividers[0] || it == dividers[1]).then_some(i + 1));

    iter.next().unwrap() * iter.next().unwrap()
}

fn parse(input: &str) -> Vec<Pair> {
    input
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .map(Item::parse_items)
        .chunks(2)
        .into_iter()
        .map(|mut iter| {
            let first = iter.next().unwrap();
            let second = iter.next().unwrap();

            first.and_then(|first| {
                second.map(|second| Pair {
                    first: Item::List(first),
                    second: Item::List(second),
                })
            })
        })
        .collect::<Result<Vec<Pair>, String>>()
        .expect("Should be able to parse input")
}

#[derive(Debug)]
struct Pair {
    first: Item,
    second: Item,
}

impl Pair {
    fn in_order(&self) -> bool {
        in_order(&self.first, &self.second)
    }
}

fn in_order(left: &Item, right: &Item) -> bool {
    matches!(
        in_order_recurse(left, right),
        ControlFlow::Break(true) | ControlFlow::Continue(_)
    )
}

fn in_order_recurse(left: &Item, right: &Item) -> ControlFlow<bool, ()> {
    match (left, right) {
        (Item::Integer(l), Item::Integer(r)) if l == r => ControlFlow::Continue(()),
        (Item::Integer(l), Item::Integer(r)) => ControlFlow::Break(l < r),
        (Item::List(l), Item::List(r)) => {
            // Recurs lists
            let result = l.iter().zip_longest(r.iter()).find_map(|x| match x {
                EitherOrBoth::Both(ll, rr) => break_value(in_order_recurse(ll, rr)),
                EitherOrBoth::Left(_) => Some(false), // Right side ran out, not in order
                EitherOrBoth::Right(_) => Some(true), // Left side ran out, in order
            });

            result
                .map(ControlFlow::Break)
                .unwrap_or_else(|| ControlFlow::Continue(()))
        }
        (l @ Item::List(_), Item::Integer(r)) => {
            let rr = Item::List(vec![Item::Integer(*r)]);
            in_order_recurse(l, &rr)
        }
        (Item::Integer(l), r @ Item::List(_)) => {
            let ll = Item::List(vec![Item::Integer(*l)]);
            in_order_recurse(&ll, r)
        }
    }
}
impl FromStr for Pair {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (s1, s2) = s
            .split_once('\n')
            .ok_or_else(|| format!("Invalid packet pair:\n{s}"))?;

        let first = Item::List(Item::parse_items(s1)?);
        let second = Item::List(Item::parse_items(s2)?);

        Ok(Self { first, second })
    }
}

#[derive(Clone, PartialEq, Eq)]
enum Item {
    Integer(u64),
    List(Vec<Item>),
}

impl Item {
    fn parse_items(s: &str) -> Result<Vec<Self>, String> {
        let list = parse_list(s)?;

        Ok(list)
    }
}

impl fmt::Debug for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn recursive_debug(s: &Item, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match s {
                Item::Integer(v) => write!(f, "{v}"),
                Item::List(ls) => {
                    write!(f, "[")?;
                    for (idx, i) in ls.iter().enumerate() {
                        recursive_debug(i, f);
                        if idx + 1 != ls.len() {
                            write!(f, ",")?;
                        }
                    }
                    write!(f, "]")
                }
            }
        }

        recursive_debug(self, f)
    }
}

impl FromStr for Item {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.chars().all(|c| c.is_ascii_digit()) {
            let parsed: u64 = s
                .parse()
                .map_err(|e| format!("Failed to parse `{s}` as u64: {e}"))?;

            Ok(Item::Integer(parsed))
        } else {
            let list = parse_list(s)?;

            Ok(Item::List(list))
        }
    }
}

fn parse_list(s: &str) -> Result<Vec<Item>, String> {
    let s = s.strip_prefix('[').ok_or_else(|| {
        format!("Expected packet sequence to start with [, but didn't find this in: `{s}`")
    })?;
    let s = s
        .strip_suffix(']')
        .ok_or_else(|| "Expected sequence to end with ]".to_string())?;

    let mut result = vec![];
    let mut s = s;
    while !s.is_empty() {
        s = s.trim_start_matches(char::is_whitespace);
        if s.starts_with(|c: char| c.is_ascii_digit()) {
            let mut parts = s.splitn(2, ',');
            let digit = parts
                .next()
                .ok_or_else(|| format!("Failed to parse leadig number from {}", s))?;
            let rest = parts.next().unwrap_or("");

            result.push(digit.parse()?);
            s = rest;
        } else if s.starts_with('[') {
            let (item, rest) = slice_balanced(s, ('[', ']'))
                .ok_or_else(|| format!("Could not find sublist in `{s}`"))?;

            result.push(item.parse()?);

            s = rest;
            if let Some(rest) = s.strip_prefix(',') {
                s = rest;
            }

            assert!(s != "]");
        }
    }

    Ok(result)
}

fn slice_balanced(s: &str, pair: (char, char)) -> Option<(&str, &str)> {
    let mut balance = 0;
    let mut chars = s.char_indices();

    if !chars.next().map(|(_, c)| c == pair.0).unwrap_or(false) {
        return None;
    }
    balance += 1;

    for ((i, c)) in chars {
        match c {
            _ if c == pair.1 => balance -= 1,
            _ if c == pair.0 => balance += 1,
            _ => {}
        }

        if balance == 0 {
            let end = (i + 1).min(s.len() - 1) + 1;
            return Some((&s[0..=i], &s[end..]));
        }
    }

    None
}

fn break_value<B, C>(c: ControlFlow<B, C>) -> Option<B> {
    match c {
        ControlFlow::Break(v) => Some(v),
        ControlFlow::Continue(_) => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &'static str = r#"
        [1,1,3,1,1]
        [1,1,5,1,1]

        [[1],[2,3,4]]
        [[1],4]

        [9]
        [[8,7,6]]

        [[4,4],4,4]
        [[4,4],4,4,4]

        [7,7,7,7]
        [7,7,7]

        []
        [3]

        [[[]]]
        [[]]

        [1,[2,[3,[4,[5,6,7]]]],8,9]
        [1,[2,[3,[4,[5,6,0]]]],8,9]
    "#;

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(INPUT), 13);
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(INPUT), 140);
    }

    #[test]
    fn test_item_parse_list() {
        let result = Item::parse_items("[[1],[2,3,4]]");

        assert!(result.is_ok());
    }

    #[test]
    fn test_item_parse_list_complex() {
        let result = Item::parse_items("[[[[],[6,0,7],[],[5]],9]]");

        assert!(result.is_ok());
    }

    #[test]
    fn test_slice_balanced() {
        let (item, rest) = slice_balanced("[2, 3, 4]", ('[', ']')).expect("Should slice");

        assert_eq!(item, "[2, 3, 4]");
        assert_eq!(rest, "");
    }

    #[test]
    fn test_2() {
        let first = Item::List(Item::parse_items("[[1],[2,3,4]]").expect("left"));
        let second = Item::List(Item::parse_items("[[1],4]").expect("right"));
        let pair = Pair { first, second };

        assert!(pair.in_order());
    }

    #[test]
    fn test_5() {
        let first = Item::List(Item::parse_items("[7,7,7,7]").expect("left"));
        let second = Item::List(Item::parse_items("[7,7,7]").expect("right"));
        let pair = Pair { first, second };

        assert!(!pair.in_order());
    }

    #[test]
    fn extensive_tests() {
        struct Case {
            input: &'static str,
            in_order: bool,
        }
        let cases = &[
            Case {
                input: r#"
                    [5,6,6,7,3]
                    [5,6,6,7]"#,
                in_order: false,
            },
            Case {
                input: r#"
                    [[7, 6, 5], [9, 1]]
                    [[[3, 2, [1, 0, 9, 2, 7], 4, 2], [[4, 10, 3, 4], 6, [0, 4]], [], [9, [1, 0], []]], [], [5, [4, [4, 10, 9, 6, 3], 3], [[8, 2, 8], [10, 7, 7, 1], 10, [], 5], [9], 9], [0, 7, 3, 5, 10]]
"#,
                in_order: false,
            },
            Case {
                input: r#"
                    [[3, [7], [0], 7]]
                    [[5, 2]]
                "#,
                in_order: true,
            },
            Case {
                input: r#"
                    [[[[6, 1, 4, 1], 8, 6, 10, [5, 3, 9, 10]], [[], 4, 0], 9], [4, 1, 7, 6], [[5, [3, 8, 1, 4, 6], 1, []], 7]]
                    [[], [7, [], [1, [6, 10, 5, 3], [1], 7, [9, 0, 5, 0, 5]], [7, 5, 4, 3], [[4, 7, 6], [], 4, [8, 9, 2, 9, 3]]], [[0, [], [8, 4, 8], 1], 4, 0, [6, [8, 10, 1, 8, 2], 3, 6]], []]
                "#,
                in_order: false,
            },
            Case {
                input: r#"
                    [[3, [7], [0], 7]]
                    [[5, 2]]
                "#,
                in_order: true,
            },
            Case {
                input: r#"
                    [[4, 2, [[4, 3], 2, [4, 5, 5, 1, 1]]], [[8, [7], [9, 7], 4, [0, 3, 8, 6]], 0], [5], [2, 2, [10, 10, [3, 6, 10, 2, 5], [], 0]]]
                    [[[[6, 3, 2, 2, 6], 1, [0, 2, 5, 9, 4], [6], [2]], 9, [7, 3]], [[[7], 7, 4, 2, 2], 5, 6, [3], 3]]
                "#,
                in_order: true,
            },
            Case {
                input: r#"
                    [[1, 4, 6, []]]
                    [[1, 3], [2], [6, 6, 0, [5, [], 9]], [[2, [1, 4, 5], [6], [5, 10, 9, 4]], [7, 4, 6, [2, 6, 8, 9], [5, 6]], [[8], []], [[9, 5, 6], []], 8], [[0, [2, 9, 6, 3], [5, 3]]]]
                "#,
                in_order: false,
            },
            Case {
                input: r#"
                    [[[[1, 7, 4, 3, 2], 4, 8, [2, 2, 6], 4], 6, 6], [3, 3, 3, 1, 5], [[1]], [10, []], [2]]
                    [[[], 3], [[], 7], [8, 0, 8], [8, 2]]
                "#,
                in_order: false,
            },
            Case {
                input: r#"
                    [1, 2, 3, 4, 4]
                    [1, 2, 3, 4, [5]]
                "#,
                in_order: true,
            },
            Case {
                input: r#"
                    [[1], 3]
                    [1, 2]
                "#,
                in_order: false,
            },
            Case {
                input: r#"
                    [[1, [2, [10, 8, 2, 1, 1]], 0]]
                    [[[1]], [[[2, 4, 10, 2], []], 3, 8], [9, 3, [5, [3, 0], [0], [4]], 6, [[9, 8, 3, 7], 4, [10, 10, 8], 10, [6, 6]]], [[[3], 7, [], [10, 5]], 0], [5, [[3, 9, 0, 2, 1], 0, [4, 5, 2], [6]]]]
                "#,
                in_order: false,
            },
            Case {
                input: r#"
                    [[[[7], [2, 5], [4, 1, 10, 9]], [[], [6, 0, 2, 1], [0], [7, 0], 9], 8, [6], 9], [4, [], []], [2]]
                    [[7], [[6, 6]]]
                "#,
                in_order: false,
            },
        ];

        for case in cases {
            let mut lines = case.input.lines().map(str::trim).filter(|l| !l.is_empty());
            let pair = Pair {
                first: Item::List(
                    lines
                        .next()
                        .and_then(|l| Item::parse_items(l).ok())
                        .unwrap(),
                ),
                second: Item::List(
                    lines
                        .next()
                        .and_then(|l| Item::parse_items(l).ok())
                        .unwrap(),
                ),
            };

            assert_eq!(
                pair.in_order(),
                case.in_order,
                "Wrong order for {}, expected in_order={}",
                case.input,
                case.in_order
            );
        }
    }
}
