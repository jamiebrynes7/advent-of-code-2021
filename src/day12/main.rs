use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    let input = get_input()?;
    println!("Part 1 result: {}", part1(&input));
    println!("Part 2 result: {}", part2(&input));
    Ok(())
}

fn part1(input: &Graph) -> u32 {
    let chain = vec![Identifier::Start];
    input.count_chains(chain, |chain, ident| chain.contains(ident))
}

fn part2(input: &Graph) -> u32 {
    let chain = vec![Identifier::Start];
    input.count_chains(chain, |chain, ident| {
        if !chain.contains(ident) {
            return false;
        }

        let smalls = chain.iter().filter(|i| i.is_small()).counts();
        !(smalls.iter().all(|(_, c)| *c != 2))
    })
}

fn get_input() -> anyhow::Result<Graph> {
    const PATH: &str = "src/day12/input.txt";

    let file = File::open(PATH)?;
    let reader = BufReader::new(file);

    Ok(Graph::new(
        reader
            .lines()
            .map(|line| {
                let line = line?;
                let mut parts = line.split("-");

                Ok((
                    Identifier::from_str(parts.next().unwrap())?,
                    Identifier::from_str(parts.next().unwrap())?,
                ))
            })
            .collect::<anyhow::Result<Vec<(Identifier, Identifier)>>>()?,
    ))
}

#[derive(Debug)]
struct Graph {
    pub nodes: HashMap<Identifier, Vec<Identifier>>,
}

impl Graph {
    pub fn new(connections: Vec<(Identifier, Identifier)>) -> Self {
        let mut identifiers = HashSet::new();

        for c in &connections {
            identifiers.insert(c.0.clone());
            identifiers.insert(c.1.clone());
        }

        let mut nodes = HashMap::new();

        for ident in identifiers {
            nodes.insert(ident, vec![]);
        }

        for c in &connections {
            nodes.entry(c.0.clone()).and_modify(|v| v.push(c.1.clone()));
            nodes.entry(c.1.clone()).and_modify(|v| v.push(c.0.clone()));
        }

        Graph { nodes }
    }

    pub fn count_chains<F>(&self, chain: Vec<Identifier>, should_skip_small: F) -> u32
    where
        F: Fn(&Vec<Identifier>, &Identifier) -> bool + Copy,
    {
        let last = chain.last().unwrap();
        let next = self.nodes.get(last).unwrap().iter();

        let mut paths = 0;

        for ident in next {
            if ident == &Identifier::End {
                paths += 1;
                continue;
            }

            if ident == &Identifier::Start {
                continue;
            }

            if let Identifier::Small(_) = ident {
                if should_skip_small(&chain, ident) {
                    continue;
                }
            }

            let mut new_chain = chain.clone();
            new_chain.push(ident.clone());
            paths += self.count_chains(new_chain, should_skip_small)
        }

        paths
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Identifier {
    Start,
    End,
    Large(String),
    Small(String),
}

impl Identifier {
    pub fn is_small(&self) -> bool {
        match self {
            Identifier::Small(_) => true,
            _ => false,
        }
    }
}

impl FromStr for Identifier {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "start" => Identifier::Start,
            "end" => Identifier::End,
            s if s.chars().all(|c| c.is_uppercase()) => Identifier::Large(s.into()),
            s if s.chars().all(|c| c.is_lowercase()) => Identifier::Small(s.into()),
            _ => Err(anyhow::format_err!("Failed to parse identifier: '{}'", s))?,
        })
    }
}
