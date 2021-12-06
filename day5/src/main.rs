use std::fmt::Display;

use eyre::{eyre, Result, WrapErr};
use regex::Regex;

fn main() -> Result<()> {
    println!("--- Day 5: Hydrothermal Venture ---");

    let input = include_str!("../input.txt");
    let lines = parse(input)?;

    println!("Straight line overlaps: {}", straight_overlaps(&lines));
    println!("Any line overlaps: {}", more_complete_overlaps(&lines));

    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Line {
    p1: Point,
    p2: Point,
}

impl Line {
    fn horizontal(&self) -> bool {
        self.p1.y == self.p2.y
    }

    fn vertical(&self) -> bool {
        self.p1.x == self.p2.x
    }

    fn at_45_degrees(&self) -> bool {
        (self.p2.x - self.p1.x).abs() == (self.p2.y - self.p1.y).abs()
    }
}

fn parse(input: &str) -> Result<Vec<Line>> {
    let re = Regex::new(r"^(\d+),(\d+) -> (\d+),(\d+)$").expect("could not build regex");

    input
        .lines()
        .map(|line| {
            let caps = re.captures(line).ok_or_else(|| eyre!("invalid format"))?;

            let coord = |i| {
                caps.get(i)
                    .expect("missing capture group")
                    .as_str()
                    .parse()
                    .wrap_err_with(|| format!("could not parse coordinate {}", i))
            };

            Ok(Line {
                p1: Point {
                    x: coord(1)?,
                    y: coord(2)?,
                },
                p2: Point {
                    x: coord(3)?,
                    y: coord(4)?,
                },
            })
        })
        .zip(1..)
        .map(|(res, lineno): (Result<_>, usize)| {
            res.wrap_err_with(|| format!("could not parse line {}", lineno))
        })
        .collect()
}

// Internally uses column-major order to match how the diagram is supposed to look.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Map(Vec<Vec<u16>>);

impl Map {
    fn new() -> Map {
        Map(vec![vec![0; 1024]; 1024])
    }

    fn add_straight_line(&mut self, line: &Line) {
        debug_assert!(line.horizontal() || line.vertical());

        for x in line.p1.x.min(line.p2.x)..=line.p1.x.max(line.p2.x) {
            for y in line.p1.y.min(line.p2.y)..=line.p1.y.max(line.p2.y) {
                let (x, y) = (x as usize, y as usize);
                self.0[y][x] = self.0[y][x].saturating_add(1);
            }
        }
    }

    fn add_line(&mut self, line: &Line) {
        if line.vertical() {
            self.add_straight_line(line);
        } else {
            debug_assert!(line.horizontal() || line.at_45_degrees());

            let m = (line.p2.y - line.p1.y) / (line.p2.x - line.p1.x);
            let b = line.p1.y - m * line.p1.x;

            for x in line.p1.x.min(line.p2.x)..=line.p1.x.max(line.p2.x) {
                let y = m * x + b;

                let (x, y) = (x as usize, y as usize);
                self.0[y][x] = self.0[y][x].saturating_add(1);
            }
        }
    }

    fn overlaps(&self) -> usize {
        let mut count = 0;

        for col in self.0.iter() {
            for &x in col {
                if x >= 2 {
                    count += 1;
                }
            }
        }

        count
    }
}

fn straight_overlaps(lines: &[Line]) -> usize {
    let mut map = Map::new();

    for line in lines {
        if line.horizontal() || line.vertical() {
            map.add_straight_line(line);
        }
    }

    map.overlaps()
}

fn more_complete_overlaps(lines: &[Line]) -> usize {
    let mut map = Map::new();

    for line in lines {
        debug_assert!(line.horizontal() || line.vertical() || line.at_45_degrees());
        map.add_line(line);
    }

    map.overlaps()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const SAMPLE: &str = indoc! {"
        0,9 -> 5,9
        8,0 -> 0,8
        9,4 -> 3,4
        2,2 -> 2,1
        7,0 -> 7,4
        6,4 -> 2,0
        0,9 -> 2,9
        3,4 -> 1,4
        0,0 -> 8,8
        5,5 -> 8,2
    "};

    #[test]
    fn parses_sample() {
        assert_eq!(
            parse(SAMPLE).unwrap(),
            vec![
                Line {
                    p1: Point { x: 0, y: 9 },
                    p2: Point { x: 5, y: 9 }
                },
                Line {
                    p1: Point { x: 8, y: 0 },
                    p2: Point { x: 0, y: 8 }
                },
                Line {
                    p1: Point { x: 9, y: 4 },
                    p2: Point { x: 3, y: 4 }
                },
                Line {
                    p1: Point { x: 2, y: 2 },
                    p2: Point { x: 2, y: 1 }
                },
                Line {
                    p1: Point { x: 7, y: 0 },
                    p2: Point { x: 7, y: 4 }
                },
                Line {
                    p1: Point { x: 6, y: 4 },
                    p2: Point { x: 2, y: 0 }
                },
                Line {
                    p1: Point { x: 0, y: 9 },
                    p2: Point { x: 2, y: 9 }
                },
                Line {
                    p1: Point { x: 3, y: 4 },
                    p2: Point { x: 1, y: 4 }
                },
                Line {
                    p1: Point { x: 0, y: 0 },
                    p2: Point { x: 8, y: 8 }
                },
                Line {
                    p1: Point { x: 5, y: 5 },
                    p2: Point { x: 8, y: 2 }
                },
            ]
        );
    }

    #[test]
    fn solves_part1_with_sample() {
        let lines = parse(SAMPLE).unwrap();

        assert_eq!(straight_overlaps(&lines), 5);
    }

    #[test]
    fn solves_part2_with_sample() {
        let lines = parse(SAMPLE).unwrap();

        assert_eq!(more_complete_overlaps(&lines), 12);
    }

    #[test]
    fn does_not_regress() {
        let input = include_str!("../input.txt");
        let lines = parse(input).unwrap();

        assert_eq!(straight_overlaps(&lines), 5835);
        assert_eq!(more_complete_overlaps(&lines), 17013);
    }
}
