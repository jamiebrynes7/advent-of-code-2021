use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> anyhow::Result<()> {
    let input = get_input()?;
    println!("Part 1 result: {}", part1(&input)?);
    println!("Part 2 result: {}", part2(&input)?);
    Ok(())
}

fn get_input() -> anyhow::Result<Vec<String>> {
    const PATH: &str = "src/day03/input.txt";

    let file = File::open(PATH)?;
    let reader = BufReader::new(file);

    Ok(reader
        .lines()
        .map(|line| line.unwrap().clone())
        .collect::<Vec<String>>())
}

fn part1(data: &Vec<String>) -> anyhow::Result<u32> {
    let counts = count_bits(data);

    let gamma = u32::from_str_radix(
        &counts
            .iter()
            .map(|count| if count[0] > count[1] { '0' } else { '1' })
            .collect::<String>(),
        2,
    )?;

    let epsilon = u32::from_str_radix(
        &counts
            .iter()
            .map(|count| if count[0] > count[1] { '1' } else { '0' })
            .collect::<String>(),
        2,
    )?;

    Ok(gamma * epsilon)
}

fn part2(data: &Vec<String>) -> anyhow::Result<u32> {
    let max_bits = data[0].len();

    let mut ox_candidates = data.clone();
    let mut bit = 0;

    while ox_candidates.len() > 1 && bit < max_bits {
        let counts = count_bits(&ox_candidates);

        let desired_bit = if counts[bit][0] > counts[bit][1] {
            '0'
        } else {
            '1'
        };

        ox_candidates = ox_candidates
            .into_iter()
            .filter(|num| num.chars().nth(bit).unwrap() == desired_bit)
            .collect();

        bit += 1;
    }

    let ox_rating = u32::from_str_radix(&ox_candidates[0], 2)?;

    let mut co2_candidates = data.clone();
    let mut bit = 0;
    while co2_candidates.len() > 1 && bit < max_bits {
        let counts = count_bits(&co2_candidates);

        let desired_bit = if counts[bit][0] > counts[bit][1] {
            '1'
        } else {
            '0'
        };

        co2_candidates = co2_candidates
            .into_iter()
            .filter(|num| num.chars().nth(bit).unwrap() == desired_bit)
            .collect();

        bit += 1;
    }

    let co2_rating = u32::from_str_radix(&co2_candidates[0], 2)?;

    Ok(co2_rating * ox_rating)
}

fn count_bits(data: &Vec<String>) -> Vec<[u32; 2]> {
    let mut counts = data[0].chars().map(|_| [0, 0]).collect::<Vec<[u32; 2]>>();

    for datum in data {
        for (i, c) in datum.chars().enumerate() {
            if c == '0' {
                counts[i][0] += 1;
            } else {
                counts[i][1] += 1;
            }
        }
    }

    counts
}
