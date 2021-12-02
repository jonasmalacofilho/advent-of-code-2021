fn main() {
    println!("--- Day 2: ---");

    let input = include_str!("../input.txt");
    let data = parse(input);

    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}

type Data = Vec<(isize, isize)>;

fn parse(input: &str) -> Data {
    input.lines().map(|line| {
        let mut parts = line.split(' ');
        let command = parts.next().unwrap();
        let amount = parts.next().unwrap().parse::<isize>().unwrap();
        assert_eq!(parts.next(), None);

        match command {
            "forward" => (amount, 0),
            "down" => (0, amount),
            "up" => (0, -amount),
            _ => panic!("unknown command: {}", command),
        }



    }).collect()
}

fn part1(data: &Data) -> isize {
    let (h, d): (Vec<_>, Vec<_>) = data.iter().copied().unzip();
    h.into_iter().sum::<isize>() * d.into_iter().sum::<isize>()
}

fn part2(data: &Data) -> isize {
    let mut hor = 0;
    let mut dep = 0;
    let mut aim = 0;

    for (f, a) in data.iter().copied() {
        if f != 0 {
            hor += f;
            dep += aim * f;
        } else {
            aim += a;
        }

        // dbg!((f, a, hor, dep, aim));
    }

    hor * dep
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const SAMPLE: &'static str = indoc! {"
        forward 5
        down 5
        forward 8
        up 3
        down 8
        forward 2
    "};

    #[test]
    fn parses_the_input() {
        assert_eq!(parse(SAMPLE), vec![
            (5, 0),
            (0, 5),
            (8, 0),
            (0, -3),
            (0, 8),
            (2, 0)
        ]);
    }

    #[test]
    fn solves_the_first_example() {
        let data = parse(SAMPLE);
        assert_eq!(part1(&data), 150);
    }

    #[test]
    fn solves_the_second_example() {
        let data = parse(SAMPLE);
        assert_eq!(part2(&data), 900);
    }
}
