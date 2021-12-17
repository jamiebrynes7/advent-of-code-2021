use std::{fs::File, io::Read, str::FromStr};

use anyhow::{Context, Result};
use regex::Regex;

fn main() -> Result<()> {
    let input = get_input()?;
    println!("Part 1 result: {}", part1(&input));
    println!("Part 2 result: {}", part2(&input));
    Ok(())
}

fn part1(input: &Target) -> i64 {
    // Closed form solution, the ideal case is that y = 0 at t = n - 1 and y = y_min at t = n where y_min is in the target bounds.
    // This maximises the solution.
    let y_min = input.y.0.abs();
    y_min * (y_min - 1) / 2
}

fn part2(input: &Target) -> usize {
    // Need to search.

    // We can find the lower bound for possible v_x0 values by noting that v_x goes to 0 when t = v_x0.
    // This means that for a given v_x0, the maximum x value is v_x0(vx_0 + 1) / 2.
    // This maximum value must be greater than x_target_min. This gives us a lower value.
    // A fudged answer for this gives us v_x0 >= sqrt(2 * x_min) - 1.

    // We can also determine the upper bound by noting that v_x0 <= x_target_max.
    let vx_min = ((2 * input.x.0) as f64).sqrt() as i64 - 1;
    let vx_max = input.x.1;

    // Similarly, we we can limit the y searching by noting it cannot be less than y_min as it would immediately undershoot.
    // The upper limit of vy follows from part 1 where if v_vy0 = -vy_min, it will overshoot on the down trend.
    // Note this only holds as vy_min < 0.
    let vy_min = input.y.0;
    let vy_max = input.y.0.abs();

    let mut count = 0;

    for vx_candidate in vx_min..=vx_max {
        for vy_candidate in vy_min..=vy_max {
            if input.falls_in_target(vx_candidate, vy_candidate) {
                count += 1;
            }
        }
    }

    count
}

fn get_input() -> Result<Target> {
    const PATH: &str = "src/day17/input.txt";

    let mut file = File::open(PATH)?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;

    let re = Regex::new(r"([-0-9]+)").unwrap();
    let captures = re
        .captures_iter(&data)
        .map(|cap| i64::from_str(&cap[0]).context("Failed to parse int"))
        .collect::<Result<Vec<i64>>>()?;

    Ok(Target {
        x: (captures[0], captures[1]),
        y: (captures[2], captures[3]),
    })
}

struct Target {
    pub x: (i64, i64),
    pub y: (i64, i64),
}

impl Target {
    pub fn falls_in_target(&self, vx_init: i64, vy_init: i64) -> bool {
        let mut x = 0;
        let mut y = 0;

        let mut vx = vx_init;
        let mut vy = vy_init;

        while x <= self.x.1 && y >= self.y.0 {
            x += vx;
            y += vy;

            vx += if vx > 0 {
                -1
            } else if vx < 0 {
                1
            } else {
                0
            };
            vy -= 1;

            if x >= self.x.0 && x <= self.x.1 && y >= self.y.0 && y <= self.y.1 {
                return true;
            }
        }

        false
    }
}
