use std::{
    collections::{HashMap, HashSet},
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

fn part1(input: &RiskMap) -> u32 {
    let start = Point { x: 0, y: 0 };
    let end = Point {
        x: input.width - 1,
        y: input.height - 1,
    };

    input.find_shortest_path(start, end)
}

fn part2(input: &RiskMap) -> u32 {
    let input = input.repeat(5);

    let start = Point { x: 0, y: 0 };
    let end = Point {
        x: input.width - 1,
        y: input.height - 1,
    };

    input.find_shortest_path(start, end)
}

fn get_input() -> Result<RiskMap> {
    const PATH: &str = "src/day15/input.txt";

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

    Ok(RiskMap::new(data))
}

#[derive(Clone)]
struct RiskMap {
    data: Vec<Vec<u32>>,
    pub height: usize,
    pub width: usize,
}

impl RiskMap {
    pub fn new(data: Vec<Vec<u32>>) -> Self {
        RiskMap {
            height: data.len(),
            width: data[0].len(),
            data,
        }
    }

    pub fn repeat(&self, n: u32) -> Self {
        let mut data = Vec::new();
        for y in 0..n {
            for row in &self.data {
                let mut new_row = Vec::new();
                for x in 0..n {
                    for elem in row {
                        let mut value = elem + x + y;
                        if value > 9 {
                            value = value % 9;
                        }
                        new_row.push(value);
                    }
                }
                data.push(new_row);
            }
        }

        RiskMap {
            data,
            height: self.height * n as usize,
            width: self.width * n as usize,
        }
    }

    pub fn find_shortest_path(&self, start: Point, end: Point) -> u32 {
        let mut shortest_path = HashMap::new();
        let mut to_explore = HashMap::new();

        to_explore.insert(start, 0);

        while to_explore.len() > 0 && !shortest_path.contains_key(&end) {
            let (point, dist) = to_explore.iter().min_by_key(|pair| pair.1).unwrap();
            let point = *point;
            let dist = *dist;

            shortest_path.insert(point, dist);
            to_explore.remove(&point);

            let adjacent = self.adjacent_elements(point);
            for elem in adjacent {
                if !shortest_path.contains_key(&elem) && !to_explore.contains_key(&elem) {
                    to_explore.insert(elem, self.value(elem) + dist);
                }
            }
        }

        *shortest_path.get(&end).unwrap()
    }

    fn adjacent_elements(&self, point: Point) -> Vec<Point> {
        let mut adj = Vec::new();

        if point.x != 0 {
            adj.push(point.add_x(-1));
        }

        if point.x != self.width - 1 {
            adj.push(point.add_x(1));
        }

        if point.y != 0 {
            adj.push(point.add_y(-1));
        }

        if point.y != self.height - 1 {
            adj.push(point.add_y(1));
        }

        adj
    }

    fn value(&self, point: Point) -> u32 {
        self.data[point.y][point.x]
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
