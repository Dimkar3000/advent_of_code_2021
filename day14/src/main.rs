use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
struct Command {
    condition: (char, char),
    addition: char,
}

impl Command {
    // CH -> B
    fn from_str(line: &str) -> Command {
        let (cond, addition) = line.split_once(" -> ").unwrap();
        let mut condition: (char, char) = Default::default();
        condition.0 = cond.chars().next().unwrap();
        condition.1 = cond.chars().nth(1).unwrap();

        Command {
            condition,
            addition: addition.chars().next().unwrap(),
        }
    }
}

#[derive(Debug)]
struct Puzzle {
    // sequence: Vec<char>,
    pairs: HashMap<(char, char), usize>,
    counts: HashMap<char, usize>,
    instructions: Vec<Command>,
}

impl Puzzle {
    fn step(&mut self) {
        let mut mutations: HashMap<(char, char), usize> = HashMap::new();
        let mut reductions: HashMap<(char, char), usize> = HashMap::new();

        for command in &self.instructions {
            if let Some(v) = self.pairs.get_mut(&command.condition) {
                if *v == 0 {
                    continue;
                }
                *reductions.entry(command.condition).or_insert(0) += *v;

                let p1 = (command.condition.0, command.addition);
                let p2 = (command.addition, command.condition.1);

                *mutations.entry(p1).or_insert(0) += *v;
                *mutations.entry(p2).or_insert(0) += *v;
                *self.counts.entry(command.addition).or_insert(0) += *v;
            }
        }

        for (key, value) in mutations {
            *self.pairs.entry(key).or_insert(0) += value;
        }

        for (key, value) in reductions {
            *self.pairs.entry(key).or_default() -= value;
        }
    }

    fn simulate(&mut self, steps: usize) -> usize {
        for _ in 0..steps {
            self.step();
        }
        let max = *self.counts.values().max().unwrap();
        let min = *self.counts.values().min().unwrap();
        max - min
    }
}

fn read_data(filename: &str) -> Puzzle {
    let file = fs::read_to_string(filename).expect("failed to read file");

    let (first_part, second_part) = file.split_once("\n\n").unwrap();

    let mut pairs: HashMap<(char, char), usize> = HashMap::new();
    let mut counts: HashMap<char, usize> = HashMap::new();

    for i in 0..(first_part.len() - 1) {
        let p = (
            first_part.chars().nth(i).unwrap(),
            first_part.chars().nth(i + 1).unwrap(),
        );
        *pairs.entry(p).or_insert(0) += 1;
        *counts.entry(p.0).or_insert(0) += 1;
    }
    *counts
        .entry(first_part.chars().last().unwrap())
        .or_default() += 1;

    let commands: Vec<Command> = second_part.lines().map(Command::from_str).collect();
    Puzzle {
        pairs,
        counts,
        instructions: commands,
    }
}

fn main() {
    let mut data = read_data("input.txt");
    let s1 = data.simulate(10);
    println!("s1: {}", s1);
    let s2 = data.simulate(30);
    println!("s2: {}", s2);
}
