use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("Hello, world!");

    let (template, rules) = parse(INPUT);

    dbg!(time(|| score(polymerize(template, &rules, 10).chars())));
    dbg!(time(|| score_fast(template, &rules, 10)));
    dbg!(time(|| score_fast(template, &rules, 40)));
}

fn time<F, T>(f: F) -> (T, Duration)
where
    F: Fn() -> T,
{
    let start = Instant::now();
    let result = f();
    (result, Instant::now() - start)
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
            let insert = rules[std::str::from_utf8(pair).unwrap()];

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

fn score(polymer: impl Iterator<Item = char>) -> usize {
    let mut counts: HashMap<char, usize> = HashMap::new();

    for element in polymer {
        let count = counts.entry(element).or_default();
        *count += 1;
    }

    let most_common = counts.iter().max_by_key(|&(_, c)| c).unwrap();
    let least_common = counts.iter().min_by_key(|&(_, c)| c).unwrap();

    most_common.1 - least_common.1
}

fn score_fast(template: &str, rules: &HashMap<&str, u8>, steps: usize) -> usize {
    // To use `slice::windows()` we take advantage of knowing that all elements are (uppercase)
    // ASCII letters; by asserting this here, we can simply unwrap any later fallible conversions
    // from &[u8]/Vec<u8> to &str/String
    assert!(template.is_ascii());

    let mut pairs: HashMap<(u8, u8), usize> = HashMap::new();

    for pair in template.as_bytes().windows(2) {
        let count = pairs.entry((pair[0], pair[1])).or_default();
        *count += 1;
    }

    for _step in 0..steps {
        let mut new_pairs = HashMap::new();
        for (&pair, &count) in pairs.iter() {
            let pair_arr = [pair.0, pair.1];
            let pair_str = std::str::from_utf8(&pair_arr).unwrap();

            let insert = rules[pair_str];
            let left = (pair.0, insert);
            let right = (insert, pair.1);

            for new in [left, right] {
                let new_count = new_pairs.entry(new).or_default();
                *new_count += count;
            }
        }
        pairs = new_pairs;
    }

    let mut elements: HashMap<char, usize> = HashMap::new();

    for (pair, count) in pairs {
        let elem_count = elements.entry(pair.0.into()).or_default();
        *elem_count += count;
    }

    *elements
        .entry(template.chars().last().unwrap())
        .or_default() += 1;

    let most_common = elements.iter().max_by_key(|&(_, c)| c).unwrap();
    let least_common = elements.iter().min_by_key(|&(_, c)| c).unwrap();

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
        assert_eq!(score(polymerize(template, &rules, 10).chars()), 1588);
        assert_eq!(score_fast(template, &rules, 10), 1588);
    }

    #[test]
    fn sample_polymer_after_40_steps() {
        let (template, rules) = parse(SAMPLE);
        assert_eq!(score_fast(template, &rules, 40), 2188189693529);
    }

    #[test]
    fn does_not_regres() {
        let (template, rules) = parse(INPUT);
        assert_eq!(score(polymerize(template, &rules, 10).chars()), 2797);
        assert_eq!(score_fast(template, &rules, 40), 2926813379532);
    }
}
