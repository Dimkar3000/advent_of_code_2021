use std::{
    fs::File,
    io::{BufRead, BufReader},
};

static INPUT1: &str = "input1.txt";
static INPUT2: &str = "input2.txt";

fn part1() {
    let file = File::open(INPUT1).expect("failed to read file");
    let reader = BufReader::new(file);

    let mut forward = 0;
    let mut depth = 0;
    for line in reader.lines() {
        if let Ok(command) = line {
            let (word, num) = command
                .split_once(' ')
                .expect("failed to split the word in 2");
            let count = num.parse::<u32>().expect("failed to convert to number");
            match word {
                "forward" => forward += count,
                "down" => depth += count,
                "up" => depth -= count,
                _ => (),
            }
        }
    }
    println!("{}", forward * depth);
}
fn part2() {
    let file = File::open(INPUT2).expect("failed to read file");
    let reader = BufReader::new(file);

    let mut forward = 0;
    let mut depth = 0;
    let mut aim = 0;
    for line in reader.lines() {
        if let Ok(command) = line {
            let (word, num) = command
                .split_once(' ')
                .expect("failed to split the word in 2");
            let count = num.parse::<u32>().expect("failed to convert to number");
            match word {
                "forward" => {
                    forward += count;
                    depth += aim * count;
                }
                "down" => aim += count,
                "up" => aim -= count,
                _ => (),
            }
        }
    }
    println!("{}", forward * depth);
}

fn main() {
    part1();
    part2();
}
