use std::{
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

const PATH: &str = "src/day01/input.txt";

fn main() -> anyhow::Result<()> {
    let input = get_input()?;
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    Ok(())
}

fn get_input() -> anyhow::Result<Vec<u64>> {
    let file = File::open(PATH)?;
    let reader = BufReader::new(file);

    Ok(reader
        .lines()
        .map(|line| u64::from_str(&line.unwrap()))
        .collect::<Result<Vec<u64>, _>>()?)
}

fn part1(data: &Vec<u64>) -> u32 {
    let mut count = 0;
    let mut last = *data.get(0).unwrap();

    for current in &data[1..] {
        if *current > last {
            count += 1;
        }

        last = *current;
    }

    count
}

fn part2(data: &Vec<u64>) -> u32 {
    let mut count = 0;
    let mut last: u64 = data[0..3].iter().sum();

    for current in data[1..].windows(3) {
        let sum: u64 = current.iter().sum();

        if sum > last {
            count += 1;
        }

        last = sum;
    }

    count
}
