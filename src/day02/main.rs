use std::{
    convert::TryFrom,
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

fn part1(input: &Vec<Command>) -> u64 {
    let mut depth = 0;
    let mut horizontal = 0;

    for command in input {
        match command.direction {
            Direction::Forward => horizontal += command.magnitude,
            Direction::Down => depth += command.magnitude,
            Direction::Up => depth -= command.magnitude,
        }
    }

    depth * horizontal
}

fn part2(input: &Vec<Command>) -> u64 {
    let mut depth = 0;
    let mut horizontal = 0;
    let mut aim = 0;

    for command in input {
        match command.direction {
            Direction::Forward => {
                horizontal += command.magnitude;
                depth += aim * command.magnitude
            }
            Direction::Down => aim += command.magnitude,
            Direction::Up => aim -= command.magnitude,
        }
    }

    depth * horizontal
}

fn get_input() -> anyhow::Result<Vec<Command>> {
    const PATH: &str = "src/day02/input.txt";

    let file = File::open(PATH)?;
    let reader = BufReader::new(file);

    Ok(reader
        .lines()
        .map(|line| Command::try_from(line.unwrap().as_str()))
        .collect::<Result<Vec<Command>, _>>()?)
}

struct Command {
    pub direction: Direction,
    pub magnitude: u64,
}

impl TryFrom<&str> for Command {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let parts = value.split(" ").collect::<Vec<&str>>();
        let direction = Direction::try_from(parts[0])?;
        let magnitude = u64::from_str(parts[1])?;

        Ok(Command {
            direction,
            magnitude,
        })
    }
}

enum Direction {
    Forward,
    Down,
    Up,
}

impl TryFrom<&str> for Direction {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "forward" => Ok(Direction::Forward),
            "down" => Ok(Direction::Down),
            "up" => Ok(Direction::Up),
            _ => Err(anyhow::Error::msg(format!("Unknown command: {}", value))),
        }
    }
}
