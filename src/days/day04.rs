use aoc_lib::{Bench, BenchResult, Day, NoError, ParseResult, UserError};
use color_eyre::{Report, Result};

pub const DAY: Day = Day {
    day: 4,
    name: "Printing Department",
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Paper,
}

#[derive(Debug, Clone)]
struct Map {
    tiles: Vec<Tile>,
    height: usize,
    width: usize,
}

impl Map {
    fn try_get_tile(&self, x: usize, y: usize) -> Option<Tile> {
        if x >= self.width || y >= self.height {
            return None;
        }

        let idx = y * self.width + x;
        Some(self.tiles[idx])
    }

    fn get_tile(&self, x: usize, y: usize) -> Tile {
        self.try_get_tile(x, y).unwrap_or_else(|| {
            panic!(
                "coordinates out of bounds: {x}x{y}, (bounds are {}x{})",
                self.width, self.height
            )
        })
    }

    fn get_neigbours_8_dir(&self, x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> {
        #[rustfmt::skip]
        const RELATIVES: [(isize, isize); 8] = [
            (-1, -1), (0, -1), (1, -1),
            (-1, 0),           (1, 0),
            (-1, 1),  (0, 1),  (1, 1),
        ];

        RELATIVES
            .into_iter()
            .flat_map(move |(rx, ry)| -> Option<(usize, usize)> {
                Some((x.checked_add_signed(rx)?, y.checked_add_signed(ry)?))
            })
            .filter(|&(x, y)| x < self.width && y < self.height)
    }
}

fn parse(input: &str) -> Result<Map> {
    let mut tiles = Vec::new();
    let mut width = 0;
    let mut height = 0;

    for row in input.lines() {
        height += 1;
        width = row.len();

        for &ch in row.as_bytes() {
            tiles.push(if ch == b'.' { Tile::Empty } else { Tile::Paper });
        }
    }

    Ok(Map {
        width,
        height,
        tiles,
    })
}

fn part1(data: &Map) -> usize {
    let mut count = 0;

    for y in 0..data.height {
        for x in 0..data.width {
            if data.get_tile(x, y) == Tile::Empty {
                continue;
            }

            let num_nb = data
                .get_neigbours_8_dir(x, y)
                .filter(|&(x, y)| data.get_tile(x, y) == Tile::Paper)
                .count();

            count += (num_nb < 4) as usize
        }
    }

    count
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
        let expected = 13;
        let actual = part1(&parsed);

        assert_eq!(expected, actual);
    }
}
