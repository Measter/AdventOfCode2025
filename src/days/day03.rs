use aoc_lib::{Bench, BenchResult, Day, NoError, ParseResult, UserError};
use color_eyre::{Report, Result};

pub const DAY: Day = Day {
    day: 3,
    name: "Lobby",
    part_1: run_part1,
    part_2: None,
    other: &[("Parse", run_parse)],
};

fn run_part1(input: &str, b: Bench) -> BenchResult {
    let data = parse(input).map_err(UserError)?;
    b.bench(|| Ok::<_, NoError>(part1(&data)))
}

fn run_parse(input: &str, b: Bench) -> BenchResult {
    b.bench(|| {
        let data = parse(input).map_err(UserError)?;
        Ok::<_, Report>(ParseResult(data))
    })
}

fn parse(input: &str) -> Result<Vec<&[u8]>> {
    Ok(input.lines().map(str::as_bytes).collect())
}

fn as_num(left: u8, right: u8) -> u8 {
    (left - b'0') * 10 + (right - b'0')
}

fn get_joltage(bank: &[u8]) -> u32 {
    let mut current = 0;
    let mut right = *bank.last().unwrap();

    for &left in bank[..bank.len() - 1].iter().rev() {
        let num = as_num(left, right);
        current = current.max(num);
        right = right.max(left);
    }

    current as _
}

fn part1(banks: &[&[u8]]) -> u32 {
    banks.iter().copied().map(get_joltage).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_lib::Example;

    #[test]
    fn get_joltage_test() {
        let tests: [(&str, u32); _] = [
            ("987654321", 98),
            ("819", 89),
            ("234278", 78),
            ("819121", 92),
        ];

        for (i, (test, expected)) in tests.into_iter().enumerate() {
            let actual = get_joltage(test.as_bytes());
            assert_eq!(actual, expected, "{i}: {test:?}");
        }
    }

    #[test]
    fn part1_test() {
        let data = aoc_lib::input(DAY.day)
            .example(Example::Part1, 1)
            .open()
            .unwrap();

        let parsed = parse(&data).unwrap();
        let expected = 357;
        let actual = part1(&parsed);

        assert_eq!(expected, actual);
    }
}
