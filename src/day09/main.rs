use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

use anyhow::{Context, Result};

fn main() -> Result<()> {
    let input = get_input()?;
    println!("Part 1 result: {}", part1(&input));
    println!("Part 2 result: {}", part2(&input));
    Ok(())
}

fn part1(input: &HeightMap) -> u32 {
    input
        .find_low_points()
        .iter()
        .map(|point| input.get_height(*point) + 1)
        .sum()
}

fn part2(input: &HeightMap) -> usize {
    let mut basins = input.find_basins();
    basins.sort_by(|x, y| y.len().cmp(&x.len()));
    basins[0..=2].iter().map(|basin| basin.len()).product()
}

fn get_input() -> Result<HeightMap> {
    const PATH: &str = "src/day09/input.txt";

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

    Ok(HeightMap::new(data))
}

struct HeightMap {
    data: Vec<Vec<u32>>,
    height: usize,
    width: usize,
}

impl HeightMap {
    pub fn new(data: Vec<Vec<u32>>) -> Self {
        HeightMap {
            height: data.len(),
            width: data[0].len(),
            data,
        }
    }

    pub fn find_basins(&self) -> Vec<Vec<Point>> {
        self.find_low_points()
            .iter()
            .map(|point| self.explore_basin(*point))
            .collect()
    }

    fn explore_basin(&self, point: Point) -> Vec<Point> {
        let mut basin = vec![];

        let mut queue = vec![];
        let mut explored: HashSet<Point> = HashSet::new();

        queue.push(point);

        while queue.len() > 0 {
            let point = queue.pop().unwrap();

            if explored.contains(&point) {
                continue;
            }

            basin.push(point);
            explored.insert(point);

            let height = self.get_height(point);

            queue.append(
                &mut self
                    .get_adjacent_elements(point)
                    .into_iter()
                    .filter(|candidate| {
                        !explored.contains(candidate)
                            && self.get_height(*candidate) > height
                            && self.get_height(*candidate) != 9
                    })
                    .collect(),
            );
        }

        basin
    }

    pub fn find_low_points(&self) -> Vec<Point> {
        let mut low_points = vec![];

        for point in self.get_points() {
            let value = self.get_height(point);

            if self
                .get_adjacent_elements(point)
                .iter()
                .all(|point| self.get_height(*point) > value)
            {
                low_points.push(point);
            }
        }

        low_points
    }

    pub fn get_height(&self, point: Point) -> u32 {
        self.data[point.y][point.x]
    }

    fn get_points<'a>(&'a self) -> impl Iterator<Item = Point> + 'a {
        (0..self.height).flat_map(move |y| (0..self.width).map(move |x| Point { x, y }))
    }

    fn get_adjacent_elements(&self, point: Point) -> Vec<Point> {
        let mut elements = vec![];

        if point.x != 0 {
            elements.push(point.add_x(-1));
        }

        if point.x != self.width - 1 {
            elements.push(point.add_x(1));
        }

        if point.y != 0 {
            elements.push(point.add_y(-1));
        }

        if point.y != self.height - 1 {
            elements.push(point.add_y(1))
        }

        elements
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
