use ndarray::{s, Array2, Axis, Dimension};

const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("--- Day 13: Transparent Origami ---");

    let (paper, folds) = parse(INPUT);

    dbg!(dots_after_folds(&paper, folds.iter().take(1)));

    let final_paper = fold(&paper, folds.iter());
    let code = final_paper.map(|x| if *x != 0 { '█' } else { ' ' });

    println!("{}", code); // reads: UEFZCUCJ
}

type Paper = Array2<u8>; // use [0,1] instead of bool it looks nicer with Debug

#[derive(Debug)]
enum Fold {
    Left(usize), // for x == value
    Up(usize),   // for y == value
}

fn parse(s: &str) -> (Paper, Vec<Fold>) {
    let mut lines = s.lines();

    let mut dots = vec![];
    let mut max_x = 0;
    let mut max_y = 0;

    for line in &mut lines {
        if line.is_empty() {
            break;
        }

        let (x, y) = line.split_once(',').expect("missing comma (`,`)");
        let x: usize = x.parse().expect("invalid x coordinate");
        let y: usize = y.parse().expect("invalid y coordinate");

        dots.push((x, y));

        if x > max_x {
            max_x = x;
        }
        if y > max_y {
            max_y = y;
        }
    }

    let mut paper = Array2::zeros((max_y + 1, max_x + 1));

    for (x, y) in dots {
        paper[[y, x]] = 1;
    }

    let mut folds = vec![];

    for line in lines {
        let fold = line
            .strip_prefix("fold along ")
            .expect(r"missing `fold along ` prefix");

        let (axis, value) = fold.split_once('=').expect("missing equal sign (`=`)");
        let value: usize = value.parse().expect("invalid fold coordinate");

        let fold = match axis {
            "x" => Fold::Left(value),
            "y" => Fold::Up(value),
            _ => panic!("unknown axis: {}", axis),
        };

        folds.push(fold);
    }

    (paper, folds)
}

fn fold<'a>(paper: &Paper, folds: impl Iterator<Item = &'a Fold>) -> Paper {
    let mut paper = paper.clone(); // FIXME avoid unnecessary allocation and copy

    for fold in folds {
        let (a, b) = match fold {
            Fold::Left(x) => (
                paper.slice(s![0.., ..*x]),
                paper.slice(s![0.., *x + 1..;-1]),
            ),
            Fold::Up(y) => (
                paper.slice(s![..*y, 0..]),
                paper.slice(s![*y + 1..;-1, 0..]),
            ),
        };

        let mut fold_result = a.to_owned();

        // The result shape determinated by `a` may be larger than the folded part `b`; when
        // that happens, compute how of `a` is not going to be covered by `b`...
        let uncovered = (a.raw_dim() - b.raw_dim()).into_pattern();

        // ...and only do the dot union on the part that overlaps
        let mut overlap = fold_result.slice_mut(s![uncovered.0.., uncovered.1..]);

        overlap |= &b;
        paper = fold_result;
    }

    paper
}

fn dots_after_folds<'a>(paper: &Paper, folds: impl Iterator<Item = &'a Fold>) -> usize {
    fold(paper, folds).iter().filter(|&&x| x != 0).count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const SAMPLE: &str = indoc! {"
        6,10
        0,14
        9,10
        0,3
        10,4
        4,11
        6,0
        6,12
        4,1
        0,13
        10,12
        3,4
        3,0
        8,4
        1,10
        2,14
        8,10
        9,0

        fold along y=7
        fold along x=5
    "};

    #[test]
    fn dots_after_first_fold() {
        let (paper, folds) = parse(SAMPLE);
        assert_eq!(dots_after_folds(&paper, folds.iter().take(1)), 17);
    }

    #[test]
    fn does_not_regress() {
        let (paper, folds) = parse(INPUT);
        assert_eq!(dots_after_folds(&paper, folds.iter().take(1)), 669);
        // TODO assert the final code (UEFZCUCJ)
    }
}
