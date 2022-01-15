use std::{
    fs::{read_to_string, File},
    io::{BufRead, BufReader},
};

static FILE_NAME: &str = "input.txt";
static FILE_NAME2: &str = "input2.txt";

fn part1() {
    let file = File::open(FILE_NAME).expect("file not found");

    let reader = BufReader::new(file);

    let mut counter = 0u32;
    let mut current = u32::MAX;
    for line in reader.lines() {
        if let Ok(word) = line {
            let test = word.parse::<u32>().unwrap();
            if test > current {
                counter += 1;
            }
            current = test;
        }
    }
    println!("{}", counter);
}

fn part2() {
    let file = read_to_string(FILE_NAME2).expect("failed to read file");
    let numbers = file
        .split('\n')
        .filter_map(|w| w.parse::<u32>().ok())
        .collect::<Vec<u32>>();

    let mut counter = 0u32;
    let mut current = u32::MAX;
    for nums in numbers.as_slice().windows(3) {
        let x = nums[0];
        let y = nums[1];
        let z = nums[2];
        let s = x + y + z;
        if s > current {
            counter += 1;
        }
        current = s;
    }
    println!("{}", counter);
}

fn main() {
    part1();
    part2();
}
