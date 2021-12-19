use std::{
    fs::File,
    io::{BufRead, BufReader},
    ops::Add,
    str::FromStr,
};

use anyhow::{Context, Result};

fn main() -> Result<()> {
    let input = get_input()?;
    println!("Part 1 result: {}", part1(&input));
    println!("Part 2 result: {}", part2(&input));
    Ok(())
}

fn part1(input: &Vec<SnailfishNumber>) -> u64 {
    let first = input[0].clone();
    input[1..]
        .into_iter()
        .fold(first, |acc, elem| acc + elem.clone())
        .magnitude()
}

fn part2(input: &Vec<SnailfishNumber>) -> u64 {
    (0..input.len())
        .flat_map(|i| {
            (0..input.len())
                .filter(move |j| i != *j)
                .flat_map(move |j| vec![(i, j), (j, i)])
        })
        .map(|(i, j)| {
            let sum = input[i].clone() + input[j].clone();
            sum.magnitude()
        })
        .max()
        .unwrap()
}

fn get_input() -> Result<Vec<SnailfishNumber>> {
    const PATH: &str = "src/day18/input.txt";

    let file = File::open(PATH)?;
    let reader = BufReader::new(file);

    Ok(reader
        .lines()
        .map(|line| {
            let line = line?;

            SnailfishNumber::from_str(&line)
        })
        .collect::<Result<Vec<SnailfishNumber>, _>>()?)
}

#[derive(Debug, Clone)]
struct SnailfishNumber {
    pub left: Number,
    pub right: Number,
}

impl SnailfishNumber {
    pub fn reduce(&mut self) {
        while self.try_explode(0).is_some() || self.try_split() {}
    }

    pub fn magnitude(&self) -> u64 {
        self.left.magnitude() * 3 + self.right.magnitude() * 2
    }

    fn try_explode(&mut self, depth: usize) -> Option<ExplodeResult> {
        if let Some(pair) = self.left.try_explode(depth + 1) {
            return Some(match pair {
                ExplodeResult::Exploded => ExplodeResult::Exploded,
                ExplodeResult::NeedsExplode(left, right) => {
                    self.right.add_to_most_left(right);

                    if left.is_some() {
                        ExplodeResult::NeedsExplode(left, None)
                    } else {
                        ExplodeResult::Exploded
                    }
                }
            });
        }

        if let Some(pair) = self.right.try_explode(depth + 1) {
            return Some(match pair {
                ExplodeResult::Exploded => ExplodeResult::Exploded,
                ExplodeResult::NeedsExplode(left, right) => {
                    self.left.add_to_most_right(left);

                    if right.is_some() {
                        ExplodeResult::NeedsExplode(None, right)
                    } else {
                        ExplodeResult::Exploded
                    }
                }
            });
        }

        None
    }

    fn try_split(&mut self) -> bool {
        // Traverse down the left and then the right,
        // First check if each is a number and greater than 10, if so, perform this split.
        self.left.try_split() || self.right.try_split()
    }

    fn is_root_pair(&self) -> bool {
        match (&self.left, &self.right) {
            (Number::Value(_), Number::Value(_)) => true,
            _ => false,
        }
    }
}

enum ExplodeResult {
    Exploded,
    NeedsExplode(Option<u64>, Option<u64>),
}

impl FromStr for SnailfishNumber {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // We want to find the comma separating the left and right then recursively parse the parts.
        let middle_idx =
            get_middle_comma_index(s).context("Failed to find middle comma of string")?;

        let left = &s[1..middle_idx]; // Grab the inner part of the left, ignoring the first character (bracket).
        let right = &s[middle_idx + 1..s.len() - 1]; // Grab the inner part of the right, ignoring the first character (bracket).

        Ok(SnailfishNumber {
            left: Number::from_str(left)?,
            right: Number::from_str(right)?,
        })
    }
}

impl Add for SnailfishNumber {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut num = SnailfishNumber {
            left: Number::Pair(Box::new(self)),
            right: Number::Pair(Box::new(rhs)),
        };

        num.reduce();

        num
    }
}

impl PartialEq for SnailfishNumber {
    fn eq(&self, other: &Self) -> bool {
        self.left == other.left && self.right == other.right
    }
}

#[derive(Debug, Clone)]
enum Number {
    Value(u64),
    Pair(Box<SnailfishNumber>),
}

impl Number {
    pub fn try_split(&mut self) -> bool {
        match self {
            Number::Value(x) if *x >= 10 => {
                let left_val = *x / 2;
                let right_val = *x - left_val;
                *self = Number::Pair(Box::new(SnailfishNumber {
                    left: Number::Value(left_val),
                    right: Number::Value(right_val),
                }));

                return true;
            }
            Number::Pair(inner) => inner.try_split(),
            _ => false,
        }
    }

