use std::{
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

const PATH: &str = "src/day01/input.txt";

fn main() -> anyhow::Result<()> {
    let input = get_input()?;
    println!("Part 1: {}", calc::<1>(&input));
    println!("Part 2: {}", calc::<3>(&input));

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

fn calc<const T: usize>(data: &Vec<u64>) -> u32 {
    let mut count = 0;
    let mut last: u64 = data[0..T].iter().sum();

    for current in data[1..].windows(T) {
        let sum: u64 = current.iter().sum();

        if sum > last {
            count += 1;
        }

        last = sum;
    }

    count
}
