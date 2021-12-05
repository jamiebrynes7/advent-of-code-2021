use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

fn main() -> anyhow::Result<()> {
    let input = get_input()?;

    println!("Part 1 result: {}", part1(&input));
    println!("Part 2 result: {}", part2(&input));

    Ok(())
}

fn part1(input: &Vec<Line>) -> usize {
    let non_diagonal = input
        .iter()
        .filter(|line| line.is_horizontal() || line.is_vertical())
        .collect::<Vec<&Line>>();

    let mut covered_points: HashMap<Point, u32> = HashMap::new();

    for line in non_diagonal {
        let points = line.get_points();

        for point in points {
            *covered_points.entry(point).or_insert(0) += 1;
        }
    }

    covered_points
        .into_iter()
        .filter(|(_, count)| *count > 1)
        .count()
}

fn part2(input: &Vec<Line>) -> usize {
    let mut covered_points: HashMap<Point, u32> = HashMap::new();

    for line in input {
        let points = line.get_points();

        for point in points {
            *covered_points.entry(point).or_insert(0) += 1;
        }
    }

    covered_points
        .into_iter()
        .filter(|(_, count)| *count > 1)
        .count()
}

fn get_input() -> anyhow::Result<Vec<Line>> {
    const PATH: &str = "src/day05/input.txt";

    let file = File::open(PATH)?;
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let mut parts = line.split(" -> ");

            Ok(Line {
                start: Point::from_str(parts.next().unwrap())?,
                end: Point::from_str(parts.next().unwrap())?,
            })
        })
        .collect::<Result<Vec<Line>, _>>()
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Point {
    pub x: i64,
    pub y: i64,
}

impl FromStr for Point {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(",");
        let x = i64::from_str(parts.next().unwrap())?;
        let y = i64::from_str(parts.next().unwrap())?;

        Ok(Point { x, y })
    }
}

#[derive(Debug, Clone)]
struct Line {
    pub start: Point,
    pub end: Point,
}

impl Line {
    pub fn is_vertical(&self) -> bool {
        self.start.x == self.end.x
    }

    pub fn is_horizontal(&self) -> bool {
        self.start.y == self.end.y
    }

    pub fn get_points(&self) -> Vec<Point> {
        let rise = self.end.y - self.start.y;
        let run = self.end.x - self.start.x;

        // Reduce slope as much as possible
        let gcd = gcd(rise.abs(), run.abs());
        let rise = rise / gcd;
        let run = run / gcd;

        let mut x = self.start.x;
        let mut y = self.start.y;
        let mut points = Vec::new();

        while x != self.end.x || y != self.end.y {
            points.push(Point { x, y });
            x += run;
            y += rise;
        }

        points.push(self.end.clone());
        points
    }
}

pub fn gcd(a: i64, b: i64) -> i64 {
    // Terminal cases
    if a == b {
        return a;
    }
    if a == 0 {
        return b;
    }
    if b == 0 {
        return a;
    }

    if a % 2 == 0 {
        // a is even
        if b % 2 != 0 {
            // b is odd
            return gcd(a / 2, b);
        } else {
            // a and b are even
            return gcd(a / 2, b / 2) * 2;
        }
    }

    // a is odd
    if b % 2 == 0 {
        // b is even
        return gcd(a, b / 2);
    }

    // Reduce larger argument
    if a > b {
        return gcd((a - b) / 2, b);
    }

    return gcd((b - a) / 2, a);
}
