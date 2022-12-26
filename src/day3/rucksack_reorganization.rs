use anyhow::{anyhow, Error, Result};

use std::collections::HashSet;
use std::convert::TryFrom;
use std::fs::File;
use std::hash::Hash;
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

fn line_to_priorities(line: &str) -> Result<Priorities> {
    line.chars().map(|c| Priority::try_from(c)).collect()
}

fn read_file(path: &str) -> Result<Vec<Priorities>> {
    let f = File::open(path)?;
    let buf_reader = BufReader::new(f);
    buf_reader
        .lines()
        .map(|line_res| {
            line_res
                .map_err(Error::from)
                .and_then(|line| line_to_priorities(&line))
        })
        .collect()
}

fn part1(prios: &Priorities) -> u64 {
    assert!(prios.len() % 2 == 0);
    let p0_distinct = prios.iter().take(prios.len() / 2).collect::<HashSet<_>>();
    let p1_distinct = prios.iter().skip(prios.len() / 2).collect::<HashSet<_>>();
    p0_distinct
        .intersection(&p1_distinct)
        .map(|p| p.0)
        .sum::<u64>()
}

struct NIterator<I> {
    iter: I,
    n: usize,
}

impl<I> NIterator<I> {
    fn new(iter: I, n: usize) -> Self {
        NIterator { n, iter }
    }
}

impl<I> Iterator for NIterator<I>
where
    I: Iterator,
{
    type Item = Vec<<I as Iterator>::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        let elem0 = self.iter.next();

        if elem0.is_some() {
            let mut v = vec![elem0];
            for _ in 1..self.n {
                let elem = self.iter.next();
                v.push(elem);
            }
            v.into_iter()
                .filter(|e| e.is_some())
                .collect::<Option<Self::Item>>()
        } else {
            None
        }
    }
}

fn part2(line_prios: &[Priorities]) -> u64 {
    let niter = NIterator::new(line_prios.iter(), 3);
    niter.map(|three_lines| {
        assert!(three_lines.len() == 3);
        let h0 = three_lines[0].iter().collect::<HashSet<_>>();
        let h1 = three_lines[1].iter().collect::<HashSet<_>>();
        let h2 = three_lines[2].iter().collect::<HashSet<_>>();

        let intersections = h0.intersection(&h1)
            .map(|p| *p)
            .collect::<HashSet<&Priority>>()
            .intersection(&h2)
            .map(|&p| p.0)
            .collect::<Vec<_>>();
        intersections.iter().sum::<u64>()
    }).sum::<u64>()
}

fn main() {
    let vps = read_file("src/day3/input_day3.txt").expect("Error reading file.");
    
    let part1 = vps
        .iter()
        .map(|prios| part1(prios))
        .sum::<u64>();
    println!("{}", part1);

    let part2 = part2(&vps);
    println!("{}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line_to_priorities_passes() {
        let answer = line_to_priorities("aBxY");
        assert!(answer.is_ok());
        let p0 = answer.unwrap();
        assert_eq!(p0.len(), 4);
        assert_eq!(p0[0].0, 1);
        assert_eq!(p0[1].0, 28);
        assert_eq!(p0[2].0, 24);
        assert_eq!(p0[3].0, 51);
    }

    #[test]
    #[should_panic(expected = "assertion failed: prios.len() % 2 == 0")]
    fn part1_fails_odd_length() {
        let prios = vec![Priority(2), Priority(9), Priority(2), Priority(10), Priority(2), ];
        let _ = part1(&prios);
    }

    #[test]
    fn part1_sum_passes() {
        let prios = vec![Priority(2), Priority(9), Priority(2), Priority(10), Priority(2), Priority(10), Priority(10), Priority(1), ];
        assert_eq!(part1(&prios), 12);
    }

    #[test]
    fn part1_sum_passes_list_empty() {
        let prios = vec![];
        assert_eq!(part1(&prios), 0);
    }

    #[test]
    fn niterator_should_group_with_remainder() {
        let elems = vec![0, 1, 2, 3, 4, 5, 6, 7, ];
        let niter = NIterator::new(elems.into_iter(), 3);
        let nelems = niter.collect::<Vec<_>>();
        assert_eq!(nelems[0], vec![0, 1, 2, ]);
        assert_eq!(nelems[1], vec![3, 4, 5, ]);
        assert_eq!(nelems[2], vec![6, 7, ]);
    }

    #[test]
    fn part2_passes() {
        let p0s = vec![Priority(2), Priority(9), Priority(2), Priority(10), Priority(2), Priority(10), Priority(10), Priority(1), ];
        let p1s = vec![Priority(90), Priority(12), Priority(10), Priority(21), Priority(10), ];
        let p2s = vec![Priority(12), Priority(10), Priority(15), Priority(9), ];

        let answer = part2(&vec![p0s, p1s, p2s, ]);
        assert_eq!(answer, 10)
    }

    #[test]
    #[should_panic(expected = "three_lines.len() == 3")]
    fn part2_fails_on_non_3_length() {
        let p0s = vec![Priority(2), Priority(9), Priority(2), Priority(10), Priority(2), Priority(10), Priority(10), Priority(1), ];
        let p1s = vec![Priority(90), Priority(12), Priority(10), Priority(21), Priority(10), ];
        let p2s = vec![Priority(12), Priority(10), Priority(15), Priority(9), ];
        let p3s = vec![Priority(16), Priority(15), Priority(90), ];

        let answer = part2(&vec![p0s, p1s, p2s, p3s, ]);
        assert_eq!(answer, 10)
    }
}
