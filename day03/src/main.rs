use eyre::{Context, Result};

fn main() -> Result<()> {
    println!("--- Day 3: Binary Diagnostic ---");

    let input = include_str!("../input.txt");
    let mut data = parse(input)?;

    println!("Power consumption: {}", power_consumption(&data));
    println!("Life support rating: {}", life_support_rating(&mut data));

    Ok(())
}

type Value = u16;

fn parse(input: &str) -> Result<Vec<Value>> {
    input
        .lines()
        .map(|line| {
            Value::from_str_radix(line, 2).wrap_err_with(|| format!("could not parse: {}", line))
        })
        .zip(1..)
        .map(|(res, lineno): (Result<Value>, usize)| {
            res.wrap_err_with(|| format!("could not parse line {}", lineno))
        })
        .collect()
}

fn power_consumption(data: &[Value]) -> usize {
    let mut gamma = 0;
    let mut epsilon = 0;
    let half = data.len() / 2;

    for count in bit_counts(data).into_iter().rev() {
        gamma <<= 1;
        epsilon <<= 1;

        if count > half {
            gamma |= 1;
        } else if count > 0 {
            epsilon |= 1;
        } else {
            // Bit not in use, ignore
        }
    }

    eprintln!("gamma = {:b}, epsilon = {:b}", gamma, epsilon);
    gamma * epsilon
}

fn bit_counts(data: &[Value]) -> [usize; Value::BITS as _] {
    let mut counts = [0usize; Value::BITS as _];

    for report in data {
        for (bit, count) in counts.iter_mut().enumerate() {
            if report & (1 << bit) != 0 {
                *count += 1;
            }
        }
    }

    counts
}

fn life_support_rating(data: &mut [Value]) -> usize {
    data.sort_unstable();

    let o2generation = find_with(BitCriteria::WithMostCommonBits, data);
    let co2scrubbing = find_with(BitCriteria::WithLeastCommonBits, data);

    eprintln!(
        "o2generation = {:b}, co2scrubbing = {:b}",
        o2generation, co2scrubbing
    );
    o2generation as usize * co2scrubbing as usize
}

enum BitCriteria {
    WithMostCommonBits,
    WithLeastCommonBits,
}

/// Finds values of interest by successively filtering using most/least bit criteria.
///
/// As the name implies, `sorted_data` must be sorted, otherwise the returned value will be
/// meaningless.
fn find_with(criteria: BitCriteria, sorted_data: &[Value]) -> Value {
    let mut rest = sorted_data;

    for bit in (0..Value::BITS).rev() {
        if rest.len() == 1 {
            // Found it, done
            break;
        }

        // Since `rest` is sorted, all values before index `p` have this bit set to zero, and all
        // values with indices equal or above `p` have this bit set to one
        let p = rest.partition_point(|&x| x & (1 << bit) == 0);
        let have_zero = &rest[..p];
        let have_one = &rest[p..];

        if p == rest.len() {
            // Bit not in use, ignore
            continue;
        }

        // Successively pick the partition with this bit set to the most/least common value
        match criteria {
            BitCriteria::WithMostCommonBits => {
                if have_one.len() >= have_zero.len() {
                    rest = have_one;
                } else {
                    rest = have_zero;
                }
            }
            BitCriteria::WithLeastCommonBits => {
                if have_one.len() >= have_zero.len() {
                    rest = have_zero;
                } else {
                    rest = have_one;
                }
            }
        }
    }

    assert_eq!(
        rest.len(),
        1,
        "bit filter exhausted but slice still has more than one element"
    );

    rest[0]
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const SAMPLE: &str = indoc! {"
        00100
        11110
        10110
        10111
        10101
        01111
        00111
        11100
        10000
        11001
        00010
        01010
    "};

    #[test]
    fn parses_the_input() {
        assert_eq!(
            parse(SAMPLE).unwrap(),
            vec![
                0b00100, 0b11110, 0b10110, 0b10111, 0b10101, 0b01111, 0b00111, 0b11100, 0b10000,
                0b11001, 0b00010, 0b01010,
            ]
        );
    }

    #[test]
    fn solves_the_first_example() {
        let data = parse(SAMPLE).unwrap();
        assert_eq!(power_consumption(&data), 22 * 9);
    }

    #[test]
    fn solves_the_second_example() {
        let mut data = parse(SAMPLE).unwrap();
        assert_eq!(life_support_rating(&mut data), 23 * 10);
    }

    #[test]
    fn does_not_regress() {
        let input = include_str!("../input.txt");
        let mut data = parse(input).unwrap();

        assert_eq!(power_consumption(&data), 2724524);
        assert_eq!(life_support_rating(&mut data), 2775870);
    }
}
