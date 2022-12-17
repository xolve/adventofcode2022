use anyhow::{anyhow, Error, Result};

use std::collections::HashSet;
use std::convert::TryFrom;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(PartialEq, Eq, Hash)]
struct Priority(u64);

type Priorities = Vec<Priority>;

impl TryFrom<char> for Priority {
    type Error = Error;

    fn try_from(value: char) -> Result<Self> {
        if value.is_ascii_alphabetic() {
            if value.is_ascii_lowercase() {
                Ok(Priority((value as u64) - ('a' as u64) + 1))
            } else {
                Ok(Priority((value as u64) - ('A' as u64) + 27))
            }
        } else {
            Err(anyhow!("Invalid input: {}", value))
        }
    }
}

fn line_to_priorities(line: &str) -> Result<(Priorities, Priorities)> {
    assert!(line.len() % 2 == 0);
    let line_iter = line.chars();
    let v0 = line_iter
        .take(line.len() / 2)
        .map(|c| Priority::try_from(c))
        .collect::<Result<Priorities>>()?;
    let line_iter = line.chars();
    let v1 = line_iter
        .skip(line.len() / 2)
        .take(line.len() / 2)
        .map(|c| Priority::try_from(c))
        .collect::<Result<Priorities>>()?;
    Ok((v0, v1))
}

fn read_file(path: &str) -> Result<Vec<(Priorities, Priorities)>> {
    let f = File::open(path)?;
    let bufReader = BufReader::new(f);
    bufReader
        .lines()
        .map(|line_res| {
            line_res
                .map_err(Error::from)
                .and_then(|line| line_to_priorities(&line))
        })
        .collect()
}

fn intersection_sum(p0s: &Priorities, p1s: &Priorities) -> u64 {
    let p0_distinct = p0s.iter().collect::<HashSet<_>>();
    let p1_distinct = p1s.iter().collect::<HashSet<_>>();
    p0_distinct
        .intersection(&p1_distinct)
        .map(|p| p.0)
        .sum::<u64>()
}

fn main() {
    let vps = read_file("src/day3/input_day3.txt").expect("Error reading file.");
    let part1 = vps
        .iter()
        .map(|(p0, p1)| intersection_sum(p0, p1))
        .sum::<u64>();
    println!("{}", part1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line_to_priorities_passes() {
        let answer = line_to_priorities("aBxY");
        assert!(answer.is_ok());
        let (p0, p1) = answer.unwrap();
        assert_eq!(p0.len(), 2);
        assert_eq!(p1.len(), 2);
        assert_eq!(p0[0].0, 1);
        assert_eq!(p0[1].0, 28);
        assert_eq!(p1[0].0, 24);
        assert_eq!(p1[1].0, 51);
    }

    #[test]
    #[should_panic(expected = "assertion failed: line.len() % 2 == 0")]
    fn line_to_priorities_fails_odd_length() {
        let answer = line_to_priorities("aBxYz");
        assert!(answer.is_err());
    }

    #[test]
    fn intersection_sum_passes() {
        let p0s = vec![Priority(2), Priority(9), Priority(2), Priority(10)];
        let p1s = vec![Priority(2), Priority(10), Priority(10), Priority(1)];
        assert_eq!(intersection_sum(&p0s, &p1s), 12);
    }

    #[test]
    fn intersection_sum_passes_one_list_empty() {
        let p0s = vec![Priority(2), Priority(9), Priority(2), Priority(10)];
        let p1s = vec![];
        assert_eq!(intersection_sum(&p0s, &p1s), 0);
        assert_eq!(intersection_sum(&p1s, &p0s), 0);
    }

    #[test]
    fn intersection_sum_passes_both_lists_empty() {
        let p0s = vec![];
        let p1s = vec![];
        assert_eq!(intersection_sum(&p0s, &p1s), 0);
    }
}