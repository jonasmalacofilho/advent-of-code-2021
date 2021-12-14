use std::{
    collections::{BinaryHeap, HashMap},
    time::{Duration, Instant},
};

const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("Hello, world!");

    let (template, rules) = parse(INPUT);

    dbg!(time(|| score(polymerize(template, &rules, 10).chars())));
    dbg!(time(|| score(polymerize_mem_bounded(template, &rules, 10))));
    dbg!(time(|| score(polymerize_mem_bounded(template, &rules, 40))));
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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Pair {
    step: u32,
    ord: u64,
    left: char,
    right: char,
}

fn polymerize_mem_bounded(
    template: &str,
    rules: &HashMap<&str, u8>,
    steps: usize,
) -> impl Iterator<Item = char> {
    let steps: u32 = steps as _;

    let mut better_sules = HashMap::new();
    for (&pair, &insert) in rules {
        let mut chars = pair.chars();
        let left = chars.next().unwrap();
        let right = chars.next().unwrap();
        better_sules.insert((left, right), insert.into());
    }
    let rules = better_sules;

    let mut ord = u64::MAX;
    let mut heap = BinaryHeap::new();
    for (left, right) in template.chars().zip(template.chars().skip(1)) {
        heap.push(Pair {
            step: 0,
            ord,
            left,
            right,
        });
        ord -= 1;
    }

    std::iter::from_fn(move || {
        while let Some(Pair {
            step, left, right, ..
        }) = heap.pop()
        {
            if step >= steps {
                if heap.is_empty() && step == steps {
                    heap.push(Pair {
                        step: u32::MAX,
                        ord: 0,
                        left: right,
                        right,
                    });
                }
                return Some(left);
            } else {
                let insert = rules[&(left, right)];
                heap.push(Pair {
                    step: step + 1,
                    ord,
                    left,
                    right: insert,
                });
                heap.push(Pair {
                    step: step + 1,
                    ord: ord - 1,
                    left: insert,
                    right,
                });
                ord -= 2;
            }
        }
        None
    })
}

fn score(polymer: impl Iterator<Item = char>) -> usize {
    let mut counts: HashMap<char, usize> = HashMap::new();

    for element in polymer {
        // dbg!(element);
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
        assert_eq!(score(polymerize(template, &rules, 10).chars()), 1588);
        assert_eq!(score(polymerize_mem_bounded(template, &rules, 10)), 1588);
    }

    #[test]
    #[ignore] // requires, effectively, infinite time
    fn sample_polymer_after_40_steps() {
        let (template, rules) = parse(SAMPLE);
        assert_eq!(
            score(polymerize_mem_bounded(template, &rules, 40)),
            2188189693529
        );
    }

    #[test]
    fn does_not_regres() {
        let (template, rules) = parse(INPUT);
        assert_eq!(score(polymerize(template, &rules, 10).chars()), 2797);
    }
}
