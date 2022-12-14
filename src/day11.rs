use std::collections::HashMap;

pub fn star_one(mut monkeys: Vec<Monkey>) -> u64 {
    solve(monkeys, 20, false)
}

pub fn star_two(mut monkeys: Vec<Monkey>) -> u64 {
    solve(monkeys, 10_000, true)
}

fn solve(mut monkeys: Vec<Monkey>, rounds: usize, worried: bool) -> u64 {
    let mut inspect_counts: HashMap<usize, u64> = HashMap::new();

    for _ in 1..=rounds {
        do_round(&mut monkeys, worried, |monkey| {
            *inspect_counts.entry(monkey).or_default() += 1;
        });
    }

    let sorted = {
        let mut v: Vec<_> = inspect_counts.into_values().collect();
        v.sort_by(|a, b| b.cmp(a));

        v
    };

    sorted.into_iter().take(2).product()
}

fn do_round<F>(monkeys: &mut [Monkey], worried: bool, mut callback: F)
where
    F: FnMut(usize),
{
    let div_mod = monkeys
        .iter()
        .fold(1, |acc, monkey| acc * monkey.test.operand);

    for i in 0..monkeys.len() {
        for j in 0..monkeys[i].items.len() {
            let item = monkeys[i].items[j];
            let new_value = monkeys[i].inspect(item, worried, div_mod);
            let new_monkey = monkeys[i].test.outcome(new_value);

            monkeys[new_monkey].items.push(new_value);

            // Monkey inspected an item
            callback(i);
        }

        monkeys[i].items.clear();
    }
}

#[derive(Debug, Clone)]
pub struct Monkey {
    id: usize,
    items: Vec<u64>,
    operation: Operation,
    test: Test,
}

impl Monkey {
    pub fn new(id: usize, items: Vec<u64>, operation: Operation, test: Test) -> Self {
        Self {
            id,
            items,
            operation,
            test,
        }
    }

    fn inspect(&self, item: u64, worried: bool, div_mod: u64) -> u64 {
        if worried {
            self.operation.apply(item, div_mod)
        } else {
            self.operation.apply(item, div_mod) / 3
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Operation {
    MulOld(u64),
    AddOld(u64),
    MulOldSelf,
    AddOldSelf,
}

impl Operation {
    fn apply(self, item: u64, div_mod: u64) -> u64 {
        use Operation::*;
        match self {
            MulOld(arg) => (item * arg) % div_mod,
            AddOld(arg) => (item + arg) % div_mod,
            MulOldSelf => (item * item) % div_mod,
            AddOldSelf => (item + item) % div_mod,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Test {
    operand: u64,
    true_target: usize,
    false_target: usize,
}

impl Test {
    pub fn new(operand: u64, true_target: usize, false_target: usize) -> Self {
        Self {
            operand,
            true_target,
            false_target,
        }
    }

    fn outcome(&self, item: u64) -> usize {
        if item % self.operand == 0 {
            self.true_target
        } else {
            self.false_target
        }
    }
}

pub mod prelude {
    pub use super::{star_one, star_two, Monkey, Operation, Test};
}

macro_rules! monkey {
    ({
        Monkey $n:literal:
        Starting items: $($items:literal),+
        Operation: new = old * $op_arg:literal
        Test: divisible by $div_arg:literal
            If true: throw to monkey $true_target:literal
            If false: throw to monkey $false_target:literal
    }) => {
        Monkey::new(
            $n,
            vec![$($items),+],
            Operation::MulOld($op_arg),
            Test::new(
                     $div_arg,
            $true_target,
            $false_target
            )
        )
    };
    ({
        Monkey $n:literal:
        Starting items: $($items: literal),+
        Operation: new = old * old
        Test: divisible by $div_arg:literal
            If true: throw to monkey $true_target:literal
            If false: throw to monkey $false_target:literal
    }) => {
        Monkey::new(
            $n,
            vec![$($items),+],
            Operation::MulOldSelf,
            Test::new(
                     $div_arg,
            $true_target,
            $false_target
            )
        )
    };
    ({
        Monkey $n:literal:
        Starting items: $($items: literal),+
        Operation: new = old + $op_arg: literal
        Test: divisible by $div_arg:literal
            If true: throw to monkey $true_target:literal
            If false: throw to monkey $false_target:literal
    }) => {
        Monkey::new(
            $n,
            vec![$($items),+],
            Operation::AddOld($op_arg),
            Test::new(
                     $div_arg,
            $true_target,
            $false_target
            )
        )
    };
    ({
        Monkey $n:literal:
        Starting items: $($items: literal),+
        Operation: new = old + old
        Test: divisible by $div_arg:literal
            If true: throw to monkey $true_target:literal
            If false: throw to monkey $false_target:literal
    }) => {
        Monkey::new(
            $n,
            vec![$($items),+],
            Operation::AddOldSelf,
            Test::new(
                     $div_arg,
            $true_target,
            $false_target
            )
        )
    };
}

macro_rules! monkeys {
    ($(
            $monkey: tt
          )*) => {
            vec![
                $(
                    monkey!($monkey)
                ),*
            ]
    };
}

#[cfg(test)]
mod tests {
    use super::{star_one, star_two, Monkey, Operation, Test};

    fn test_monkeys() -> Vec<Monkey> {
        monkeys!(
            {
            Monkey 0:
              Starting items: 79, 98
              Operation: new = old * 19
              Test: divisible by 23
                If true: throw to monkey 2
                If false: throw to monkey 3
            }

            {
            Monkey 1:
              Starting items: 54, 65, 75, 74
              Operation: new = old + 6
              Test: divisible by 19
                If true: throw to monkey 2
                If false: throw to monkey 0
            }

            {
            Monkey 2:
              Starting items: 79, 60, 97
              Operation: new = old * old
              Test: divisible by 13
                If true: throw to monkey 1
                If false: throw to monkey 3
            }

            {
            Monkey 3:
              Starting items: 74
              Operation: new = old + 3
              Test: divisible by 17
                If true: throw to monkey 0
                If false: throw to monkey 1
            }
        )
    }

    #[test]
    fn test_star_one() {
        assert_eq!(star_one(test_monkeys()), 10605)
    }

    #[test]
    fn test_star_two() {
        assert_eq!(star_two(test_monkeys()), 2713310158);
    }
}
