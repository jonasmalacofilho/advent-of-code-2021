use std::{collections::BTreeMap, fmt::Debug, str::FromStr};

use eyre::{ensure, eyre, Report, Result};

const INPUT: &str = include_str!("../input.txt");

fn main() -> Result<()> {
    println!("--- Day 8: Seven Segment Search ---");

    let entries = parse(INPUT)?;
    println!("Easy digits: {}", count_easy_digits(&entries));
    println!("Output sum: {}", sum_values(&entries)?);

    Ok(())
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Pattern(u8);

impl Pattern {
    fn new(inner: u8) -> Pattern {
        Pattern(inner)
    }

    fn count(&self) -> u32 {
        self.0.count_ones()
    }

    fn union(&self, other: Pattern) -> Pattern {
        Pattern::new(self.0 | other.0)
    }
}

impl FromStr for Pattern {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut numeric = 0;

        for c in s.bytes() {
            ensure!(
                (b'a'..=b'g').contains(&c),
                "unknown segment: {}",
                char::from(c)
            );

            let bit = c - b'a';
            numeric |= 1 << bit;
        }

        Ok(Pattern::new(numeric))
    }
}

impl Debug for Pattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:07b}", self.0))
    }
}

type Entry = (Vec<Pattern>, Vec<Pattern>);

fn parse(s: &str) -> Result<Vec<Entry>> {
    s.lines()
        .map(|line| {
            let (patterns, outputs) = line
                .split_once('|')
                .ok_or_else(|| eyre!("missing | separator between patterns and outputs"))?;

            let patterns: Result<_> = patterns.split_whitespace().map(Pattern::from_str).collect();
            let outputs: Result<_> = outputs.split_whitespace().map(Pattern::from_str).collect();

            Ok((patterns?, outputs?))
        })
        .collect()
}

fn count_easy_digits(entries: &[Entry]) -> usize {
    entries
        .iter()
        .map(|(_, outputs)| {
            outputs
                .iter()
                .filter(|pat| [2, 4, 3, 7].contains(&pat.count()))
                .count()
        })
        .sum()
}

fn sum_values(entries: &[Entry]) -> Result<i32> {
    entries
        .iter()
        .map(|(patterns, outputs)| {
            let decoder = decode(patterns)?;

            let mut value: i32 = 0;

            for pat in outputs.iter() {
                let digit: i32 = decoder[pat].into();

                value *= 10;
                value += digit;
            }

            Ok(value)
        })
        .sum()
}

fn decode(patterns: &[Pattern]) -> Result<BTreeMap<Pattern, u8>> {
    let one = *patterns
        .iter()
        .find(|pat| pat.count() == 2)
        .ok_or_else(|| eyre!("missing 2-segment pattern for one"))?;
    let four = *patterns
        .iter()
        .find(|pat| pat.count() == 4)
        .ok_or_else(|| eyre!("missing 4-segment pattern for four"))?;

    let mut decoder = BTreeMap::new();

    // Don't mind the specific segments, use the already known one and four patterns and match how
    // many segments are on in each case: digit, digit ∪ one, digit ∪ four
    //
    // Unions (∪) are used for no special reason other than the name "union" being short;
    // intersections (∩) or symmetric differences (⊖) would work just as well, provided that the
    // match arms are adjusted accordingly
    for pat in patterns {
        let digit = match (pat.count(), pat.union(one).count(), pat.union(four).count()) {
            (2, _, _) => 1,
            (3, _, _) => 7,
            (4, _, _) => 4,
            (5, 5, _) => 3,
            (5, 6, 6) => 5,
            (5, 6, 7) => 2,
            (6, 6, 6) => 9,
            (6, 6, 7) => 0,
            (6, 7, _) => 6,
            (7, _, _) => 8,
            _ => unreachable!(),
        };

        decoder.insert(*pat, digit);
    }

    ensure!(decoder.len() == 10, "not enough patterns");
    Ok(decoder)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const SAMPLE: &str = indoc! {"
        be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
        edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
        fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
        fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
        aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
        fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
        dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
        bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
        egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
        gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
    "};

    #[test]
    fn finds_the_easy_digits() {
        let entries = parse(SAMPLE).unwrap();

        assert_eq!(count_easy_digits(&entries), 26);
    }

    #[test]
    fn decodes_all_digits() {
        let entries = parse(SAMPLE).unwrap();

        assert_eq!(sum_values(&entries).unwrap(), 61229);
    }

    #[test]
    fn does_not_regress() {
        let entries = parse(INPUT).unwrap();

        assert_eq!(count_easy_digits(&entries), 530);
        assert_eq!(sum_values(&entries).unwrap(), 1051087);
    }
}
