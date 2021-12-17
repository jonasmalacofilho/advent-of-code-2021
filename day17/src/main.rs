use std::ops::RangeInclusive;

fn main() {
    println!("--- Day 17: Trick Shot ---");

    let start = (0, 0);
    let target = (248..=285, -85..=-56);

    dbg!(optimal_velocity_naive(start, &target));
    dbg!(velocities_on_target_naive(start, &target).count());
}

fn horizontal_step(x0: i32, vx0: i32) -> impl Iterator<Item = i32> {
    let mut x = x0;
    let mut vx = vx0;
    let dvx = if vx0 > 0 { 1 } else { -1 };

    std::iter::from_fn(move || {
        if vx != 0 {
            x += vx;
            vx -= dvx;
        }
        Some(x)
    })
}

fn vertical_step(y0: i32, vy0: i32) -> impl Iterator<Item = i32> {
    let mut y = y0;
    let mut vy = vy0;

    std::iter::from_fn(move || {
        y += vy;
        vy -= 1;
        Some(y)
    })
}

fn step(p0: (i32, i32), v0: (i32, i32)) -> impl Iterator<Item = (i32, i32)> {
    horizontal_step(p0.0, v0.0).zip(vertical_step(p0.1, v0.1))
}

fn step_on_target(step: (i32, i32), target: &(RangeInclusive<i32>, RangeInclusive<i32>)) -> bool {
    target.0.contains(&step.0) && target.1.contains(&step.1)
}

fn on_target(
    start_position: (i32, i32),
    start_velocity: (i32, i32),
    target: &(RangeInclusive<i32>, RangeInclusive<i32>),
    max_steps: usize,
) -> bool {
    step(start_position, start_velocity)
        .take(max_steps)
        .any(|step| step_on_target(step, target))
}

// HACK:
//
// Hard code limits that have been found, by experimentation, to be suitable for the given
// sample and input.
//
// There is some logic to their values, which could be incorporated in a future version of the
// function responsible for finding velocities that are on target:
//
//  - the maximum number of steps should be larger than both horizontal and vertical spans
//  from start position to the farthest coordinate (in that dimension) of the target region;
//
//  - the range of horizontal velocities to test should be close to the horizontal span from start
//  position to the farthest coordinate of the target region;
//
//  - the range of vertical velocities to test should allow for both high absolute vx (where the
//  absolute vertical velocity is like to be low) as well as high maximum y  (where the absolute
//  horizontal velocity is likely to be low, and where the path will reach heights greater than
//  both the start and target heights).
const NAIVE_MAX_STEPS: usize = 300;
const NAIVE_VX0_RANGE: RangeInclusive<i32> = -0..=300;
const NAIVE_VY0_RANGE: RangeInclusive<i32> = -100..=100;

fn velocities_on_target_naive(
    start_position: (i32, i32),
    target: &(RangeInclusive<i32>, RangeInclusive<i32>),
) -> impl Iterator<Item = (i32, i32)> {
    let target = target.clone();

    NAIVE_VX0_RANGE
        .map(move |vx0| {
            let target = target.clone();
            NAIVE_VY0_RANGE.filter_map(move |vy0| {
                if on_target(start_position, (vx0, vy0), &target, NAIVE_MAX_STEPS) {
                    Some((vx0, vy0))
                } else {
                    None
                }
            })
        })
        .flatten()
}

fn optimal_velocity_naive(
    start_position: (i32, i32),
    target: &(RangeInclusive<i32>, RangeInclusive<i32>),
) -> (i32, i32, i32) {
    velocities_on_target_naive(start_position, target)
        .map(|(vx0, vy0)| {
            let max_height = step(start_position, (vx0, vy0))
                .take(NAIVE_MAX_STEPS)
                .map(|(_, y)| y)
                .max()
                .unwrap();
            (vx0, vy0, max_height)
        })
        .max_by_key(|&(_, _, max_height)| max_height)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn some_horizontal_steps() {
        let steps: Vec<_> = horizontal_step(0, 7).take(7).collect();
        assert_eq!(steps, vec![7, 13, 18, 22, 25, 27, 28]);
    }

    #[test]
    fn some_vertical_steps() {
        let steps: Vec<_> = vertical_step(0, 2).take(7).collect();
        assert_eq!(steps, vec![2, 3, 3, 2, 0, -3, -7]);
    }

    #[test]
    fn some_steps() {
        let steps: Vec<_> = step((0, 0), (7, 2)).take(7).collect();
        assert_eq!(
            steps,
            vec![
                (7, 2),
                (13, 3),
                (18, 3),
                (22, 2),
                (25, 0),
                (27, -3),
                (28, -7)
            ]
        );
    }

    #[test]
    fn optimal_velocity_for_sample() {
        let start = (0, 0);
        let target = (20..=30, -10..=-5);

        let (vx0, vy0, max_height) = optimal_velocity_naive(start, &target);
        assert_eq!(max_height, 45);

        assert!((6..=7).contains(&vx0));
        assert_eq!(vy0, 9);
    }

    #[test]
    fn velocies_on_target_for_sampe() {
        let start = (0, 0);
        let target = (20..=30, -10..=-5);
        assert_eq!(velocities_on_target_naive(start, &target).count(), 112);
    }

    #[test]
    fn does_not_regress_on_part1() {
        let start = (0, 0);
        let target = (248..=285, -85..=-56);

        let (vx0, vy0, max_height) = optimal_velocity_naive(start, &target);
        assert_eq!(max_height, 3570);

        assert!((22..=23).contains(&vx0));
        assert_eq!(vy0, 84);
    }

    #[test]
    fn does_not_regress_on_part2() {
        let start = (0, 0);
        let target = (248..=285, -85..=-56);
        assert_eq!(velocities_on_target_naive(start, &target).count(), 1919);
    }
}
