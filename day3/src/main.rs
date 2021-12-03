use eyre::{bail, ensure, eyre, Context, Result};

fn main() -> Result<()> {
    println!("--- Day 3 ---");

    let input = include_str!("../input.txt");
    let data = parse(input)?;

    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));

    Ok(())
}

type Input = usize;
type Output = usize;

fn parse(input: &str) -> Result<Vec<Input>> {
    input
        .lines()
        .map(|line| {
            usize::from_str_radix(line, 2).wrap_err_with(|| format!("could not parse: {}", line))
        })
        .zip(1..)
        .map(|(res, lineno): (Result<Input>, usize)| {
            res.wrap_err_with(|| format!("could not parse line {}", lineno))
        })
        .collect()
}

fn ones(data: &[Input]) -> [usize; usize::BITS as _] {
    let mut ones = [0usize; usize::BITS as _];

    for report in data {
        for (i, count) in ones.iter_mut().enumerate() {
            if report & (1 << i) != 0 {
                *count += 1;
            }
        }
    }

    ones
}

fn compress(data: &[Input]) -> (usize, usize) {
    let mut gamma = 0;
    let mut epsilon = 0;
    let threshold = data.len() / 2;

    for count in ones(data).into_iter().rev() {
        gamma <<= 1;
        epsilon <<= 1;

        if count > threshold {
            gamma |= 1;
        } else if count > 0 {
            epsilon |= 1;
        }
    }

    (gamma, epsilon)
}

fn part1(data: &[Input]) -> Output {
    let (gamma, epsilon) = compress(data);
    eprintln!("gamma = {:b}, epsilon = {:b}", gamma, epsilon);
    gamma * epsilon
}

fn compute(sorted_data: &[Input], most: bool) -> Input {
    let mut part = sorted_data;
    let mut b: usize = 1 << (usize::BITS - 1);

    for bit in (0..usize::BITS).rev() {
        if part.len() == 1 {
            break;
        }

        let p = part.partition_point(|&x| x & (1 << bit) == 0);

        if p == part.len() {
            continue;
        }

        let zeros_at_bit = &part[..p];
        let ones_at_bit = &part[p..];

        if most {
            if ones_at_bit.len() >= zeros_at_bit.len() {
                part = ones_at_bit;
            } else {
                part = zeros_at_bit;
            }
        } else {
            if zeros_at_bit.len() <= ones_at_bit.len() {
                part = zeros_at_bit;
            } else {
                part = ones_at_bit;
            }
        }
    }
    assert_eq!(part.len(), 1);

    part[0]
}

fn part2(data: &[Input]) -> Output {
    let mut data = data.to_vec();
    data.sort_unstable();

    let o2generation = compute(&data, true);
    let co2scrubbing = compute(&data, false);

    eprintln!("o2generation = {:b}, co2scrubbing = {:b}", o2generation, co2scrubbing);
    o2generation * co2scrubbing
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const SAMPLE: &str = indoc! {"
        00100
        11110
        10110
        10111
        10101
        01111
        00111
        11100
        10000
        11001
        00010
        01010
    "};

    #[test]
    fn parses_the_input() {
        assert_eq!(
            parse(SAMPLE).unwrap(),
            vec![
                0b00100, 0b11110, 0b10110, 0b10111, 0b10101, 0b01111, 0b00111, 0b11100, 0b10000,
                0b11001, 0b00010, 0b01010,
            ]
        );
    }

    #[test]
    fn solves_the_first_example() {
        let data = parse(SAMPLE).unwrap();
        assert_eq!(part1(&data), 22 * 9);
    }

    #[test]
    fn solves_the_second_example() {
        let data = parse(SAMPLE).unwrap();
        assert_eq!(part2(&data), 23 * 10);
    }
}
