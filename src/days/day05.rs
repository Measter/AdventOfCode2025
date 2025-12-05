use aoc_lib::{Bench, BenchResult, Day, NoError, ParseResult, UserError};
use color_eyre::{Report, Result, eyre::eyre};

pub const DAY: Day = Day {
    day: 5,
    name: "Cafeteria",
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

#[derive(Debug, Clone, Copy)]
struct Range {
    start: u64,
    end: u64,
}

impl Range {
    fn new(start: u64, end: u64) -> Self {
        Self { start, end }
    }

    fn contains(self, x: u64) -> bool {
        (self.start..=self.end).contains(&x)
    }
}

struct Data {
    ranges: Vec<Range>,
    ids: Vec<u64>,
}

fn parse(input: &str) -> Result<Data> {
    let (ranges, ids) = input
        .split_once("\n\n")
        .ok_or_else(|| eyre!("invalid input"))?;

    let ranges = ranges
        .lines()
        .map(|r| -> Result<Range> {
            let (start, end) = r
                .split_once('-')
                .ok_or_else(|| eyre!("invalid range: {r:?}"))?;
            Ok(Range::new(start.parse()?, end.parse()?))
        })
        .collect::<Result<_>>()?;

    let ids = ids.lines().map(|l| Ok(l.parse()?)).collect::<Result<_>>()?;

    Ok(Data { ranges, ids })
}

fn part1(data: &Data) -> usize {
    data.ids
        .iter()
        .copied()
        .filter(|&id| data.ranges.iter().any(|r| r.contains(id)))
        .count()
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
        let expected = 3;
        let actual = part1(&parsed);

        assert_eq!(expected, actual);
    }
}
