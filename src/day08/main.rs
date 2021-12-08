use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::Result;
use itertools::Itertools;

fn main() -> Result<()> {
    let input = get_input()?;
    println!("Part 1 result: {}", part1(&input));
    println!("Part 2 result: {}", part2(&input));
    Ok(())
}

fn part1(input: &Vec<NoteEntry>) -> usize {
    let unique_output_counts = [2, 4, 3, 7]; // The number of segments to make [1, 4, 7, 8].

    input
        .iter()
        .flat_map(|note| {
            note.output
                .iter()
                .filter(|elem| unique_output_counts.contains(&elem.len()))
        })
        .count()
}

fn part2(input: &Vec<NoteEntry>) -> i32 {
    let mut sum = 0;
    for entry in input {
        // First need to determine the mapping.
        let mapping = get_mapping(&entry.patterns);
        let result = entry
            .output
            .iter()
            .map(|e| {
                e.chars()
                    .map(|c| mapping.get(&c).unwrap())
                    .sorted()
                    .collect::<String>()
            })
            .map(|elem| match elem.as_str() {
                "abcefg" => 0,
                "cf" => 1,
                "acdeg" => 2,
                "acdfg" => 3,
                "bcdf" => 4,
                "abdfg" => 5,
                "abdefg" => 6,
                "acf" => 7,
                "abcdefg" => 8,
                "abcdfg" => 9,
                _ => panic!("unknown pattern"),
            })
            .rev()
            .enumerate()
            .fold(0, |count, elem| count + elem.1 * 10i32.pow(elem.0 as u32));

        sum += result;
    }
    sum
}

fn get_input() -> Result<Vec<NoteEntry>> {
    const PATH: &str = "src/day08/input.txt";

    let file = File::open(PATH)?;
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| {
            let line = line?;
            let mut parts = line.split(" | ");
            let patterns = parts
                .next()
                .unwrap()
                .split(" ")
                .map(|s| s.to_string())
                .collect::<Vec<String>>();
            let output = parts
                .next()
                .unwrap()
                .split(" ")
                .map(|s| s.to_string())
                .collect::<Vec<String>>();

            Ok(NoteEntry { patterns, output })
        })
        .collect::<Result<Vec<NoteEntry>, _>>()
}

struct NoteEntry {
    pub patterns: Vec<String>,
    pub output: Vec<String>,
}

pub fn get_mapping(patterns: &Vec<String>) -> HashMap<char, char> {
    let mut mapping = HashMap::new();
    let mut possible_mappings = HashMap::new();

    let mut patterns_by_length: HashMap<usize, Vec<&String>> = HashMap::new();

    for pattern in patterns {
        patterns_by_length
            .entry(pattern.len())
            .and_modify(|v| v.push(pattern))
            .or_insert(vec![pattern]);
    }

    // By comparing the 2 and 3 length segments, we can determine the mapping to 'a'.
    let two = patterns_by_length.get(&2).unwrap()[0];
    let three = patterns_by_length.get(&3).unwrap()[0];

    mapping.insert(get_unique_char(two, three), 'a');

    // We also know the possible mappings for 'c' and 'f'.
    for c in two.chars() {
        possible_mappings.insert(c, vec!['c', 'f']);
    }

    // We can then use the 4 length segment to get possible mappings for 'b' and 'd'.
    let four = patterns_by_length.get(&4).unwrap()[0];

    for c in four.chars() {
        possible_mappings.entry(c).or_insert(vec!['b', 'd']);
    }

    // Now we can look at the 5 length segments, either '3' or '5' will have 4/5 letters in the mapping/possible mapping already.
    // This means that the final missing letter must be 'g'.
    let fives = patterns_by_length.get(&5).unwrap();

    let missing_letter = fives
        .iter()
        .filter(|elem| {
            elem.chars()
                .filter(|c| !mapping.contains_key(c) && !possible_mappings.contains_key(c))
                .count()
                == 1
        })
        .nth(0)
        .unwrap()
        .chars()
        .filter(|c| !mapping.contains_key(c) && !possible_mappings.contains_key(c))
        .nth(0)
        .unwrap();

    mapping.insert(missing_letter, 'g');

    // Now we can figure out 'e' by examining '2', using the same process as above.
    let missing_letter = fives
        .iter()
        .filter(|elem| {
            elem.chars()
                .filter(|c| !mapping.contains_key(c) && !possible_mappings.contains_key(c))
                .count()
                == 1
        })
        .nth(0)
        .unwrap()
        .chars()
        .filter(|c| !mapping.contains_key(c) && !possible_mappings.contains_key(c))
        .nth(0)
        .unwrap();

    mapping.insert(missing_letter, 'e');

    // Then we can figure out 'd' and 'b' by counting the number of unknown letters left in the fives.
    let occurences = fives
        .iter()
        .flat_map(|elem| elem.chars())
        .filter(|c| !mapping.contains_key(c))
        .fold(HashMap::new(), |mut map, c| {
            map.entry(c).and_modify(|count| *count += 1).or_insert(1);
            map
        });

    let b = occurences
        .iter()
        .find(|(c, count)| **count == 1)
        .unwrap()
        .0
        .clone();

    mapping.insert(b, 'b');

    let d = occurences
        .iter()
        .find(|(c, count)| **count == 3)
        .unwrap()
        .0
        .clone();

    mapping.insert(d, 'd');

    // Now we can figure out 'f' from '6' by process of elimination (we have 5/6).
    let six = patterns_by_length
        .get(&6)
        .unwrap()
        .iter()
        .find(|elem| elem.chars().filter(|c| !mapping.contains_key(c)).count() == 1)
        .unwrap();

    let f = six
        .chars()
        .filter(|c| !mapping.contains_key(c))
        .nth(0)
        .unwrap();

    mapping.insert(f, 'f');

    // And now the last one is the remaining letter.
    let remaining = patterns_by_length.get(&7).unwrap()[0]
        .chars()
        .filter(|c| !mapping.contains_key(c))
        .nth(0)
        .unwrap();

    mapping.insert(remaining, 'c');

    mapping
}

fn get_unique_char(first: &str, second: &str) -> char {
    first
        .chars()
        .chain(second.chars())
        .fold(HashMap::new(), |mut map, elem| {
            map.entry(elem)
                .and_modify(|count| {
                    *count += 1;
                })
                .or_insert(1);

            map
        })
        .iter()
        .find(|(_, count)| **count == 1)
        .unwrap()
        .0
        .clone()
}
