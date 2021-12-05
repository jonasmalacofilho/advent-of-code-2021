use eyre::{ensure, eyre, Context, Result};

fn main() -> Result<()> {
    println!("--- Day 4: Giant Squid ---");

    let input = include_str!("../input.txt");
    let mut game = parse(input)?;

    println!("Score with winning board: {}", winning_board(&mut game));

    game.reset();

    println!("Score with losing board: {}", losing_board(&mut game));

    Ok(())
}

#[derive(Debug)]
struct Game {
    sequence: Vec<u8>,
    boards: Vec<Board>,
}

impl Game {
    pub fn reset(&mut self) {
        for b in self.boards.iter_mut() {
            b.reset();
        }
    }
}

#[derive(Debug)]
struct Board {
    board: [u8; 25],
    marked: [bool; 25],
    done: Option<u32>,
}

#[derive(Debug)]
enum BoardResult {
    Pending,
    Bingo(u32),
    Done(u32),
}

impl Board {
    pub fn new(board: [u8; 25]) -> Self {
        Board {
            board,
            marked: [false; 25],
            done: None,
        }
    }

    pub fn mark(&mut self, x: u8) -> BoardResult {
        if let Some(score) = self.done {
            return BoardResult::Done(score);
        }

        for (i, &n) in self.board.iter().enumerate() {
            if n == x {
                self.marked[i] = true;
            }
        }

        if self.bingo() {
            let score = self.score(x);
            self.done = Some(score);
            BoardResult::Bingo(score)
        } else {
            BoardResult::Pending
        }
    }

    pub fn reset(&mut self) {
        self.marked = [false; 25];
        self.done = None;
    }

    fn bingo(&self) -> bool {
        for row in 0..5 {
            let i = row * 5;
            if self.marked[i..i + 5].iter().all(|&x| x) {
                return true;
            }
        }

        for col in 0..5 {
            if (0..5).map(|row| self.marked[row * 5 + col]).all(|x| x) {
                return true;
            }
        }

        false
    }

    fn score(&self, winner: u8) -> u32 {
        let mut acc: u32 = 0;

        for (&n, &marked) in self.board.iter().zip(self.marked.iter()) {
            if !marked && n != winner {
                acc += n as u32;
            }
        }

        acc * winner as u32
    }
}

fn parse_sequence<'a>(lines: &mut impl Iterator<Item = (&'a str, usize)>) -> Result<Vec<u8>> {
    lines
        .next()
        .map(|(line, lineno)| {
            line.split(',')
                .map(|x| {
                    x.trim()
                        .parse::<u8>()
                        .wrap_err_with(|| format!("{}: could not parse number: {}", lineno, x))
                })
                .collect()
        })
        .ok_or_else(|| eyre!("no sequence"))?
}

fn parse_blank_line<'a>(lines: &mut impl Iterator<Item = (&'a str, usize)>) -> Result<()> {
    lines
        .next()
        .map(|(blank, lineno)| {
            ensure!(blank.trim().is_empty(), "{}: expected a blank line", lineno);

            Ok(())
        })
        .ok_or_else(|| eyre!("no blank line"))?
}

fn parse_board<'a>(lines: &mut impl Iterator<Item = (&'a str, usize)>) -> Result<Board> {
    let mut board = [0; 25];

    for i in 0..5 {
        let mut nums = lines
            .next()
            .map(|(line, lineno)| {
                line.split_whitespace().map(move |x| {
                    x.parse::<u8>()
                        .wrap_err_with(|| format!("{}: could not parse number: {}", lineno, x))
                })
            })
            .ok_or_else(|| eyre!("incomplete table"))?;

        for j in 0..5 {
            board[i * 5 + j] = nums.next().ok_or_else(|| eyre!("incomplete line"))??;
        }

        ensure!(nums.next().is_none(), "trailing numbers");
    }

    Ok(Board::new(board))
}

