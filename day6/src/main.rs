use std::fs::{self, File};

fn main() {
    // let input = "3,4,3,1,2";
    let input = fs::read_to_string("input.txt").expect("failed to read file");
    let fishes: Vec<usize> = input
        .split(',')
        .map(|x| usize::from_str_radix(x, 10))
        // .filter(|x| x.is_ok())
        .map(|x| x.unwrap())
        .collect();
    let mut registry = vec![0; 9];
    for i in fishes {
        registry[i] += 1;
    }
    for _ in 0..256 {
        registry.rotate_left(1);
        registry[6] += registry[8];
    }
    println!("Size: {}", registry.iter().sum::<usize>());
}
