use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    let input = get_input()?;
    println!("Part 1 result: {}", part1(&input));
    part2(&input);
    Ok(())
}

fn part1(input: &Input) -> usize {
    let mut points = input.points.clone();
    fold_paper(&mut points, &input.insts[0]);
    points.len()
}

fn part2(input: &Input) {
    let mut points = input.points.clone();
    for inst in &input.insts {
        fold_paper(&mut points, inst);
    }

    println!("Code for part 2:");
    print_points(points);
}

fn fold_paper(points: &mut HashSet<Point>, instruction: &Instruction) {
    let points_to_fold = points
        .iter()
        .filter(|point| match instruction.axis {
            Axis::X => point.x > instruction.magnitude,
            Axis::Y => point.y > instruction.magnitude,
        })
        .map(|p| p.clone())
        .collect_vec();

    for point in points_to_fold {
        points.remove(&point);

        let new_coords = match instruction.axis {
            Axis::X => Point {
                x: 2 * instruction.magnitude - point.x,
                y: point.y,
            },
            Axis::Y => Point {
                x: point.x,
                y: 2 * instruction.magnitude - point.y,
            },
        };

        points.insert(new_coords);
    }
}

fn print_points(points: HashSet<Point>) {
    let x_max = points.iter().map(|p| p.x).max().unwrap();
    let y_max = points.iter().map(|p| p.y).max().unwrap();

    for y in 0..=y_max {
        let line = (0..=x_max)
            .map(|x| {
                let point = Point { x, y };
                if points.contains(&point) {
                    '#'
                } else {
                    ' '
                }
            })
            .collect::<String>();
        println!("{}", line);
    }
}

fn get_input() -> anyhow::Result<Input> {
    const PATH: &str = "src/day13/input.txt";

    let file = File::open(PATH)?;
    let reader = BufReader::new(file);

    let mut input = Input {
        points: HashSet::new(),
        insts: vec![],
    };

    let mut is_points = true;

    for line in reader.lines() {
        let line = line?;

        if line.is_empty() {
            is_points = false;
            continue;
        }

        if is_points {
            input.points.insert(Point::from_str(&line)?);
        } else {
            input.insts.push(Instruction::from_str(&line)?);
        }
    }

    Ok(input)
}

struct Input {
    pub points: HashSet<Point>,
    pub insts: Vec<Instruction>,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Point {
    pub x: u64,
    pub y: u64,
}

impl FromStr for Point {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(",");

        Ok(Point {
            x: u64::from_str(parts.next().unwrap())?,
            y: u64::from_str(parts.next().unwrap())?,
        })
    }
}

struct Instruction {
    pub axis: Axis,
    pub magnitude: u64,
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.replace("fold along ", "");
        let mut parts = s.split("=");

        let axis = match parts.next().unwrap() {
            "y" => Axis::Y,
            "x" => Axis::X,
            s => Err(anyhow::format_err!("Unknown axis: {}", s))?,
        };

        Ok(Instruction {
            axis,
            magnitude: u64::from_str(parts.next().unwrap())?,
        })
    }
}

enum Axis {
    X,
    Y,
}
