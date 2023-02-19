use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::vec;

#[derive(Debug)]
struct StackInstruction {
    quantity: i32,
    source: usize,
    target: usize,
}

impl TryFrom<&str> for StackInstruction {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> anyhow::Result<Self> {
        let re = Regex::new(r"move (?P<qty>\d+) from (?P<source>\w) to (?P<target>\w)").unwrap();
        match re.captures(value) {
            Some(caps) => Ok(Self {
                quantity: caps["qty"].parse().unwrap(),
                source: caps["source"].parse().unwrap(),
                target: caps["target"].parse().unwrap(),
            }),
            None => Err(anyhow::Error::msg("Cannot parse.")),
        }
    }
}

#[derive(Debug)]
struct LetterStack {
    stack: Vec<char>,
}

impl LetterStack {
    fn pop(&mut self) -> Option<char> {
        self.stack.pop()
    }

    fn pop_many(&mut self, count: usize) -> Vec<char> {
        let mut ret_val: Vec<char> = (0..count).map(|_| '\0').collect();
        (0..count).for_each(|i| {
            let c = self.stack.pop();
            ret_val[count - i - 1] = c.unwrap();
        });
        ret_val
    }

    fn push(&mut self, letter: char) {
        self.stack.push(letter);
    }

    fn push_many(&mut self, letters: Vec<char>) {
        letters.iter().for_each(|c| self.stack.push(*c));
    }

    fn top(&self) -> Option<&char> {
        self.stack.last()
    }
}

fn read_stacks(buf_reader: &mut BufReader<File>) -> Vec<LetterStack> {
    let inital_stack_lines: Vec<String> = buf_reader
        .lines()
        .take_while(|line| match line {
            Ok(line) => line.len() > 0,
            Err(_) => false,
        })
        .map(|line| line.unwrap())
        .collect();

    let (last_line, stack_info) = inital_stack_lines
        .split_last()
        .expect("Missing initial stack representation data.");
    
    last_line
        .chars()
        .enumerate()
        .filter(|(_, c)| !c.is_ascii_whitespace())
        .map(|(idx, x)| {
            let stack: Vec<char> = stack_info
                .iter()
                .rev()
                .map(|line| line.as_bytes()[idx] as char)
                .filter(|c| c.is_alphabetic())
                .collect();
            LetterStack {
                stack: stack,
            }
        })
        .collect()
}


fn main() {
    {
        let mut buf_reader = BufReader::new(File::open("src/day5/input_day5.txt").unwrap());
        let mut stacks = read_stacks(&mut buf_reader);

        buf_reader.lines().for_each(|line| {
            let s = StackInstruction::try_from(line.unwrap().as_str()).unwrap();
            for _ in 0..s.quantity {
                let popped = stacks[s.source - 1].pop().unwrap();
                stacks[s.target - 1].push(popped);
            }
        });

        let part1 = stacks.iter().map(|s| s.top()).collect::<Option<String>>();
        println!("part1: {:?}", part1);
    }

    {
        let mut buf_reader = BufReader::new(File::open("src/day5/input_day5.txt").unwrap());
        let mut stacks = read_stacks(&mut buf_reader);

        buf_reader.lines().for_each(|line| {
            let s = StackInstruction::try_from(line.unwrap().as_str()).unwrap();
            let popped = stacks[s.source - 1].pop_many(s.quantity as usize);
            stacks[s.target - 1].push_many(popped);
        });

        let part1 = stacks.iter().map(|s| s.top()).collect::<Option<String>>();
        println!("part2: {:?}", part1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pop_many() {
        let mut letter_stack = LetterStack {
            stack: vec!['a', 'b', 'c', 'd', 'e'],
        };

        let popped = letter_stack.pop_many(4);
        assert_eq!(popped, vec!['b', 'c', 'd', 'e']);
    }

    #[test]
    fn test_push_many() {
        let mut letter_stack = LetterStack {
            stack: vec!['a', 'b', 'c', 'd', 'e'],
        };

        letter_stack.push_many(vec!['f', 'g', 'h']);
        assert_eq!(letter_stack.pop(), Some('h'));
        assert_eq!(letter_stack.pop(), Some('g'));
        assert_eq!(letter_stack.pop(), Some('f'));
    }
}