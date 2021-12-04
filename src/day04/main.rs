use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

fn main() -> anyhow::Result<()> {
    let input = get_input()?;
    println!("Part 1 result: {}", part1(&input)?);
    println!("Part 2 result: {}", part2(&input)?);
    Ok(())
}

fn part1(game_state: &GameState) -> anyhow::Result<u64> {
    let mut called_nums = HashSet::new();

    for num in &game_state.numbers {
        called_nums.insert(*num);

        for board in &game_state.boards {
            if board.has_won(&called_nums) {
                return Ok(num * board.get_unmarked_nums(&called_nums).sum::<u64>());
            }
        }
    }

    Err(anyhow::anyhow!("Failed to find a winning board"))
}

fn part2(game_state: &GameState) -> anyhow::Result<u64> {
    let mut called_nums = HashSet::new();
    let mut candidate_boards = game_state.boards.clone();

    for num in &game_state.numbers {
        called_nums.insert(*num);

        if candidate_boards.len() > 1 {
            called_nums.insert(*num);

            candidate_boards = candidate_boards
                .into_iter()
                .filter(|b| !b.has_won(&called_nums))
                .collect();
        } else {
            let last_board = &candidate_boards[0];

            if last_board.has_won(&called_nums) {
                return Ok(num * last_board.get_unmarked_nums(&called_nums).sum::<u64>());
            }
        }
    }

    Err(anyhow::anyhow!("Failed to find a losing board"))
}

fn get_input() -> anyhow::Result<GameState> {
    const PATH: &str = "src/day04/input.txt";

    let file = File::open(PATH)?;
    let reader = BufReader::new(file);
    let lines = reader.lines().collect::<Result<Vec<String>, _>>()?;

    let numbers = lines[0]
        .split(",")
        .map(|num| u64::from_str(num))
        .collect::<Result<Vec<u64>, _>>()?;

    let mut boards = Vec::new();

    for raw_grid in lines[1..]
        .iter()
        .filter(|line| !line.is_empty())
        .collect::<Vec<&String>>()
        .chunks(5)
    {
        let nums = raw_grid
            .iter()
            .map(|line| {
                line.split(" ")
                    .filter(|num| !num.is_empty()) // Filter out empty strings as the grids are formatted with additional whitespace
                    .map(|num| u64::from_str(num))
                    .collect::<Result<Vec<u64>, _>>()
            })
            .collect::<Result<Vec<Vec<u64>>, _>>()?;

        boards.push(Board { grid: nums })
    }

    Ok(GameState { numbers, boards })
}

struct GameState {
    pub numbers: Vec<u64>,
    pub boards: Vec<Board<5>>,
}

#[derive(Clone)]
struct Board<const T: usize> {
    pub grid: Vec<Vec<u64>>,
}

impl<const T: usize> Board<T> {
    pub fn has_won(&self, called_numbers: &HashSet<u64>) -> bool {
        // First check rows:
        for row in &self.grid {
            if row.iter().all(|num| called_numbers.contains(num)) {
                return true;
            }
        }

        // Now check columns
        for index in 0..T {
            if self
                .grid
                .iter()
                .map(|row| row[index])
                .all(|num| called_numbers.contains(&num))
            {
                return true;
            }
        }

        false
    }

    pub fn get_unmarked_nums<'a, 'b: 'a>(
        &'a self,
        called_numbers: &'b HashSet<u64>,
    ) -> impl Iterator<Item = &'a u64> {
        self.grid
            .iter()
            .flat_map(|row| row)
            .filter(move |num| !called_numbers.contains(num))
    }
}