    pub fn try_explode(&mut self, depth: usize) -> Option<ExplodeResult> {
        match self {
            Number::Value(_) => None,
            Number::Pair(inner) => {
                if depth >= 4 && inner.is_root_pair() {
                    let (left, right) = match (&inner.left, &inner.right) {
                        (Number::Value(first), Number::Value(second)) => (*first, *second),
                        _ => panic!("What?"),
                    };

                    *self = Number::Value(0);
                    return Some(ExplodeResult::NeedsExplode(Some(left), Some(right)));
                }

                inner.try_explode(depth)
            }
        }
    }

    pub fn add_to_most_left(&mut self, value: Option<u64>) {
        if let Some(val) = value {
            let mut target = self;
            loop {
                match target {
                    Number::Value(ref mut x) => {
                        *x += val;
                        return;
                    }
                    Number::Pair(inner) => target = &mut inner.left,
                }
            }
        }
    }

    pub fn add_to_most_right(&mut self, value: Option<u64>) {
        if let Some(val) = value {
            let mut target = self;
            loop {
                match target {
                    Number::Value(ref mut x) => {
                        *x += val;
                        return;
                    }
                    Number::Pair(inner) => target = &mut inner.right,
                }
            }
        }
    }

    pub fn magnitude(&self) -> u64 {
        match self {
            Number::Value(x) => *x,
            Number::Pair(inner) => inner.magnitude(),
        }
    }
}

impl PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Value(l0), Self::Value(r0)) => l0 == r0,
            (Self::Pair(l0), Self::Pair(r0)) => l0 == r0,
            _ => false,
        }
    }
}

impl FromStr for Number {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // All the numbers are less than 10 ¯\_(ツ)_/¯
        if s.len() == 1 {
            return Ok(Number::Value(u64::from_str(s)?));
        }

        Ok(Number::Pair(Box::new(SnailfishNumber::from_str(s)?)))
    }
}

fn get_middle_comma_index(s: &str) -> Option<usize> {
    // To do this, we want to keep a count of open brackets, if we encounter a comma with only 1 open bracket, we know this is the middle.
    let mut bracket_counter = 0;
    for (i, c) in s.chars().enumerate() {
        match c {
            '[' => bracket_counter += 1,
            ']' => bracket_counter -= 1,
            ',' => {
                if bracket_counter == 1 {
                    return Some(i);
                }
            }
            _ => continue,
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use maplit::hashmap;

    use super::*;

    #[test]
    pub fn get_middle_comma_index_test() {
        const S: &str = "[1,2]";
        assert_eq!(get_middle_comma_index(S), Some(2));

        const S2: &str = "[[1,2], 3]";
        assert_eq!(get_middle_comma_index(S2), Some(6));
    }

    #[test]
    pub fn add_snailfish_nums() {
        let left = SnailfishNumber {
            left: Number::Value(1),
            right: Number::Value(2),
        };

        let right = SnailfishNumber {
            left: Number::Pair(Box::new(SnailfishNumber {
                left: Number::Value(3),
                right: Number::Value(4),
            })),
            right: Number::Value(5),
        };

        let sum = left + right;

        let expected = SnailfishNumber {
            left: Number::Pair(Box::new(SnailfishNumber {
                left: Number::Value(1),
                right: Number::Value(2),
            })),
            right: Number::Pair(Box::new(SnailfishNumber {
                left: Number::Pair(Box::new(SnailfishNumber {
                    left: Number::Value(3),
                    right: Number::Value(4),
                })),
                right: Number::Value(5),
            })),
        };

        assert_eq!(expected, sum);
    }

    #[test]
    pub fn test_explode() {
        let cases = hashmap! {
            "[[[[[9,8],1],2],3],4]" => "[[[[0,9],2],3],4]",
            "[7,[6,[5,[4,[3,2]]]]]" => "[7,[6,[5,[7,0]]]]",
            "[[6,[5,[4,[3,2]]]],1]" => "[[6,[5,[7,0]]],3]",
            "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]" => "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
            "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]" => "[[3,[2,[8,0]]],[9,[5,[7,0]]]]"
        };

        for case in cases {
            let mut input = SnailfishNumber::from_str(case.0).unwrap();
            let output = SnailfishNumber::from_str(case.1).unwrap();

            input.try_explode(0);
            assert_eq!(input, output);
        }
    }

    #[test]
    pub fn test_magnitude() {
        let cases = hashmap! {
            "[[1,2],[[3,4],5]]" => 143,
            "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]" => 1384
        };

        for case in cases {
            let input = SnailfishNumber::from_str(case.0).unwrap();
            assert_eq!(input.magnitude(), case.1);
        }
    }
}
