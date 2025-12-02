use aoc_lib::{Bench, BenchResult, Day, NoError, ParseResult, UserError};
use color_eyre::{Report, Result, eyre::bail};

pub const DAY: Day = Day {
    day: 2,
    name: "Gift Shop",
    part_1: run_part1,
    part_2: Some(run_part2),
    other: &[("Parse", run_parse)],
};

fn run_part1(input: &str, b: Bench) -> BenchResult {
    let data = parse(input).map_err(UserError)?;
    b.bench(|| Ok::<_, NoError>(part1(&data)))
}

fn run_part2(input: &str, b: Bench) -> BenchResult {
    let data = parse(input).map_err(UserError)?;
    b.bench(|| Ok::<_, NoError>(part2(&data)))
}

fn run_parse(input: &str, b: Bench) -> BenchResult {
    b.bench(|| {
        let data = parse(input).map_err(UserError)?;
        Ok::<_, Report>(ParseResult(data))
    })
}

#[derive(Debug, Copy, Clone)]
struct ProductIdRange {
    start: u64,
    end: u64,
}

fn parse(input: &str) -> Result<Vec<ProductIdRange>> {
    input
        .trim()
        .split(',')
        .map(|r| -> Result<ProductIdRange> {
            let Some((left, right)) = r.split_once('-') else {
                bail!("Invalid input: {r:?}");
            };

            Ok(ProductIdRange {
                start: left.parse()?,
                end: right.parse()?,
            })
        })
        .collect()
}

fn get_divisors_p1(num_digits: u32) -> u64 {
    match num_digits {
        2 => 10,
        4 => 100,
        6 => 1000,
        8 => 10000,
        10 => 100000,
        12 => 1000000,
        14 => 10000000,
        16 => 100000000,
        18 => 1000000000,
        20 => 10000000000,
        _ => 1,
    }
}

fn is_valid_p1(id: u64) -> bool {
    let num_digits = id.ilog10() + 1;
    let divisor = get_divisors_p1(num_digits);
    let upper = id / divisor;
    let lower = id % divisor;

    upper != lower
}

fn part1(ranges: &[ProductIdRange]) -> u64 {
    let mut sum = 0;

    for range in ranges {
        sum += (range.start..=range.end)
            .filter(|&id| !is_valid_p1(id))
            .sum::<u64>()
    }

    sum
}

fn get_divisors_p2(num_digits: u32) -> &'static [u64] {
    match num_digits {
        1 | 2 | 3 | 5 | 7 | 11 | 13 | 17 | 19 => &[10],
        4 => &[10, 100],
        6 => &[10, 100, 1000],
        8 => &[10, 100, 10000],
        9 => &[10, 1000],
        10 => &[10, 100, 100000],
        12 => &[10, 100, 1000, 10000, 1000000],
        14 => &[10, 100, 10000000],
        15 => &[10, 1000, 100000],
        16 => &[10, 100, 10000, 100000000],
        18 => &[10, 100, 1000, 1000000, 1000000000],
        20 => &[10, 100, 10000, 100000, 10000000000],
        _ => unreachable!("{num_digits}"),
    }
}

fn is_valid_p2(id: u64) -> bool {
    if id < 10 {
        return true;
    }
    let num_digits = id.ilog10() + 1;
    let divisors = get_divisors_p2(num_digits);

    let mut all_divisors_valid = true;

    'divisor: for &divisor in divisors {
        let mut id = id;

        let first_part = id % divisor;
        id /= divisor;

        while id > 0 {
            let next = id % divisor;
            id /= divisor;
            if next != first_part {
                continue 'divisor;
            }
        }

        all_divisors_valid = false;
    }

    all_divisors_valid
}

fn part2(ranges: &[ProductIdRange]) -> u64 {
    let mut sum = 0;

    for range in ranges {
        sum += (range.start..=range.end)
            .filter(|&id| !is_valid_p2(id))
            .sum::<u64>()
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_lib::Example;

    #[test]
    fn part1_test() {
        let data = aoc_lib::input(DAY.day)
            .example(Example::Part1, 1)
            .open()
            .unwrap();

        let parsed = parse(&data).unwrap();
        let expected = 1227775554;
        let actual = part1(&parsed);

        assert_eq!(expected, actual);
    }

    #[test]
    fn part2_test() {
        let data = aoc_lib::input(DAY.day)
            .example(Example::Part1, 1)
            .open()
            .unwrap();

        let parsed = parse(&data).unwrap();
        let expected = 4174379265;
        let actual = part2(&parsed);

        assert_eq!(expected, actual);
    }

    #[test]
    fn part2_valid_check() {
        let tests = [
            (11, false),
            (22, false),
            (99, false),
            (111, false),
            (1188511885, false),
            (222222, false),
            (446446, false),
            (38593859, false),
            (565656, false),
            (284284284, false),
            (2121212121, false),
            (100, true),
            (12341234, false),
            (123123123, false),
            (1212121212, false),
            (1111111, false),
        ];

        for (i, &(id, expected)) in tests.iter().enumerate() {
            assert_eq!(is_valid_p2(id), expected, "{i}: {id}");
        }
    }
}
