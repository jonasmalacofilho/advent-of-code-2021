use std::collections::BinaryHeap;

use eyre::{eyre, Report, Result, WrapErr};

const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("--- Day 9: ---");

    let map = parse(INPUT);

    dbg!(risk(&map));
    dbg!(basins(&map));
}

fn parse(s: &str) -> Vec<Vec<u8>> {
    s.lines()
        .map(|line| line.bytes().map(|b| b - b'0').collect())
        .collect()
}

fn risk(map: &[Vec<u8>]) -> usize {
    let mut acc = 0;

    for i in 0..map.len() {
        for j in 0..map[i].len() {
            let h = map[i][j];
            let t = if i > 0 { map[i - 1][j] } else { u8::MAX };
            let r = if j < map[i].len() - 1 {
                map[i][j + 1]
            } else {
                u8::MAX
            };
            let b = if i < map.len() - 1 {
                map[i + 1][j]
            } else {
                u8::MAX
            };
            let l = if j > 0 { map[i][j - 1] } else { u8::MAX };

            if h < t && h < r && h < b && h < l {
                acc += h as usize + 1;
            }
        }
    }

    acc
}

fn basins(map: &[Vec<u8>]) -> usize {
    // FIXME assumes that map is rectangular
    let mut basins: Vec<Vec<(usize, usize)>> = vec![vec![(0, 0); map[0].len()]; map.len()];
    let mut done = false;

    for i in 0..map.len() {
        for j in 0..map[i].len() {
            basins[i][j] = (i, j);
        }
    }

    while !done {
        done = true;

        for i in 0..map.len() {
            for j in 0..map[i].len() {
                if map[i][j] == 9 {
                    continue;
                }

                let (mut mi, mut mj) = basins[i][j];
                let mut mh = map[mi][mj];

                let t = if i > 0 { map[i - 1][j] } else { u8::MAX };
                let r = if j < map[i].len() - 1 {
                    map[i][j + 1]
                } else {
                    u8::MAX
                };
                let b = if i < map.len() - 1 {
                    map[i + 1][j]
                } else {
                    u8::MAX
                };
                let l = if j > 0 { map[i][j - 1] } else { u8::MAX };

                if t != 9 && t < mh {
                    mh = t;
                    mi = i - 1;
                    mj = j;
                    done = false;
                }

                if r != 9 && r < mh {
                    mh = r;
                    mi = i;
                    mj = j + 1;
                    done = false;
                }

                if b != 9 && b < mh {
                    mh = b;
                    mi = i + 1;
                    mj = j;
                    done = false;
                }

                if l != 9 && l < mh {
                    mh = l;
                    mi = i;
                    mj = j - 1;
                    done = false;
                }

                if basins[mi][mj] != (mi, mj) {
                    done = false;
                }

                basins[i][j] = basins[mi][mj];
            }
        }
    }

    let mut basin_sizes = BinaryHeap::new();

    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] != 9 && basins[i][j] == (i, j) {
                // Is low point, now measure the basin

                let basin_size: usize = basins
                    .iter()
                    .map(|r| r.iter().filter(|&&c| c == (i, j)).count())
                    .sum();

                basin_sizes.push(basin_size);
            }
        }
    }

    basin_sizes.iter().take(3).product()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const SAMPLE: &str = indoc! {"
        2199943210
        3987894921
        9856789892
        8767896789
        9899965678
    "};

    #[test]
    fn part1() {
        let map = parse(SAMPLE);

        assert_eq!(risk(&map), 15);
    }

    #[test]
    fn part2() {
        let map = parse(SAMPLE);

        assert_eq!(basins(&map), 1134);
    }

    #[test]
    fn does_not_regres() {
        let map = parse(INPUT);

        assert_eq!(risk(&map), 489);
        assert_eq!(basins(&map), 1056330);
    }
}
