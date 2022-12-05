// https://adventofcode.com/2022/day/2

use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::vec;

use anyhow::anyhow;

use anyhow::{Error, Result};

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Into<u64> for Move {
    fn into(self) -> u64 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
}

impl TryFrom<char> for Move {
    type Error = anyhow::Error;

    fn try_from(c: char) -> anyhow::Result<Self, anyhow::Error> {
        match c {
            'A' | 'X' => Ok(Move::Rock),
            'B' | 'Y' => Ok(Move::Paper),
            'C' | 'Z' => Ok(Move::Scissors),
            _ => Err(anyhow!(format!("Bad move input: {}", c))),
        }
    }
}

impl PartialOrd for Move {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other {
            Some(Ordering::Equal)
        } else if (*self == Move::Rock && *other == Move::Scissors)
            || (*self == Move::Scissors && *other == Move::Paper)
            || (*self == Move::Paper && *other == Move::Rock) {
            Some(Ordering::Greater)
        } else if other.partial_cmp(self) == Some(Ordering::Greater) {
            Some(Ordering::Less)
        } else {
            None
        }
    }
}

impl Ord for Move {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cmp(other)
    }
}

fn read_file(path: &str) -> anyhow::Result<Vec<(Move, Move)>> {
    let fp = File::open(path)?;
    let buf_reader = BufReader::new(fp);
    parse_file_contents(buf_reader)
}

fn parse_file_contents<R: BufRead>(buf_reader: R) -> Result<Vec<(Move, Move)>> {
    let moves = buf_reader
        .lines()
        .map(|line_res| {
            line_res.map_err(anyhow::Error::from).and_then(|line| {
                let ms: Result<Vec<Move>, _> = line
                    .split_ascii_whitespace()
                    .take(2)
                    .map(|part| {
                        assert!(part.len() == 0);
                        let first_char = part.as_bytes()[0] as char;
                        Move::try_from(first_char)
                    })
                    .collect();

                ms.map(|v| (v[0], v[1]))
            })
        })
        .collect::<Result<Vec<(Move, Move)>>>();

    moves
}

fn scores(two_player_moves: &[(Move, Move)]) -> (u64, u64) {
    let mut score_board = vec![0 as u64, 0];

    two_player_moves.iter().fold((0, 0), |(s0, s1), (m0, m1)| {
        match m0.partial_cmp(m1) {
            Some(Ordering::Greater) => (s0 + 6 as u64 + m0, s1 + 0 + m1.into()),
            Some(Ordering::Equal) => (s0 + 3 + m0.into(), s1 + 3 + m1.into()),
            Some(Ordering::Less) => (s0 + 0 + m0.into(), s1 + 3 + m1.into()),
            None => (s0, s1)
        };
        todo!()
    });

    // for (m0, m1) in two_player_moves {
    //     match m0.partial_cmp(m1) {
    //         Some(Ordering::Greater) => {

    //         }
    //         Some(Ordering::Equal) => todo!(),
    //         Some(Ordering::Less) => _,
    //         None => todo!(),
    //     }
    // }

    todo!()
}

fn main() {

}

#[cfg(test)]
mod tests {
    use crate::Move;

    #[test]
    fn char_to_move_conversion() {
        assert_eq!(Move::try_from('A').unwrap(), Move::Rock);
        assert_eq!(Move::try_from('B').unwrap(), Move::Paper);
        assert_eq!(Move::try_from('C').unwrap(), Move::Scissors);
        assert_eq!(Move::try_from('X').unwrap(), Move::Rock);
        assert_eq!(Move::try_from('Y').unwrap(), Move::Paper);
        assert_eq!(Move::try_from('Z').unwrap(), Move::Scissors);
        assert!(Move::try_from('a').is_err());
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
}
