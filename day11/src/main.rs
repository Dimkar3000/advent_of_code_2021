use std::fs;
use std::sync::mpsc::sync_channel;

fn read_data(filename: &str) -> Vec<Vec<u8>> {
    //     let d = "5483143223
    // 2745854711
    // 5264556173
    // 6141336146
    // 6357385478
    // 4167524645
    // 2176841721
    // 6882881134
    // 4846848554
    // 5283751526";

    let d = fs::read_to_string(filename).expect("failed to open file");

    d.lines()
        .map(|x| x.chars())
        .map(|x| x.map(|f| f.to_digit(10).unwrap() as u8).collect())
        .collect()
}

fn simulation_step(data: &mut Vec<Vec<u8>>) -> usize {
    let mut flashes = 0;
    for x in 0..data.len() {
        for y in 0..data[0].len() {
            data[x][y] += 1;
        }
    }
    let mut new_flashes;
    let mut loacked = Vec::new();
    loop {
        new_flashes = 0;
        for x in 0..data.len() {
            for y in 0..data[0].len() {
                if loacked.contains(&(x, y)) {
                    continue;
                }
                if data[x][y] > 9 {
                    new_flashes += 1;
                    flashes += 1;
                    data[x][y] = 0;
                    loacked.push((x, y));

                    let indexes = [
                        (x as isize - 1, y as isize - 1),
                        (x as isize - 1, y as isize),
                        (x as isize - 1, y as isize + 1),
                        (x as isize + 1, y as isize - 1),
                        (x as isize + 1, y as isize),
                        (x as isize + 1, y as isize + 1),
                        (x as isize, y as isize - 1),
                        (x as isize, y as isize + 1),
                    ];
                    for (x1, y1) in indexes {
                        if loacked.contains(&(x1 as usize, y1 as usize)) {
                            continue;
                        }
                        if let Some(d) = data.get_mut(x1 as usize) {
                            if let Some(d1) = d.get_mut(y1 as usize) {
                                *d1 += 1;
                            }
                        }
                    }
                }
            }
        }
        if new_flashes == 0 {
            break;
        }
    }
    flashes
}

fn simulate(data: &mut Vec<Vec<u8>>, iterations: usize) -> usize {
    let mut flashes = 0;
    for _ in 0..iterations {
        flashes += simulation_step(data);
    }
    flashes
}

fn synchronization(data: &mut Vec<Vec<u8>>) -> usize {
    let mut iteration = 1;

    while simulation_step(data) != data.len() * data[0].len() {
        iteration += 1;
    }

    iteration
}

fn main() {
    let mut d = read_data("input.txt");
    let flashes = simulate(&mut d.to_vec(), 100);
    let synch = synchronization(&mut d);
    println!("Flashes: {}", flashes);
    println!("Iteration: {}", synch);
}
