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

fn parse(s: &str) -> (&str, HashMap<&str, &str>) {
    let mut lines = s.lines();

    let template = lines.next().expect("missing template");

    assert_eq!(lines.next(), Some(""), "missing empty separator line");

    let mut rules = HashMap::new();

    for line in lines {
        let (pair, insert) = line.split_once(" -> ").expect("missing ` -> `");

        assert_eq!(pair.chars().count(), 2, "pair must consist of two elements");
        assert_eq!(insert.chars().count(), 1, "insert exactly one element");

        rules.insert(pair, insert);
    }

    (template, rules)
}

fn polymerize(template: &str, rules: &HashMap<&str, &str>, steps: usize) -> String {
    // We take advantage of knowing that all elements are (uppercase) ASCII letters, which allows
    // us to use `.as_bytes().windows(2)`; by asserting that this hypothesis is indeed true, we can
    // simply unwrap (or even use unchecked versions of) calls to `from_utf8`
    assert!(template.is_ascii(), "only ASCII elements are supported");

    // We also need to check that the rules only use ASCII
    assert!(rules.iter().all(|(&k, &v)| k.is_ascii() && v.is_ascii()));

    let mut pre = template.as_bytes();

    // Only use two buffers, alternating between them
    let mut buf = vec![];
    let mut out = vec![];

    for _step in 0..steps {
        let mut first = true;

        buf.truncate(0);
        buf.reserve(pre.len() * 2 - 1);

        for pair in pre.windows(2) {
            let insert = rules[std::str::from_utf8(pair).unwrap()].as_bytes()[0];

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

    let most_common = counts.values().max().unwrap();
    let least_common = counts.values().min().unwrap();
    most_common - least_common
}

fn score_fast(template: &str, rules: &HashMap<&str, &str>, steps: usize) -> usize {
    // We take advantage of knowing that all elements are (uppercase) ASCII letters, which allows
    // us to use `.as_bytes().windows(2)`; by asserting that this hypothesis is indeed true, we can
    // simply unwrap (or even use unchecked versions of) calls to `from_utf8`
    assert!(template.is_ascii(), "only ASCII elements are supported");

    // We also need to check that the rules only use ASCII
    assert!(rules.iter().all(|(&k, &v)| k.is_ascii() && v.is_ascii()));

    // First, count all pairs in the `template` polymer
    let mut pairs: HashMap<(u8, u8), usize> = HashMap::new();
    for pair in template.as_bytes().windows(2) {
        let count = pairs.entry((pair[0], pair[1])).or_default();
        *count += 1;
    }

    // Next, go over each polymerization step, according to the `rules`, but only keep track of how
    // many of each pair of elements is in the polymer
    for _step in 0..steps {
        let mut new_pairs = HashMap::new();
        for (&pair, &count) in pairs.iter() {
            let pair_arr = [pair.0, pair.1];
            let pair_str = std::str::from_utf8(&pair_arr).unwrap();

            let insert = rules[pair_str].as_bytes()[0];

            let left = (pair.0, insert);
            let right = (insert, pair.1);

            for new in [left, right] {
                let new_count = new_pairs.entry(new).or_default();
                *new_count += count;
            }
        }
        pairs = new_pairs;
    }

    // Count the how many times the *first* element of each pair appears; this is *almost* the
    // final element count...
    let mut elements: HashMap<u8, usize> = HashMap::new();
    for (pair, count) in pairs {
        let elem_count = elements.entry(pair.0).or_default();
        *elem_count += count;
    }

    // ...except for the very last element in the polymer chain, which we can just take from the
    // original template
    *elements
        .entry(*template.as_bytes().last().unwrap())
        .or_default() += 1;

    // Finally, find the most and least common elements and compute the score
    let most_common = elements.values().max().unwrap();
    let least_common = elements.values().min().unwrap();
    most_common - least_common
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
        assert_eq!(score_fast(template, &rules, 10), 2797);
    }
}
