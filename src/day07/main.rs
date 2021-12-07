use std::{
    fs::File,
    io::{BufReader, Read},
    str::FromStr,
};

use anyhow::Result;

fn main() -> Result<()> {
    let input = get_input()?;
    println!("Part 1 result: {}", part1(input.clone()));
    println!("Part 2 result: {}", part2(input));

    Ok(())
}

// This part is effectively finding the L-1 norm of the dataset.
fn part1(mut data: Vec<i64>) -> i64 {
    data.sort();
    let median = data[data.len() / 2];

    data.iter().map(|elem| (elem - median).abs()).sum()
}

// Just brute force this one, couldn't find a nice closed form for this.
fn part2(data: Vec<i64>) -> i64 {
    let min = *data.iter().min().unwrap();
    let max = *data.iter().max().unwrap();

    let calc_costs = |target: i64| {
        data.iter()
            .map(|elem| {
                // The cost of each element is the sum of 1 to N where N = |elem - target|.
                // This then reduces down as a linear sum.
                let abs = (elem - target).abs();

                abs * (abs + 1) / 2
            })
            .sum::<i64>()
    };

    let optimal = (min..max)
        .map(|target| (target, calc_costs(target)))
        .min_by(|x, y| x.1.cmp(&y.1))
        .unwrap();

    optimal.1
}

fn get_input() -> Result<Vec<i64>> {
    const PATH: &str = "src/day06/input.txt";

    let file = File::open(PATH)?;
    let mut reader = BufReader::new(file);

    let mut data = String::new();
    reader.read_to_string(&mut data)?;

    Ok(data
        .split(",")
        .map(|elem| i64::from_str(elem))
        .collect::<Result<Vec<i64>, _>>()?)
}
