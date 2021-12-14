use std::{collections::HashMap, str};

const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("Hello, world!");

    let (template, rules) = parse(INPUT);
    dbg!(score(&polymerize(template, &rules, 10)));
    // dbg!(score(&polymerize(template, &rules, 40)));
}

fn parse(s: &str) -> (&str, HashMap<&str, u8>) {
    let mut lines = s.lines();

    let template = lines.next().expect("missing template");

    assert_eq!(lines.next().unwrap(), "");

    let mut rules = HashMap::new();

    for line in lines {
        let (pair, insert) = line.split_once(" -> ").expect("missing ` -> `");
        assert_eq!(insert.as_bytes().len(), 1);
        rules.insert(pair, insert.as_bytes()[0]);
    }

    (template, rules)
}

fn polymerize(template: &str, rules: &HashMap<&str, u8>, steps: usize) -> String {
    // To use `slice::windows()` we take advantage of knowing that all elements are (uppercase)
    // ASCII letters; by asserting this here, we can simply unwrap any later fallible conversions
    // from &[u8]/Vec<u8> to &str/String
    assert!(template.is_ascii());

    let mut pre = template.as_bytes();

    // Only use two buffers, alternating between them
    let mut buf = vec![];
    let mut out = vec![];

    for _step in 0..steps {
        let mut first = true;

        buf.truncate(0);
        buf.reserve(pre.len() * 2 - 1);

        for pair in pre.windows(2) {
            let insert = rules[str::from_utf8(pair).unwrap()];

            if first {
                first = false;
                buf.push(pair[0]);
            }

            buf.push(insert);
            buf.push(pair[1]);
        }

        std::mem::swap(&mut buf, &mut out);
        pre = out.as_slice();
    }

    String::from_utf8(out).unwrap()
}

fn score(polymer: &str) -> usize {
    let mut counts: HashMap<char, usize> = HashMap::new();

    for element in polymer.chars() {
        let count = counts.entry(element).or_default();
        *count += 1;
    }

    let most_common = counts.iter().max_by_key(|&(_, c)| c).unwrap();
    let least_common = counts.iter().min_by_key(|&(_, c)| c).unwrap();

    // dbg!(polymer, &most_common, &least_common);
    most_common.1 - least_common.1
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const SAMPLE: &str = indoc! {"
        NNCB

        CH -> B
        HH -> N
        CB -> H
        NH -> C
        HB -> C
        HC -> B
        HN -> C
        NN -> C
        BH -> H
        NC -> B
        NB -> B
        BN -> B
        BB -> N
        BC -> B
        CC -> N
        CN -> C
    "};

    #[test]
    fn sample_polymer_after_10_steps() {
        let (template, rules) = parse(SAMPLE);
        assert_eq!(score(&polymerize(template, &rules, 10)), 1588);
    }

    #[test]
    fn does_not_regres() {
        let (template, rules) = parse(INPUT);
        assert_eq!(score(&polymerize(template, &rules, 10)), 2797);
    }
}
