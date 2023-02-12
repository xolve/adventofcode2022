// https://adventofcode.com/2022/day/4
use anyhow::{Error, Result};

use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct SectionRange(i32, i32);

impl TryFrom<&str> for SectionRange {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        let parts = value
            .split("-")
            .map(|p| p.parse::<i32>().map_err(Error::from))
            .collect::<Result<Vec<i32>>>();
        parts.map(|res| SectionRange(res[0], res[1]))
    }
}

fn read_file(path: &str) -> Result<Vec<(SectionRange, SectionRange)>> {
    let f = File::open(path)?;
    let buf_reader = BufReader::new(f);

    let parsed = buf_reader.lines().map(|line_res| {
        line_res.map_err(Error::from).and_then(|line| {
            let  mut parts = line.split(",").map(|s| SectionRange::try_from(s));
            if let (Some(Ok(x)), Some(Ok(y))) = (parts.next(), parts.next()) {
                Ok((x, y))
            } else {
                Err(Error::msg(format!("Error parsing line {}", line)))
            }
        })
    });

    parsed.collect()
}

fn either_fully_contained(sr0: &SectionRange, sr1: &SectionRange) -> bool {
    fn contained(this_one: &SectionRange, that_one: &SectionRange) -> bool {
        this_one.0 <= that_one.0 && this_one.1 >= that_one.1
    }

    contained(sr0, sr1) || contained(sr1, sr0)
}

fn either_overlap(sr0: &SectionRange, sr1: &SectionRange) -> bool {
    (sr0.0 <= sr1.0 && sr0.1 >= sr1.0)
        || (sr0.0 >= sr1.0 && sr0.0 <= sr1.1)
}

fn main() {
    let contents = read_file("src/day4/input_day4.txt").unwrap();
    let part0 = contents.iter().filter(|(sr0, sr1)| either_fully_contained(sr0, sr1)).count();
    println!("part0: {}", part0);

    let part1 = contents.iter().filter(|(sr0, sr1)| either_overlap(sr0, sr1)).count();
    println!("part0: {}", part1);
}

mod tests {
    use  super::*;

    #[test]
    fn either_overlap_test() {
        let sr_pairs = vec![
            (SectionRange(2, 4), SectionRange(6, 8)),
            (SectionRange(2, 3), SectionRange(4, 5)),
            (SectionRange(5, 7), SectionRange(7, 9)),
            (SectionRange(2, 8), SectionRange(3, 7)),
            (SectionRange(6, 6), SectionRange(4, 6)),
            (SectionRange(2, 6), SectionRange(4, 8)),
        ];

        let ans = sr_pairs.iter().map(|(sr0, sr1)| either_overlap(sr0, sr1)).collect::<Vec<bool>>();
        assert_eq!(ans, vec![false, false, true, true, true, true]);
    }
}