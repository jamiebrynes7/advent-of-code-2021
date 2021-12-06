use std::{
    fs::File,
    io::{BufReader, Read},
    str::FromStr,
};

use anyhow::Result;

fn main() -> Result<()> {
    let input = get_input()?;

    println!("Part 1 result: {}", part1(input.clone()));
    println!("Part 2 result: {}", part2(input.clone()));
    Ok(())
}

fn part1(data: [u64; 9]) -> u64 {
    advance_model(data, 80)
}

fn part2(data: [u64; 9]) -> u64 {
    advance_model(data, 256)
}

fn advance_model(mut fish: [u64; 9], days: usize) -> u64 {
    for _ in 0..days {
        fish.rotate_left(1);
        fish[6] += fish[8];
    }

    fish.iter().sum()
}

fn get_input() -> Result<[u64; 9]> {
    const PATH: &str = "src/day06/input.txt";

    let file = File::open(PATH)?;
    let mut reader = BufReader::new(file);

    let mut data = String::new();
    reader.read_to_string(&mut data)?;

    let mut counts: [u64; 9] = [0; 9];
    for elem in data.split(",") {
        counts[usize::from_str(elem)?] += 1;
    }

    Ok(counts)
}
