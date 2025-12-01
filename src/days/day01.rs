use aoc_lib::{Bench, BenchResult, Day, NoError, ParseResult, UserError};
use color_eyre::{Report, Result, eyre::bail};

pub const DAY: Day = Day {
    day: 1,
    name: "Secret Entrance",
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

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
struct Instruction {
    dir: Direction,
    mag: u16,
}

fn parse(input: &str) -> Result<Vec<Instruction>> {
    let mut instructions = Vec::new();

    for line in input.lines() {
        let [dir @ (b'L' | b'R'), digits @ ..] = line.as_bytes() else {
            bail!("Invalid input: {line:?}");
        };

        let dir = if *dir == b'L' {
            Direction::Left
        } else {
            Direction::Right
        };

        let mag = match digits {
            &[a, b, c] => (a - b'0') as u16 * 100 + (b - b'0') as u16 * 10 + (c - b'0') as u16,
            &[b, c] => (b - b'0') as u16 * 10 + (c - b'0') as u16,
            &[c] => (c - b'0') as u16,
            _ => bail!("Invalid input: {line:?}"),
        };

        instructions.push(Instruction { dir, mag })
    }

    Ok(instructions)
}

fn part1(instructions: &[Instruction]) -> u32 {
    let mut cur_position: u16 = 50;
    let mut times_at_zero = 0;

    for inst in instructions {
        let mag = inst.mag % 100;
        match inst.dir {
            Direction::Left => match cur_position.overflowing_sub(mag) {
                (np, false) => cur_position = np,
                (np, true) => cur_position = np.wrapping_add(100),
            },
            Direction::Right => {
                cur_position = (cur_position + mag) % 100;
            }
        }

        times_at_zero += (cur_position == 0) as u32;
    }

    times_at_zero
}

fn part2(instructions: &[Instruction]) -> u16 {
    let mut cur_position: u16 = 50;
    let mut times_at_zero = 0;

    for inst in instructions {
        if inst.mag == 0 {
            continue;
        }

        let start_at_zero = cur_position == 0;
        let full_cycles = inst.mag / 100;
        let sub_cycle_change = inst.mag % 100;

        let (new_pos, on_or_pass_zero) = match inst.dir {
            Direction::Left => match cur_position.overflowing_sub(sub_cycle_change) {
                (np, false) => (np, (np == 0)),
                (np, true) => (np.wrapping_add(100), true & !start_at_zero),
            },
            Direction::Right => {
                let new_pos = cur_position + sub_cycle_change;
                (new_pos % 100, new_pos >= 100)
            }
        };

        times_at_zero += full_cycles + (on_or_pass_zero as u16);
        cur_position = new_pos;
    }

    times_at_zero
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

    #[test]
    fn part2_test() {
        let data = aoc_lib::input(DAY.day)
            .example(Example::Part1, 1)
            .open()
            .unwrap();

        let parsed = parse(&data).unwrap();
        let expected = 6;
        let actual = part2(&parsed);

        assert_eq!(expected, actual);
    }

    fn left(mag: u16) -> Instruction {
        Instruction {
            dir: Direction::Left,
            mag,
        }
    }

    fn right(mag: u16) -> Instruction {
        Instruction {
            dir: Direction::Right,
            mag,
        }
    }

    #[test]
    fn part2_test2() {
        let tests: [(&[Instruction], u16); _] = [
            (&[], 0),
            (&[left(50)], 1),
            (&[left(125)], 1),
            (&[left(150)], 2),
            (&[left(175)], 2),
            (&[left(50), left(0)], 1),
            (&[right(50)], 1),
            (&[right(125)], 1),
            (&[right(150)], 2),
            (&[right(175)], 2),
            (&[right(50), right(0)], 1),
        ];

        for (i, &(inst, expected)) in tests.iter().enumerate() {
            let actual = part2(&inst);
            assert_eq!(expected, actual, "{i}");
        }
    }

    #[test]
    fn part2_few_example3992() {
        let tests: [(&[Instruction], u16); _] = [(&[right(50), left(50), left(50), right(50)], 2)];

        for (i, &(inst, expected)) in tests.iter().enumerate() {
            let actual = part2(&inst);
            assert_eq!(expected, actual, "{i}");
        }
    }
}
