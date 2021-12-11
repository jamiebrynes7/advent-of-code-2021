use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

use anyhow::{Context, Result};
use itertools::iproduct;

fn main() -> Result<()> {
    let input = get_input()?;
    println!("Part 1 result: {}", part1(&input));
    println!("Part 2 result: {}", part2(&input));
    Ok(())
}

fn part1(input: &OctopusGrid) -> usize {
    let mut input = input.clone();
    (0..100).map(|_| input.step()).sum()
}

fn part2(input: &OctopusGrid) -> usize {
    let mut input = input.clone();
    (0..10000).find(|_| input.step() == input.size()).unwrap() + 1
}

fn get_input() -> Result<OctopusGrid> {
    const PATH: &str = "src/day11/input.txt";

    let file = File::open(PATH)?;
    let reader = BufReader::new(file);

    let data = reader
        .lines()
        .map(|line| {
            line?
                .chars()
                .map(|c| {
                    u32::from_str(&c.to_string()).context("Failed to parse character to number")
                })
                .collect::<Result<Vec<u32>, _>>()
        })
        .collect::<Result<Vec<Vec<u32>>, _>>()?;

    Ok(OctopusGrid::new(data))
}

#[derive(Clone)]
struct OctopusGrid {
    data: Vec<Vec<u32>>,
    height: usize,
    width: usize,
}

impl OctopusGrid {
    pub fn new(data: Vec<Vec<u32>>) -> Self {
        OctopusGrid {
            height: data.len(),
            width: data[0].len(),
            data,
        }
    }

    pub fn size(&self) -> usize {
        self.height * self.width
    }

    pub fn step(&mut self) -> usize {
        for point in self.points() {
            self.data[point.y][point.x] += 1;
        }

        let mut flashed = HashSet::new();
        let mut did_any_flashed = true;
        while did_any_flashed {
            did_any_flashed = false;
            for point in self.points() {
                if self.data[point.y][point.x] > 9 && !flashed.contains(&point) {
                    flashed.insert(point);

                    for adjacent in self.get_adjacent_elements(point) {
                        self.data[adjacent.y][adjacent.x] += 1;
                    }

                    did_any_flashed = true;
                }
            }
        }

        for point in &flashed {
            self.data[point.y][point.x] = 0;
        }

        flashed.len()
    }

    fn points(&self) -> impl Iterator<Item = Point> {
        let height = self.height;
        let width = self.width;
        (0..height).flat_map(move |y| (0..width).map(move |x| Point { x, y }))
    }

    fn get_adjacent_elements(&self, point: Point) -> Vec<Point> {
        let x_diff = match point.x {
            0 => vec![0, 1],
            x if x == self.width - 1 => vec![-1, 0],
            _ => vec![-1, 0, 1],
        };

        let y_diff = match point.y {
            0 => vec![0, 1],
            y if y == self.height - 1 => vec![-1, 0],
            _ => vec![-1, 0, 1],
        };

        iproduct!(x_diff, y_diff)
            .map(|(x, y)| point.add_x(x).add_y(y))
            .collect()
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    fn add_x(self, diff: isize) -> Point {
        Point {
            x: (self.x as isize + diff) as usize,
            y: self.y,
        }
    }

    fn add_y(self, diff: isize) -> Point {
        Point {
            x: self.x,
            y: (self.y as isize + diff) as usize,
        }
    }
}
