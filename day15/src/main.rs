use std::fs;

use pathfinding::prelude::dijkstra;

fn read_data(filename: &str) -> Vec<Vec<u32>> {
    let file = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";
    let file = fs::read_to_string(filename).expect("failed to read file");

    file.lines()
        .map(|x| x.chars().map(|y| y.to_digit(10).unwrap()).collect())
        .collect()
}

fn generate_neighbors(x: usize, y: usize, limit_x: usize, limit_y: usize) -> Vec<(usize, usize)> {
    let mut result = vec![];
    if x + 1 < limit_x {
        result.push((x + 1, y));
    }
    if y + 1 < limit_y {
        result.push((x, y + 1));
    }
    if x > 0 {
        result.push((x - 1, y));
    }
    if y > 0 {
        result.push((x, y - 1));
    }

    result
}

fn get_cost(data: &Vec<Vec<u32>>, x: usize, y: usize) -> usize {
    let i = x % data[0].len();
    let j = y % data.len();
    let ai = x / data[0].len();
    let aj = y / data.len();
    let mut v = data[i][j] as usize + ai + aj;
    if v > 9 {
        v %= 9;
    }

    v
}

fn main() {
    let data = read_data("input.txt");
    let goal = (data[0].len() * 5 - 1, data.len() * 5 - 1);
    println!("{:?}", goal);
    // goal.0 *= 5;
    // goal.1 *= 5;
    let result = dijkstra(
        &(0, 0),
        |&(x, y)| {
            generate_neighbors(x, y, data[0].len() * 5, data.len() * 5)
                .into_iter()
                .map(|p| (p, get_cost(&data, p.1, p.0)))
        },
        |&p| (p.0 as usize, p.1 as usize) == goal,
    );
    println!("{:?}", result.unwrap().1);
}
