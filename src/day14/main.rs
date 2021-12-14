use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

use anyhow::Result;
use itertools::{Itertools, MinMaxResult};

fn main() -> Result<()> {
    let input = get_input()?;
    println!("Part 1 result: {}", part1(&input));
    println!("Part 2 result: {}", part2(&input));

    Ok(())
}

fn part1(input: &Input) -> usize {
    find_answer(input, 10)
}

fn part2(input: &Input) -> usize {
    find_answer(input, 40)
}

fn find_answer(input: &Input, iterations: usize) -> usize {
    let initial = input
        .template
        .chars()
        .tuple_windows()
        .map(|(c1, c2)| format!("{}{}", c1, c2))
        .counts();

    let final_state = (0..iterations).fold(initial, |state, _| apply_process(state, &input.rules));

    let mut frequency = final_state.iter().fold(HashMap::new(), |mut state, pair| {
        for c in pair.0.chars() {
            state
                .entry(c)
                .and_modify(|val| *val += *pair.1)
                .or_insert(*pair.1);
        }
        state
    });

    // HACK: We've effectively counted each letter twice, except for the first and last element in the initial template.. so lets add those and then divide the final answer by 2.
    let template_chars = &mut input.template.chars();
    frequency
        .entry(template_chars.nth(0).unwrap())
        .and_modify(|val| *val += 1);
    frequency
        .entry(template_chars.last().unwrap())
        .and_modify(|val| *val += 1);

    let minmax = frequency.iter().minmax_by_key(|pair| pair.1);

    if let MinMaxResult::MinMax(min, max) = minmax {
        // Need to divide by two, as we've counted every letter twice.
        (max.1 - min.1) / 2
    } else {
        panic!("Min and max not found!")
    }
}

fn apply_process(
    state: HashMap<String, usize>,
    rules: &HashMap<String, InsertionRule>,
) -> HashMap<String, usize> {
    let mut result: HashMap<String, usize> = HashMap::new();

    for (pattern, count) in state {
        let rule = rules.get(&pattern).unwrap();
        for product_pattern in &rule.produces {
            result
                .entry(product_pattern.clone())
                .and_modify(|val| *val += count)
                .or_insert(count);
        }
    }

    result
}

fn get_input() -> Result<Input> {
    const PATH: &str = "src/day14/input.txt";

    let file = File::open(PATH)?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let template = lines.next().unwrap()?;
    lines.next();

    let rules = lines
        .map(|line| InsertionRule::from_str(&line?))
        .collect::<Result<Vec<InsertionRule>, _>>()?;

    let rules_map = rules
        .into_iter()
        .map(|rule| (rule.pattern.clone(), rule))
        .collect::<HashMap<String, InsertionRule>>();

    Ok(Input {
        template,
        rules: rules_map,
    })
}

struct Input {
    pub template: String,
    pub rules: HashMap<String, InsertionRule>,
}

struct InsertionRule {
    pub pattern: String,
    pub produces: [String; 2],
}

impl FromStr for InsertionRule {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(" -> ");
        let pattern = parts.next().unwrap().to_owned();
        let insertion = parts.next().unwrap();

        let chars = &mut pattern.chars();

        Ok(InsertionRule {
            produces: [
                format!("{}{}", chars.nth(0).unwrap(), insertion),
                format!("{}{}", insertion, chars.nth(0).unwrap()),
            ],
            pattern,
        })
    }
}
