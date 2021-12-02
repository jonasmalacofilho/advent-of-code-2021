use eyre::{bail, ensure, eyre, Context, Result};

fn main() -> Result<()> {
    println!("--- Day 3 ---");

    let input = include_str!("../input.txt");
    let data = parse(input)?;

    println!("Part 1: {}", part1(&data));
    println!("Part 1: {}", part2(&data));

    Ok(())
}

type Input = usize;
type Output = isize;

fn parse(input: &str) -> Result<Vec<Input>> {
    input
        .lines()
        .map(|line| todo!())
        .zip(1..)
        .map(|(res, lineno): (Result<Input>, usize)| {
            res.wrap_err_with(|| format!("could not parse line {}", lineno))
        })
        .collect()
}

fn part1(data: &[Input]) -> Output {
    todo!()
}

fn part2(data: &[Input]) -> Output {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const SAMPLE: &str = indoc! {"
        forward 5
        down 5
        forward 8
        up 3
        down 8
        forward 2
    "};

    #[test]
    fn parses_the_input() {
        assert_eq!(parse(SAMPLE).unwrap(), vec![todo!()]);
    }

    #[test]
    fn solves_the_first_example() {
        let data = parse(SAMPLE).unwrap();
        assert_eq!(part1(&data), 150);
    }

    // #[test]
    // fn solves_the_second_example() {
    //     let data = parse(SAMPLE).unwrap();
    //     assert_eq!(part2(&data), 900);
    // }
}
