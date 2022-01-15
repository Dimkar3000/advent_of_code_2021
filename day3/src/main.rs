use std::{
    fs::File,
    io::{BufRead, BufReader},
};

static INPUT: &str = "input.txt";

fn read_file(file_name: &str) -> Vec<usize> {
    let f = File::open(file_name).expect("failed to open file");
    let reader = BufReader::new(f);

    // Info: Assumes 12 bits per line.
    let binary_string_to_number = |x: String| {
        let mut result = 0;
        for i in 0..12 {
            result <<= 1;
            if x.chars().nth(i).unwrap() == '1' {
                result |= 1;
            }
        }

        result
    };
    reader
        .lines()
        .filter(|x| x.is_ok())
        .map(|x| x.unwrap())
        .map(binary_string_to_number)
        .collect()
}

fn part1() {
    let file = File::open(INPUT).expect("failed to read a file");
    let numbers: Vec<u32> = BufReader::new(file)
        .lines()
        .filter_map(|x| u32::from_str_radix(&x.unwrap_or_default(), 2).ok())
        .collect();

    let mut counters = [0usize; 12];
    for element in &numbers {
        let mut mask = 0b1;
        for i in 0..12 {
            if element & mask > 0 {
                counters[11 - i] += 1;
            }
            mask <<= 1;
        }
    }
    let mut gama = 0;
    let mut epsilon = 0;
    for counter in counters {
        if counter > numbers.len() / 2 {
            gama = (gama << 1) | 1;
            epsilon <<= 1;
        } else {
            epsilon = (epsilon << 1) | 1;
            gama <<= 1;
        }
    }
    println!("Result 1: {:?}", epsilon * gama);
}

type Handler = dyn Fn(usize, usize) -> bool;

fn generate_value(numbers: &[usize], more_ones: &Handler, less_ones: &Handler) -> usize {
    let mut clone = numbers.to_vec();
    let mut position = 0;
    while clone.len() > 1 {
        let ones_count = clone
            .iter()
            .filter(|&&x| x & (1 << (11 - position)) > 0)
            .count();
        if 2 * ones_count >= clone.len() {
            clone = clone
                .iter()
                .filter(|&&x| more_ones(x, position))
                .map(|&x| x)
                .collect();
        } else {
            clone = clone
                .iter()
                .filter(|&&x| less_ones(x, position))
                .map(|&x| x)
                .collect();
        }
        position += 1;
    }

    clone[0]
}

fn part2() {
    let numbers = read_file(INPUT);

    let one_at_pos = |x: usize, position: usize| x & (1 << (11 - position)) > 0;
    let zero_at_pos = |x: usize, position: usize| x & (1 << (11 - position)) == 0;
    let oxygen = generate_value(&numbers, &one_at_pos, &zero_at_pos);
    let co2 = generate_value(&numbers, &zero_at_pos, &one_at_pos);

    println!("Result 2: {}", oxygen * co2);
}

fn main() {
    part1();
    part2();
}
