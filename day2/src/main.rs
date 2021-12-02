fn main() {
    println!("--- Day 2: ---");

    let input = include_str!("../input.txt");
    let data = parse(input);

    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}

type Data = Vec<String>;

fn parse(input: &str) -> Data {
    input.lines().map(|line| line.to_owned()).collect()
}

fn part1(data: &Data) -> usize {
    todo!()
}

fn part2(data: &Data) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const SAMPLE: &'static str = indoc! {"
    "};

    #[test]
    fn parses_the_input() {
        assert_eq!(parse(SAMPLE), Vec::<String>::new());
    }

    #[test]
    fn solves_the_first_example() {
        let data = parse(SAMPLE);
        assert_eq!(part1(&data), 42);
    }

    #[test]
    fn solves_the_second_example() {
        let data = parse(SAMPLE);
        assert_eq!(part2(&data), 42);
    }
}
