use eyre::{eyre, Report, Result, WrapErr};

const INPUT: &str = include_str!("../input.txt");

fn main() -> Result<()> {
    println!("Hello, world!");

    let lines = parse(INPUT)?;

    dbg!(error_score(&lines));

    Ok(())
}

const ALLOWED: &[u8] = b"()[]{}<>";

fn parse(s: &str) -> Result<Vec<&[u8]>> {
    // FIXME check chars
    s.lines().map(|line| Ok(line.as_bytes())).collect()
}

fn corrupted<'a>(lines: &'a [&'a [u8]]) -> impl Iterator<Item = u8> + 'a {
    lines.iter().zip(1..).filter_map(|(&line, lineno)| {
        let mut stack = vec![];

        for c in line {
            if b"([{<".contains(c) {
                stack.push(c);
            } else if b")]}>".contains(c) {
                let start = stack.pop().unwrap();
                match (start, c) {
                    (b'(', b')') => { /* ok */ },
                    (b'[', b']') => { /* ok */ },
                    (b'{', b'}') => { /* ok */ },
                    (b'<', b'>') => { /* ok */ },
                    _ => return Some(*c)
                };
            } else {
                unreachable!();
            }
        }

        None
    })
}

fn error_score(lines: &[&[u8]]) -> usize {
    corrupted(lines)
        .map(|illegal| match illegal {
            b')' => 3,
            b']' => 57,
            b'}' => 1197,
            b'>' => 25137,
            _ => unreachable!(),
        })
        .sum()
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
}
