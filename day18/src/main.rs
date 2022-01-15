use std::fs;

use snailfish::SnailFish;

fn main() {
    let file = fs::read_to_string("input.txt").expect("failed to read file");
    let mut snail = snailfish::SnailFish::multiple_line(&file);
    // let mut snail = snailfish::SnailFish::multiple_line(
    //     "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
    // [[[5,[2,8]],4],[5,[[9,9],0]]]
    // [6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
    // [[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
    // [[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
    // [[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
    // [[[[5,4],[7,7]],8],[[8,3],8]]
    // [[9,3],[[9,9],[6,[4,9]]]]
    // [[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
    // [[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]",
    // );
    let commands: Vec<&str> = file.lines().collect();

    let mut max_mag = 0;

    for i in 0..commands.len() {
        for j in 0..commands.len() {
            if i == j {
                continue;
            }
            let p1 = SnailFish::single(commands[i]);
            let p2 = SnailFish::single(commands[j]);
            let mut r = p1 + p2;
            r.reduce();
            max_mag = max_mag.max(r.root.magnitude());
        }
    }
    println!("max: {}", max_mag);

    snail.reduce();
    println!("snail: {}", snail.root.magnitude());
}
