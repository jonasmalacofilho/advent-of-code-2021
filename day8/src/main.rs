use std::{collections::BTreeMap, fmt::Debug};

use indoc::indoc;

fn main() {
    println!("Hello, world!");

    // let input = "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
    let input = indoc! {"
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
    let input = include_str!("../input.txt");

    let entries = parse(input);
    // dbg!(&entries);

    let part1: usize = entries
        .iter()
        .map(|(_, outputs)| {
            outputs
                .iter()
                .filter(|pat| [2, 4, 3, 7].contains(&pat.active_count()))
                .count()
        })
        .sum();
    dbg!(part1);

    let part2: i32 = entries
        .iter()
        .map(|(patterns, outputs)| {
            let one = *patterns.iter().find(|pat| pat.active_count() == 2).unwrap();
            let four = *patterns.iter().find(|pat| pat.active_count() == 4).unwrap();

            let mut decoder = BTreeMap::new();
            decoder.insert(one, 1);
            decoder.insert(four, 4);

            for pat in patterns {
                decoder.insert(
                    *pat,
                    match (pat.active_count(), pat.union(one).active_count(), pat.union(four).active_count()) {
                        (2|4, _, _) => continue,
                        (3, _, _) => 7,
                        (5, 5, _) => 3,
                        (5, 6, 6) => 5,
                        (5, 6, 7) => 2,
                        (6, 6, 6) => 9,
                        (6, 6, 7) => 0,
                        (6, 7, _) => 6,
                        (7, _, _) => 8,
                        _ => unreachable!()
                    },
                );
            }

            dbg!(decoder.len());
            dbg!(&decoder);

            let mut value: i32 = 0;

            for digit in outputs
                .iter()
                .inspect(|x| println!("{:?}", x))
                .map(|pat| decoder[pat])
            {
                value *= 10;
                value += digit;
            }

            value
        })
        .inspect(|x| println!("{}", x))
        .sum();
    dbg!(part2);
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Pattern(u8);

impl Pattern {
    fn new(s: &str) -> Pattern {
        let mut numeric = 0;

        for c in s.bytes() {
            assert!((b'a'..=b'g').contains(&c));

            let bit = c - b'a';
            numeric |= 1 << bit;
        }

        Pattern(numeric)
    }

    fn active_count(&self) -> u8 {
        self.0.count_ones() as _
    }

    fn union(&self, other: Pattern) -> Pattern {
        Pattern(self.0 | other.0)
    }

    fn intersection(&self, other: Pattern) -> Pattern {
        Pattern(self.0 & other.0)
    }
}

impl Debug for Pattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:07b}", self.0))
    }
}

type Entry = (Vec<Pattern>, Vec<Pattern>);

fn parse(s: &str) -> Vec<Entry> {
    s.lines()
        .map(|line| {
            let (patterns, outputs) = line.split_once('|').unwrap();
            (
                patterns.split_whitespace().map(Pattern::new).collect(),
                outputs.split_whitespace().map(Pattern::new).collect(),
            )
        })
        .collect()
}
