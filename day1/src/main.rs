use std::num::ParseIntError;

fn main() -> Result<(), ParseIntError> {
    let input = include_str!("../input.txt");
    let depths = parse_depths(input)?;

    println!("--- Day 1: Sonar Sweep ---");

    println!("Measurements that increased: {}", increased(&depths));

    println!(
        "Measurements that increased (3-measurement window): {}",
        increased_with_window(&depths)
    );

    Ok(())
}

fn parse_depths(input: &str) -> Result<Vec<usize>, ParseIntError> {
    input
        .lines()
        .map(|line| line.parse::<usize>())
        .collect::<Result<Vec<_>, _>>()
}

fn increased(depths: &[usize]) -> usize {
    let mut prev = None;
    let mut increased = 0;

    for depth in depths {
        if let Some(prev) = prev {
            if depth > prev {
                increased += 1;
            }
        }

        prev = Some(depth);
    }

    increased
}

fn increased_with_window(depths: &[usize]) -> usize {
    let mut prev = None;
    let mut increased = 0;

    for ((a, b), c) in depths.iter().zip(&depths[1..]).zip(&depths[2..]) {
        let depth3 = a + b + c;

        if let Some(prev) = prev {
            if depth3 > prev {
                increased += 1;
            }
        }

        prev = Some(depth3);
    }

    increased
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn parses_input() {
        let input = indoc! {"
            199
            200
            208
            210
            200
            207
            240
            269
            260
            263
        "};

        assert_eq!(
            parse_depths(input),
            Ok(vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263,])
        );
    }

    #[test]
    fn example() {
        let depths = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

        assert_eq!(increased(&depths), 7);
    }

    #[test]
    fn example_with_window() {
        let depths = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

        assert_eq!(increased_with_window(&depths), 5);
    }
}