fn parse(input: &str) -> Result<Game> {
    let mut lines = input.lines().zip(1..).peekable();

    let sequence = parse_sequence(&mut lines)?;
    let mut boards = vec![];

    while lines.peek().is_some() {
        let _ = parse_blank_line(&mut lines)?;
        let board = parse_board(&mut lines)?;
        boards.push(board);
    }

    Ok(Game { sequence, boards })
}

fn winning_board(game: &mut Game) -> u32 {
    for &x in game.sequence.iter() {
        for game in &mut game.boards {
            if let BoardResult::Bingo(score) = game.mark(x) {
                return score;
            }
        }
    }

    unreachable!()
}

fn losing_board(game: &mut Game) -> u32 {
    let mut last_score = 0;

    for &x in game.sequence.iter() {
        for game in &mut game.boards {
            if let BoardResult::Bingo(score) = game.mark(x) {
                last_score = score;
            }
        }
    }

    last_score
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const SAMPLE: &str = indoc! {"
        7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

        22 13 17 11  0
         8  2 23  4 24
        21  9 14 16  7
         6 10  3 18  5
         1 12 20 15 19

         3 15  0  2 22
         9 18 13 17  5
        19  8  7 25 23
        20 11 10 24  4
        14 21 16 12  6

        14 21 17 24  4
        10 16 15  9 19
        18  8 23 26 20
        22 11 13  6  5
         2  0 12  3  7
    "};

    #[test]
    fn parses_the_input() {
        let game = parse(SAMPLE).unwrap();

        assert_eq!(
            game.sequence,
            vec![
                7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8,
                19, 3, 26, 1
            ]
        );

        assert_eq!(game.boards.len(), 3);

        #[rustfmt::skip]
        assert_eq!(
            game.boards[0].board,
            [
                22, 13, 17, 11,  0,
                 8,  2, 23,  4, 24,
                21,  9, 14, 16,  7,
                 6, 10,  3, 18,  5,
                 1, 12, 20, 15, 19
            ]
        );

        #[rustfmt::skip]
        assert_eq!(
            game.boards[1].board,
            [
                 3, 15,  0,  2, 22,
                 9, 18, 13, 17,  5,
                19,  8,  7, 25, 23,
                20, 11, 10, 24,  4,
                14, 21, 16, 12,  6
            ]
        );

        #[rustfmt::skip]
        assert_eq!(
            game.boards[2].board,
            [
                14, 21, 17, 24,  4,
                10, 16, 15,  9, 19,
                18,  8, 23, 26, 20,
                22, 11, 13,  6,  5,
                 2,  0, 12,  3,  7
            ]
        );
    }

    #[test]
    fn solves_the_first_example() {
        let mut game = parse(SAMPLE).unwrap();
        assert_eq!(winning_board(&mut game), 188 * 24);
    }

    #[test]
    fn solves_the_second_example() {
        let mut game = parse(SAMPLE).unwrap();
        assert_eq!(losing_board(&mut game), 148 * 13);
    }

    #[test]
    fn does_not_regress() {
        let input = include_str!("../input.txt");
        let mut game = parse(input).unwrap();

        assert_eq!(winning_board(&mut game), 2496);
        assert_eq!(losing_board(&mut game), 25925);
    }

    #[test]
    fn does_not_overflow_on_degenerate_case() {
        #[rustfmt::skip]
        let boards = vec![
            Board::new([
                79, 80, 81, 82, 83,
                84, 85, 86, 87, 88,
                89, 90, 91, 92, 93,
                94, 95, 96, 97, 98,
                 0,  1,  2,  3, 99
            ])
        ];
        let sequence = vec![0, 1, 2, 3, 99];

        let mut game = Game { sequence, boards };

        assert_eq!(
            winning_board(&mut game) as usize,
            (79..99).sum::<usize>() * 99
        );

        game.reset();

        assert_eq!(
            losing_board(&mut game) as usize,
            (79..99).sum::<usize>() * 99
        );
    }
}
