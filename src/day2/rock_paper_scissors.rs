// https://adventofcode.com/2022/day/2

use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::{anyhow, Error, Result};

trait AllowedInputTypes: TryFrom<String, Error = Error> {}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl AllowedInputTypes for Move {}

impl From<Move> for u64 {
    fn from(m: Move) -> Self {
        match m {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
}

impl TryFrom<String> for Move {
    type Error = Error;

    fn try_from(s: String) -> Result<Self> {
        match s.as_str() {
            "A" | "X" => Ok(Move::Rock),
            "B" | "Y" => Ok(Move::Paper),
            "C" | "Z" => Ok(Move::Scissors),
            _ => Err(anyhow!(format!("Bad move input: {}", s))),
        }
    }
}

impl PartialOrd for Move {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other {
            Some(Ordering::Equal)
        } else if (*self == Move::Rock && *other == Move::Scissors)
            || (*self == Move::Scissors && *other == Move::Paper)
            || (*self == Move::Paper && *other == Move::Rock)
        {
            Some(Ordering::Greater)
        } else if other.partial_cmp(self) == Some(Ordering::Greater) {
            Some(Ordering::Less)
        } else {
            None
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum MoveStrategy {
    Lose,
    Draw,
    Win,
}

impl From<MoveStrategy> for u64 {
    fn from(ms: MoveStrategy) -> Self {
        match ms {
            MoveStrategy::Lose => 0,
            MoveStrategy::Draw => 3,
            MoveStrategy::Win => 6,
        }
    }
}

impl AllowedInputTypes for MoveStrategy {}

impl TryFrom<String> for MoveStrategy {
    type Error = Error;

    fn try_from(s: String) -> Result<Self> {
        match s.as_str() {
            "X" => Ok(MoveStrategy::Lose),
            "Y" => Ok(MoveStrategy::Draw),
            "Z" => Ok(MoveStrategy::Win),
            _ => Err(anyhow!(format!("Bad move strategy input: {}", s))),
        }
    }
}

fn read_file<P0Type, P1Type>(path: &str) -> Result<Vec<(P0Type, P1Type)>>
where
    P0Type: AllowedInputTypes + Copy,
    P1Type: AllowedInputTypes + Copy,
{
    let fp = File::open(path)?;
    let buf_reader = BufReader::new(fp);
    parse_file_contents::<P0Type, P1Type, _>(buf_reader)
}

fn parse_file_contents<P0Type, P1Type, R: BufRead>(buf_reader: R) -> Result<Vec<(P0Type, P1Type)>>
where
    P0Type: AllowedInputTypes + Copy,
    P1Type: AllowedInputTypes + Copy,
{
    let moves = buf_reader
        .lines()
        .map(|line_res| {
            line_res.map_err(Error::from).and_then(|line| {
                let mut line_parts = line
                    .split_ascii_whitespace()
                    .take(2);
                let p0 = line_parts.next().map(|s| P0Type::try_from(s.to_string())).unwrap()?;
                let p1 = line_parts.next().map(|s| P1Type::try_from(s.to_string())).unwrap()?;

                Ok((p0, p1))
            })
        })
        .collect::<Result<Vec<(P0Type, P1Type)>>>();

    moves
}

fn scores(two_player_moves: &[(Move, Move)]) -> (u64, u64) {
    two_player_moves
        .iter()
        .fold((0, 0), |(s0, s1), (m0, m1)| match m0.partial_cmp(m1) {
            Some(Ordering::Greater) => (s0 + 6 + u64::from(*m0), s1 + 0 + u64::from(*m1)),
            Some(Ordering::Equal) => (s0 + 3 + u64::from(*m0), s1 + 3 + u64::from(*m1)),
            Some(Ordering::Less) => (s0 + 0 + u64::from(*m0), s1 + 6 + u64::from(*m1)),
            None => (s0, s1),
        })
}

fn scores_by_strategy(move_and_stratgy: &[(Move, MoveStrategy)]) -> u64 {
    let moves = vec![Move::Rock, Move::Paper, Move::Scissors];

    let gm = |m: &Move| { moves.iter().find(|&x| x > m).unwrap() };
    let lm = |m: &Move| { moves.iter().find(|&x| x < m).unwrap() };

    move_and_stratgy
        .iter()
        .fold(0, |score, (m0, m1)| {
            let p1_move = match m1 {
                MoveStrategy::Lose => lm(m0),
                MoveStrategy::Draw => m0,
                MoveStrategy::Win => gm(m0),
            };

            score + u64::from(*p1_move) + u64::from(*m1)
        })
}

fn main() {
    let moves = read_file("src/day2/input_day2.txt");
    let ans0 = moves.map(|ms| scores(&ms));
    println!("{:?}", ans0);

    let move_and_strategy = read_file("src/day2/input_day2.txt");
    let ans1 = move_and_strategy.map(|ms| scores_by_strategy(&ms));
    println!("{:?}", ans1);
}

#[cfg(test)]
mod tests {
    use crate::{Move, MoveStrategy};

    #[test]
    fn char_to_move_conversion() {
        assert_eq!(Move::try_from(String::from("A")).unwrap(), Move::Rock);
        assert_eq!(Move::try_from(String::from("B")).unwrap(), Move::Paper);
        assert_eq!(Move::try_from(String::from("C")).unwrap(), Move::Scissors);
        assert_eq!(Move::try_from(String::from("X")).unwrap(), Move::Rock);
        assert_eq!(Move::try_from(String::from("Y")).unwrap(), Move::Paper);
        assert_eq!(Move::try_from(String::from("Z")).unwrap(), Move::Scissors);
        assert!(Move::try_from(String::from("no")).is_err());
    }

    #[test]
    fn char_to_move_stratey_conversion() {
        assert_eq!(MoveStrategy::try_from(String::from("X")).unwrap(), MoveStrategy::Lose);
        assert_eq!(MoveStrategy::try_from(String::from("Y")).unwrap(), MoveStrategy::Draw);
        assert_eq!(MoveStrategy::try_from(String::from("Z")).unwrap(), MoveStrategy::Win);
        assert!(Move::try_from(String::from("no")).is_err());
    }

    #[test]
    fn compare_moves() {
        assert!(Move::Rock > Move::Scissors);
        assert!(Move::Scissors > Move::Paper);
        assert!(Move::Paper > Move::Rock);
        assert!(Move::Rock < Move::Paper);
        assert!(Move::Paper < Move::Scissors);
        assert!(Move::Scissors < Move::Rock);
    }

    #[test]
    fn score_sample_run() {
        let game = vec![
            (Move::Rock, Move::Paper),
            (Move::Paper, Move::Rock),
            (Move::Scissors, Move::Scissors),
        ];

        let answer = super::scores(&game);
        assert_eq!(answer, (15, 15));
    }

    #[test]
    fn scores_by_strategy_sample_run() {
        let game = vec![
            (Move::Rock, MoveStrategy::Draw),
            (Move::Paper, MoveStrategy::Lose),
            (Move::Scissors, MoveStrategy::Win),
        ];

        let answer = super::scores_by_strategy(&game);
        assert_eq!(answer, 12);
    }
}
