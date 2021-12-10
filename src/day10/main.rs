use std::{
    convert::TryFrom,
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::Result;
use itertools::Itertools;

fn main() -> Result<()> {
    let input = get_input()?;
    println!("Part 1 result: {}", part1(&input));
    println!("Part 2 result: {}", part2(&input));
    Ok(())
}

fn get_input() -> Result<Vec<Vec<Tag>>> {
    const PATH: &str = "src/day10/input.txt";

    let file = File::open(PATH)?;
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| line.unwrap().clone())
        .map(|line| {
            line.chars()
                .map(|c| Tag::try_from(c))
                .collect::<Result<Vec<Tag>>>()
        })
        .collect::<Result<Vec<Vec<Tag>>>>()
}

fn part1(data: &Vec<Vec<Tag>>) -> u32 {
    data.iter()
        .map(get_corrupted_char)
        .filter_map(|opt| opt.map(|tag| tag.sym.corrupted_value()))
        .sum()
}

fn part2(data: &Vec<Vec<Tag>>) -> u64 {
    let mut scores = data
        .iter()
        .map(get_incomplete_lines)
        .filter_map(|opt| opt)
        .map(|remaining| {
            remaining
                .iter()
                .rev()
                .map(|tag| tag.sym.completion_value())
                .fold(0u64, |acc, val| acc * 5 + val)
        })
        .collect_vec();

    scores.sort();

    scores[scores.len() / 2]
}

fn get_incomplete_lines(line: &Vec<Tag>) -> Option<Vec<&Tag>> {
    let mut stack = Vec::new();

    for tag in line {
        if tag.is_open {
            stack.push(tag);
        } else {
            let opener = stack.pop().unwrap();

            if opener.sym == tag.sym && opener.is_open {
                continue;
            }

            return None;
        }
    }

    Some(stack)
}

fn get_corrupted_char(line: &Vec<Tag>) -> Option<Tag> {
    let mut stack = Vec::new();

    for tag in line {
        if tag.is_open {
            stack.push(tag);
        } else {
            let opener = stack.pop().unwrap();

            if opener.sym == tag.sym && opener.is_open {
                continue;
            }

            return Some(*tag);
        }
    }

    None
}

#[derive(Clone, Copy, Debug)]
struct Tag {
    pub sym: Symbol,
    pub is_open: bool,
}

impl TryFrom<char> for Tag {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '(' => Ok(Tag {
                sym: Symbol::Paren,
                is_open: true,
            }),
            ')' => Ok(Tag {
                sym: Symbol::Paren,
                is_open: false,
            }),
            '[' => Ok(Tag {
                sym: Symbol::Bracket,
                is_open: true,
            }),
            ']' => Ok(Tag {
                sym: Symbol::Bracket,
                is_open: false,
            }),
            '{' => Ok(Tag {
                sym: Symbol::Brace,
                is_open: true,
            }),
            '}' => Ok(Tag {
                sym: Symbol::Brace,
                is_open: false,
            }),
            '<' => Ok(Tag {
                sym: Symbol::AngleBracket,
                is_open: true,
            }),
            '>' => Ok(Tag {
                sym: Symbol::AngleBracket,
                is_open: false,
            }),
            _ => Err(anyhow::format_err!("Unknown char: {}", value)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Symbol {
    Paren,
    Bracket,
    Brace,
    AngleBracket,
}

impl Symbol {
    pub fn corrupted_value(&self) -> u32 {
        match self {
            Symbol::Paren => 3,
            Symbol::Bracket => 57,
            Symbol::Brace => 1197,
            Symbol::AngleBracket => 25137,
        }
    }

    pub fn completion_value(&self) -> u64 {
        match self {
            Symbol::Paren => 1,
            Symbol::Bracket => 2,
            Symbol::Brace => 3,
            Symbol::AngleBracket => 4,
        }
    }
}
