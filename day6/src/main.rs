use eyre::{Result, WrapErr};

fn main() -> Result<()> {
    println!("--- Day 6: Lanternfish ---");

    let input = include_str!("../input.txt");
    let mut population = parse(input)?;

    population.simulate(80);

    println!("Laternfish after 80 days: {}", population.count());

    population.simulate(256 - 80);

    println!("Laternfish after 256 days: {}", population.count());

    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Lanternfish {
    timer: u8,
}

impl Lanternfish {
    pub fn new(timer: u8) -> Self {
        Self { timer }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Population {
    timer_counts: [usize; 9],
}

impl Population {
    pub fn new(starting_population: Vec<Lanternfish>) -> Self {
        let mut timer_counts = [0; 9];

        for fish in starting_population {
            timer_counts[fish.timer as usize] += 1;
        }

        Self { timer_counts }
    }

    pub fn simulate(&mut self, days: usize) {
        for _ in 0..days {
            self.timer_counts.rotate_left(1);
            self.timer_counts[6] = self.timer_counts[6]
                .checked_add(self.timer_counts[8])
                .unwrap();
        }
    }

    pub fn count(&self) -> usize {
        self.timer_counts.iter().sum()
    }
}

fn parse(input: &str) -> Result<Population> {
    let fish: Result<Vec<Lanternfish>> = input
        .trim()
        .split(',')
        .map(|s| {
            let timer = s
                .parse::<u8>()
                .wrap_err_with(|| format!("could not parse `{}` as laternfish timer", s));
            Ok(Lanternfish::new(timer?))
        })
        .collect();

    Ok(Population::new(fish?))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "3,4,3,1,2";

    #[test]
    fn parses_sample() {
        let population = parse(SAMPLE).unwrap();

        assert_eq!(
            population,
            Population::new(vec![
                Lanternfish::new(3),
                Lanternfish::new(4),
                Lanternfish::new(3),
                Lanternfish::new(1),
                Lanternfish::new(2),
            ])
        );
    }

    #[test]
    fn solves_part1_with_sample() {
        let mut population = parse(SAMPLE).unwrap();

        population.simulate(18);
        dbg!(&population);
        assert_eq!(population.count(), 26);

        population.simulate(80 - 18);
        dbg!(&population);
        assert_eq!(population.count(), 5934);
    }

    #[test]
    fn handles_large_populations() {
        let mut population = parse(SAMPLE).unwrap();

        population.simulate(256);
        dbg!(&population);
        assert_eq!(population.count(), 26984457539);
    }

    #[test]
    fn does_not_regress() {
        let input = include_str!("../input.txt");
        let mut population = parse(input).unwrap();

        population.simulate(80);
        dbg!(&population);
        assert_eq!(population.count(), 353274);

        population.simulate(256 - 80);
        dbg!(&population);
        assert_eq!(population.count(), 1609314870967);
    }
}
