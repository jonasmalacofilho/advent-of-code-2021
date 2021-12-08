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
    // let input = include_str!("../input.txt");

    let entries = parse(input);
    dbg!(&entries);

    let part1: usize = entries
        .iter()
        .map(|(patterns, outputs)| {
            outputs
                .iter()
                .filter(|value| [2, 4, 3, 7].contains(&value.len()))
                .count()
        })
        .sum();
    dbg!(part1);
}

type Entry<'a> = (Vec<&'a str>, Vec<&'a str>);

fn parse(s: &str) -> Vec<Entry> {
    s.lines()
        .map(|line| {
            let (patterns, outputs) = line.split_once('|').unwrap();
            (
                patterns.split_whitespace().collect(),
                outputs.split_whitespace().collect(),
            )
        })
        .collect()
}
