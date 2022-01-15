use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct Line {
    start_x: i32,
    start_y: i32,
    end_x: i32,
    end_y: i32,
}

fn read_data() -> Vec<Line> {
    //     let input = "0,9 -> 5,9
    // 8,0 -> 0,8
    // 9,4 -> 3,4
    // 2,2 -> 2,1
    // 7,0 -> 7,4
    // 6,4 -> 2,0
    // 0,9 -> 2,9
    // 3,4 -> 1,4
    // 0,0 -> 8,8
    // 5,5 -> 8,2";

    let file = File::open("input.txt").expect("failed to open file");
    let reader = BufReader::new(file);
    let proccess_line = |x: String| -> Line {
        // proccess a line to numbers
        let (start, end) = x.split_once(" -> ").expect("failed to split line");
        let (start_x_str, start_y_str) = start.split_once(',').unwrap();
        let (end_x_str, end_y_str) = end.split_once(',').unwrap();

        let start_x = start_x_str.parse().unwrap();
        let start_y = start_y_str.parse().unwrap();
        let end_x = end_x_str.parse().unwrap();
        let end_y = end_y_str.parse().unwrap();

        Line {
            start_x,
            start_y,
            end_x,
            end_y,
        }
    };

    reader
        // input
        .lines()
        .map(|x| x.unwrap())
        .map(proccess_line)
        .collect()
}

fn calculate_distance(lines: &[Line],use_diagnonal:bool) -> i32 {
    let mut data: HashMap<(i32, i32), i32> = HashMap::new();

    for l in lines {
        // Column
        if l.start_x == l.end_x {
            let start = i32::min(l.start_y, l.end_y);
            let end = i32::max(l.start_y, l.end_y);
            for index in start..=end {
                let v = data.entry((l.start_x, index)).or_insert(0);
                *v += 1;
            }
        }
        // Row
        else if l.start_y == l.end_y {
            let start = i32::min(l.start_x, l.end_x);
            let end = i32::max(l.start_x, l.end_x);
            for index in start..=end {
                let v = data.entry((index, l.start_y)).or_insert(0);
                *v += 1;
            }
        }
        // Diagnonal
        else if use_diagnonal{
            let mut current_x = l.start_x ;
            let mut current_y = l.start_y ;
            let step_x:i32 = if l.start_x < l.end_x { 1 } else { -1 };
            let step_y:i32 = if l.start_y < l.end_y { 1 } else { -1 };

            while current_x != l.end_x && current_y != l.end_y {
                let v = data.entry((current_x, current_y)).or_insert(0);
                *v += 1;

                current_x += step_x;
                current_y += step_y;
            }
            let v = data.entry((current_x, current_y)).or_insert(0);
            *v += 1;
        }
    }
    // println!("map:\n{:?}",data);
    data.values().filter(|&&x| x > 1).count() as i32
}

fn main() {
    let data = read_data();
    let result1 = calculate_distance(&data,false);
    let result2 = calculate_distance(&data,true);
    println!("Result 1: {}", result1);
    println!("Result 2: {}", result2);
}
