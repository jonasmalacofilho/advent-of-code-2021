use std::{collections::BinaryHeap, str::FromStr};

use eyre::{ensure, Report, Result};

const INPUT: &str = include_str!("../input.txt");

fn main() -> Result<()> {
    println!("--- Day 9: Smoke Basin ---");

    let map = INPUT.parse::<HeightMap>()?.find_basins();

    println!("Risk from low points: {}", total_risk(&map));
    println!("Product of largest basins: {}", basins_product(&map));

    Ok(())
}

#[derive(Debug)]
struct LazyBasins;

#[derive(Debug)]
struct WithBasins(Vec<usize>);

#[derive(Debug)]
struct HeightMap<Basins = LazyBasins> {
    width: usize,
    heights: Vec<u8>,
    basins: Basins,
}

impl HeightMap<LazyBasins> {
    fn new(heights: Vec<u8>, width: usize) -> HeightMap<LazyBasins> {
        assert_eq!(heights.len() % width, 0);
        HeightMap {
            width,
            heights,
            basins: LazyBasins,
        }
    }

    fn find_basins(self) -> HeightMap<WithBasins> {
        let mut basins = vec![];

        // Find the low points and start the basins at them
        for i in 0..self.heights.len() {
            if self.heights[i] == 9 {
                basins.push(i);
                continue;
            }

            let mut low_point = i;

            // top
            if i >= self.width {
                let top = i - self.width;
                if self.heights[top] < self.heights[low_point] {
                    low_point = top;
                }
            }

            // bottom
            if i + self.width < self.heights.len() {
                let bottom = i + self.width;
                if self.heights[bottom] < self.heights[low_point] {
                    low_point = bottom;
                }
            }

            // left
            if i % self.width > 0 {
                let left = i - 1;
                if self.heights[left] < self.heights[low_point] {
                    low_point = left;
                }
            }

            // right
            if i % self.width + 1 < self.width {
                let right = i + 1;
                if self.heights[right] < self.heights[low_point] {
                    low_point = right;
                }
            }

            basins.push(low_point);
        }

        debug_assert!(basins.len() == self.heights.len());

        let mut done = false;

        // Propagate the basins to the remaining points
        while !done {
            done = true;
            for i in 0..basins.len() {
                if basins[i] != basins[basins[i]] {
                    basins[i] = basins[basins[i]];
                    done = false;
                }
            }
        }

        HeightMap {
            width: self.width,
            heights: self.heights,
            basins: WithBasins(basins),
        }
    }
}

#[allow(dead_code)]
impl<Basins> HeightMap<Basins> {
    fn id(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    fn map_width(&self) -> usize {
        self.width
    }

    fn map_height(&self) -> usize {
        self.heights.len() / self.width
    }

    fn height(&self, x: usize, y: usize) -> Option<u8> {
        if x >= self.map_width() || y >= self.map_height() {
            return None;
        }
        Some(self.heights[self.id(x, y)])
    }
}

#[derive(Debug)]
struct LowPoint {
    pub height: u8,
    pub basin_size: usize,
}

impl HeightMap<WithBasins> {
    fn low_points(&self) -> impl Iterator<Item = LowPoint> + '_ {
        let heights = &self.heights;
        let basins = &self.basins.0;

        (0..)
            .zip(heights.iter().zip(basins.iter()))
            .filter_map(|(id, (&height, &basin))| {
                if id != basin || height == 9 {
                    return None;
                }

                let basin_size = basins.iter().filter(|&&b| b == basin).count();

                Some(LowPoint { height, basin_size })
            })
    }
}

impl FromStr for HeightMap<LazyBasins> {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut heights = vec![];
        let mut width = None;

        for line in s.lines() {
            if let Some(width) = width {
                ensure!(
                    line.len() == width,
                    "irregular map: expected width {}, got {}",
                    width,
                    line.len()
                );
            } else {
                width = Some(line.len());
            }

            for b in line.bytes() {
                ensure!(
                    (b'0'..=b'9').contains(&b),
                    "invalid height {}",
                    char::from(b)
                );
                heights.push(b - b'0');
            }
        }

        Ok(HeightMap::new(heights, width.unwrap_or(0)))
    }
}

fn total_risk(map: &HeightMap<WithBasins>) -> usize {
    map.low_points()
        .map(|LowPoint { height, .. }| height as usize + 1)
        .sum()
}

fn basins_product(map: &HeightMap<WithBasins>) -> usize {
    let mut sizes: BinaryHeap<_> = map
        .low_points()
        .map(|LowPoint { basin_size, .. }| basin_size)
        .collect();

    (0..3).map(|_| sizes.pop().unwrap_or(1)).product()
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
        let map = SAMPLE.parse::<HeightMap>().unwrap().find_basins();

        assert_eq!(total_risk(&map), 15);
    }

    #[test]
    fn part2() {
        let map = SAMPLE.parse::<HeightMap>().unwrap().find_basins();

        assert_eq!(basins_product(&map), 1134);
    }

    #[test]
    fn does_not_regres() {
        let map = INPUT.parse::<HeightMap>().unwrap().find_basins();

        assert_eq!(total_risk(&map), 489);
        assert_eq!(basins_product(&map), 1056330);
    }
}
