use eyre::{Result, WrapErr};

fn main() -> Result<()> {
    println!("--- Day 7: The Treachery of Whales ---");

    let input = include_str!("../input.txt");
    let crabs = parse(input)?;

    println!("Required fuel for alignment: {}", align(&crabs).1);
    println!(
        "Required fuel for alignment, fixed/v2: {}",
        align_v2(&crabs).1
    );

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

fn align(crabs: &[u32]) -> (u32, u32) {
    let min = *crabs.iter().min().unwrap(); // FIXME
    let max = *crabs.iter().max().unwrap(); // FIXME

    (min..max)
        .map(|pick| {
            let cost = crabs
                .iter()
                .map(|&x| (x as i32 - pick as i32).abs())
                .sum::<i32>() as u32;

            (pick, cost)
        })
        .min_by_key(|(_, cost)| *cost)
        .unwrap() // FIXME
}

fn align_v2(crabs: &[u32]) -> (u32, u32) {
    let min = *crabs.iter().min().unwrap(); // FIXME
    let max = *crabs.iter().max().unwrap(); // FIXME

    (min..max)
        .map(|pick| {
            let cost = crabs
                .iter()
                .map(|&x| {
                    let dist = (x as i32 - pick as i32).abs();
                    (1..=dist).sum::<i32>()
                })
                .sum::<i32>() as u32;

            (pick, cost)
        })
        .min_by_key(|(_, cost)| *cost)
        .unwrap() // FIXME
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

        assert_eq!(align(&crabs), (2, 37));
    }

    #[test]
    fn gets_alignment_and_fuel_cost_correct_for_real() {
        let crabs = parse(SAMPLE).unwrap();

        assert_eq!(align_v2(&crabs), (5, 168));
    }
}
