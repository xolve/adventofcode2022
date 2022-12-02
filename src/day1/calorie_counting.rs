// https://adventofcode.com/2022/day/1

use std::fs::File;
use std::io::{BufReader, Read};

fn calorie_count(input_file: &str, n: usize) -> u64 {
    let fp = File::open(input_file).expect("Unable to open file.");
    let mut reader = BufReader::new(fp);
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf).expect("Error reading file.");

    let content = String::from_utf8(buf).expect("Unexpected bytes while decoding file as UTF-8.");
    let mut toal_per_elf = content
        .split("\n\n")
        .map(|block| {
            block
                .lines()
                .map(|x| x.parse::<u64>().expect("Not a number."))
                .sum()
        })
        .collect::<Vec<u64>>();
    toal_per_elf.sort_unstable();
    toal_per_elf.iter().rev().take(n).sum()
}

fn main() {
    let res1 = calorie_count("src/day1/input_day1.txt", 1);
    println!("{}", res1);

    let res2 = calorie_count("src/day1/input_day1.txt", 3);
    println!("{}", res2);
}
