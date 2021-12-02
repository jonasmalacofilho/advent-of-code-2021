fn main() {
    println!("--- Day 2: Dive! ---");

    let input = include_str!("../input.txt");
    let data = parse(input);

    println!("Final position: {}", final_position(&data));
    println!("Final position, fixed: {}", final_position_fixed(&data));
}

#[derive(Debug, PartialEq, Eq)]
enum Command {
    Forward(isize),
    Down(isize),
    Up(isize),
}

fn parse(input: &str) -> Vec<Command> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(' ');

            let command = parts.next().expect("missing command");
            let amount = parts
                .next()
                .expect("missing amount")
                .parse::<isize>()
                .expect("cannot parse amount");

            assert_eq!(parts.next(), None, "found trailing data");

            match command {
                "forward" => Command::Forward(amount),
                "down" => Command::Down(amount),
                "up" => Command::Up(amount),
                _ => panic!("unknown command: {}", command),
            }
        })
        .collect()
}

fn final_position(data: &[Command]) -> isize {
    let mut hor = 0;
    let mut dep = 0;

    for cmd in data {
        match cmd {
            Command::Forward(x) => hor += x,
            Command::Down(x) => dep += x,
            Command::Up(x) => dep -= x,
        }
    }

    dbg!(hor, dep);
    hor * dep
}

fn final_position_fixed(data: &[Command]) -> isize {
    let mut hor = 0;
    let mut dep = 0;
    let mut aim = 0;

    for cmd in data {
        match cmd {
            Command::Forward(x) => {
                hor += x;
                dep += aim * x;
            }
            Command::Down(x) => aim += x,
            Command::Up(x) => aim -= x,
        }
    }

    dbg!(hor, dep, aim);
    hor * dep
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
        use Command::*;

        assert_eq!(
            parse(SAMPLE),
            vec![Forward(5), Down(5), Forward(8), Up(3), Down(8), Forward(2)]
        );
    }

    #[test]
    fn solves_the_first_example() {
        let data = parse(SAMPLE);
        assert_eq!(final_position(&data), 150);
    }

    #[test]
    fn solves_the_second_example() {
        let data = parse(SAMPLE);
        assert_eq!(final_position_fixed(&data), 900);
    }
}
