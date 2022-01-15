// Credits: https://blog.jpages.eu/articles/advent-of-code-2021-day-17
// Boring enough not to care

use std::ops::RangeInclusive;

struct TargetArea {
    x: RangeInclusive<i64>,
    y: RangeInclusive<i64>,
}

impl TargetArea {
    fn problem_data() -> Self {
        let xmin = 70;
        let xmax = 125;
        let ymin = -159;
        let ymax = -121;

        TargetArea {
            x: xmin..=xmax,
            y: ymin..=ymax,
        }
    }
}

// calculate steps based on speed
fn x_sequence(initial_speed: i64) -> Vec<i64> {
    let mut x = 0;
    let mut seq = vec![x];
    let mut current_speed = initial_speed;

    while current_speed != 0 {
        x += current_speed;
        if current_speed > 0 {
            current_speed = 0.max(current_speed - 1);
        } else {
            current_speed = 0.min(current_speed + 1);
        }
        seq.push(x);
    }

    seq
}

// calculate steps based on speed of decent
fn y_sequence(initial_speed: i64, ymin: i64) -> Vec<i64> {
    let mut y = 0;
    let mut seq = vec![y];
    let mut vy = initial_speed;

    while y >= ymin {
        y += vy;
        vy -= 1;
        seq.push(y);
    }

    seq
}

fn in_target(initial_speed_y: i64, target_area: &TargetArea, x_seqs: &Vec<Vec<i64>>) -> usize {
    let mut yin = Vec::new();
    let mut xin_count = 0;

    let yseq = y_sequence(initial_speed_y, *target_area.y.start());
    for i in 0..yseq.len() {
        if target_area.y.contains(&yseq[i]) {
            yin.push(i);
        }
    }

    for vx0 in 0..=*target_area.x.end() {
        for i in &yin {
            let xi = match x_seqs[vx0 as usize].get(*i) {
                Some(v) => *v,
                None => *x_seqs[vx0 as usize].last().unwrap(),
            };

            if target_area.x.contains(&xi) {
                xin_count += 1;
                break;
            }
        }
    }
    xin_count
}

fn solve_part1() -> i64 {
    let target_area = TargetArea::problem_data();
    let mut ymax = vec![0];

    let mut x_seqs = Vec::new();
    for vx0 in 0..=*target_area.x.end() {
        x_seqs.push(x_sequence(vx0));
    }

    for vy0 in 0..-target_area.y.start() {
        if in_target(vy0, &target_area, &x_seqs) > 0 {
            ymax.push(vy0 * (vy0 + 1) / 2);
        }
    }

    *ymax.iter().max().unwrap()
}

pub fn solve_part2() -> usize {
    let target_area = TargetArea::problem_data();
    let mut count = 0;

    let mut x_seqs = Vec::new();
    for vx0 in 0..=*target_area.x.end() {
        x_seqs.push(x_sequence(vx0));
    }

    let mut vy0 = *target_area.y.start();
    while vy0 <= -target_area.y.start() {
        count += in_target(vy0, &target_area, &x_seqs);
        vy0 += 1;
    }

    count
}

fn main() {
    // target area: x=70..125, y=-159..-121
    let n = solve_part1();
    let x = solve_part2();

    println!("n: {}", n);
    println!("x: {}", x);
}
