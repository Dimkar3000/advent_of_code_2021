use std::fs;

fn read_input() -> Vec<i32> {
    let input = fs::read_to_string("input.txt").expect("failed to read file");
    // let input = "16,1,2,0,4,2,7,1,2,14";
    input
        .split(',')
        .map(|x| i32::from_str_radix(x, 10).unwrap())
        .collect()
}

fn get_conv_point(values: &mut [i32]) -> i32 {
    values.sort();
    values[values.len() / 2]
}

fn main() {
    let data = read_input();
    let ma = *data.iter().max().unwrap();
    let mi = *data.iter().min().unwrap();
    let mut d = ma;
    let mut current = i32::MAX;
    println!("max:{}", ma);
    println!("min:{}", mi);
    println!("avg:{}", (ma - mi) / 2);
    for i in mi..ma {
        let v: i32 = data
            .iter()
            .map(|&x| (x - i).abs())
            .map(|x| (1..=x).sum::<i32>())
            .sum();
        if v < current {
            d = i;
            current = v;
        }
    }

    println!("d: {}", d);
    println!("avg: {}", current);
}
