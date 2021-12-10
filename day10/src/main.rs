use eyre::{bail, Result};

const INPUT: &str = include_str!("../input.txt");

fn main() -> Result<()> {
    println!("--- Day 10: Syntax Scoring ---");

    let lines = parse(INPUT)?;

    println!("Error score: {}", error_score(&lines));
    println!("Completion median score: {}", complete_middle_score(&lines));

    Ok(())
}

const ALLOWED: &[u8] = b"()[]{}<>";

fn parse(s: &str) -> Result<Vec<&[u8]>> {
    s.lines()
        .map(|line| {
            let line = line.as_bytes();

            for c in line {
                if !ALLOWED.contains(c) {
                    bail!("unsupported character: {}", char::from(*c));
                }
            }

            Ok(line)
        })
        .collect()
}

fn close(c: u8) -> u8 {
    match c {
        b'(' => b')',
        b'[' => b']',
        b'{' => b'}',
        b'<' => b'>',
        _ => unreachable!(),
    }
}

enum Analysis {
    IllegalTerminator(u8),
    MissingTerminators(Vec<u8>),
    Fine,
}

fn analyze<'a>(lines: &'a [&'a [u8]]) -> impl Iterator<Item = Analysis> + 'a {
    lines.iter().map(|&line| {
        let mut stack = vec![];

        for &c in line {
            if b"([{<".contains(&c) {
                stack.push(c);
            } else if b")]}>".contains(&c) {
                let open = stack.pop();
                if open.is_none() || c != close(open.unwrap()) {
                    return Analysis::IllegalTerminator(c);
                }
            } else {
                panic!("unsupported character: {}", char::from(c));
            }
        }

        if !stack.is_empty() {
            let missing = stack.into_iter().rev().map(close).collect();
            return Analysis::MissingTerminators(missing);
        }

        Analysis::Fine
    })
}

fn error_score(lines: &[&[u8]]) -> usize {
    analyze(lines)
        .map(|analysis| match analysis {
            Analysis::IllegalTerminator(c) => match c {
                b')' => 3,
                b']' => 57,
                b'}' => 1197,
                b'>' => 25137,
                _ => panic!("unsupported character: {}", char::from(c)),
            },
            _ => 0,
        })
        .sum()
}

fn complete_middle_score(lines: &[&[u8]]) -> usize {
    let mut scores: Vec<_> = analyze(lines)
        .filter_map(|analysis| match analysis {
            Analysis::MissingTerminators(missing) => {
                let score = missing.iter().fold(0, |acc, &c| {
                    let pvalue = match c {
                        b')' => 1,
                        b']' => 2,
                        b'}' => 3,
                        b'>' => 4,
                        _ => panic!("unsupported character: {}", char::from(c)),
                    };
                    acc * 5 + pvalue
                });

                Some(score)
            }
            _ => None,
        })
        .collect();

    scores.sort_unstable();
    scores[scores.len() / 2]
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const SAMPLE: &str = indoc! {"
        [({(<(())[]>[[{[]{<()<>>
        [(()[<>])]({[<{<<[]>>(
        {([(<{}[<>[]}>{[]{[(<()>
        (((({<>}<{<{<>}{[]{[]{}
        [[<[([]))<([[{}[[()]]]
        [{[{({}]{}}([{[{{{}}([]
        {<[[]]>}<{[{[{[]{()[[[]
        [<(<(<(<{}))><([]([]()
        <{([([[(<>()){}]>(<<{{
        <{([{{}}[<[[[<>{}]]]>[]]
    "};

    #[test]
    fn calcutes_error_scores() {
        let lines = parse(SAMPLE).unwrap();

        assert_eq!(error_score(&lines), 26397);
    }

    #[test]
    fn calcutes_complete_scores() {
        let lines = parse(SAMPLE).unwrap();

        assert_eq!(complete_middle_score(&lines), 288957);
    }

    #[test]
    fn does_not_regress() {
        let lines = parse(INPUT).unwrap();

        assert_eq!(error_score(&lines), 387363);
        assert_eq!(complete_middle_score(&lines), 4330777059);
    }
}
