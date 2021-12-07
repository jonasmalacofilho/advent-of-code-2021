use eyre::{Result, WrapErr};

fn main() -> Result<()> {
    println!("--- Day 7: The Treachery of Whales ---");

    let input = include_str!("../input.txt");
    let crabs = parse(input)?;

    let alignment = align_linear(&crabs);
    dbg!(&alignment);
    println!("Required fuel for alignment: {}", alignment.1);

    let alignment_v2 = align_v2(&crabs);
    dbg!(&alignment_v2);
    println!("Required fuel for alignment, fixed/v2: {}", alignment_v2.1);

    Ok(())
}

fn parse(input: &str) -> Result<Vec<u32>> {
    input
        .split(',')
        .map(|s| {
            s.trim()
                .parse::<u32>()
                .wrap_err_with(|| format!("could not parse `{}` as position", s))
        })
        .collect()
}

fn align(crabs: &[u32], cost: impl Fn(u32, u32) -> u32) -> (u32, u32) {
    if crabs.is_empty() {
        return (0, 0);
    } else if crabs.len() == 1 {
        return (crabs[0], 0);
    }

    let min = *crabs.iter().min().unwrap();
    let max = *crabs.iter().max().unwrap();

    (min..max)
        .map(|pick| {
            let cost = crabs.iter().map(|&x| cost(x, pick)).sum::<u32>();
            (pick, cost)
        })
        .min_by_key(|(_, cost)| *cost)
        .unwrap()
}

fn align_linear(crabs: &[u32]) -> (u32, u32) {
    align(crabs, |x, pick| (x as i32 - pick as i32).abs() as u32)
}

fn align_v2(crabs: &[u32]) -> (u32, u32) {
    align(crabs, |x, pick| {
        let dist = (x as i32 - pick as i32).abs() as u32;
        (1..=dist).sum::<u32>()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn parses_initial_positions() {
        let crabs = parse(SAMPLE).unwrap();

        assert_eq!(crabs, vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14]);
    }

    #[test]
    fn gets_alignment_and_fuel_cost_correct() {
        let crabs = parse(SAMPLE).unwrap();

        assert_eq!(align_linear(&crabs), (2, 37));
    }

    #[test]
    fn gets_alignment_and_fuel_cost_correct_for_real() {
        let crabs = parse(SAMPLE).unwrap();

        assert_eq!(align_v2(&crabs), (5, 168));
    }

    #[test]
    fn does_not_regress() {
        let input = include_str!("../input.txt");
        let crabs = parse(input).unwrap();

        assert_eq!(align_linear(&crabs), (336, 344735));
        // assert_eq!(align_v2(&crabs), (474, 96798233));
    }
}
