use std::{fmt, fs};

#[derive(Debug)]
enum Instr {
    X(usize),
    Y(usize),
}

#[derive(Debug)]
struct Puzzle {
    points: Vec<(usize, usize)>,
    instr: Vec<Instr>,
}

impl fmt::Display for Puzzle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let width = self.points.iter().map(|x| x.0).max().unwrap();
        let height = self.points.iter().map(|x| x.1).max().unwrap();

        for j in 0..=height {
            for i in 0..=width {
                if self.points.contains(&(i, j)) {
                    write!(f, "#").unwrap();
                } else {
                    write!(f, ".").unwrap();
                }
            }
            write!(f, "\n").unwrap();
        }
        write!(f, "")
    }
}

impl Puzzle {
    fn solve(&mut self) {
        while self.instr.len() > 0 {
            self.step();
        }
    }

    fn step(&mut self) -> usize {
        if let Some(inst) = self.instr.pop() {
            // println!("{:?}", inst);
            match inst {
                Instr::X(x) => {
                    self.points.iter_mut().for_each(|(xi, _)| {
                        if *xi > x {
                            *xi = x - (*xi - x)
                        }
                    });
                }
                Instr::Y(y) => {
                    self.points.iter_mut().for_each(|(_, yi)| {
                        if *yi > y {
                            // println!("fliping point: {:?}", (*xi, *yi));
                            *yi = y - (*yi - y);
                            // println!("flipped to: {:?}", (*xi, *yi));
                        }
                    });
                }
            }
        }
        self.points.sort();
        self.points
            .dedup_by(|(x0, x1), (y0, y1)| *x0 == *y0 && *x1 == *y1);

        self.points.len()
    }
}

fn read_data(filename: &str) -> Puzzle {
    let file = fs::read_to_string(filename).expect("failed to read input.txt");

    let (first_part, second_part) = file.split_once("\n\n").unwrap();
    let points: Vec<(usize, usize)> = first_part
        .lines()
        .map(|x| {
            x.split_once(',')
                .map(|(a, b)| {
                    (
                        usize::from_str_radix(a, 10).unwrap(),
                        usize::from_str_radix(b, 10).unwrap(),
                    )
                })
                .unwrap()
        })
        .collect();
    let mut instructions = second_part
        .lines()
        .map(|x| x.split_once('=').unwrap())
        .map(|(i, d)| {
            let number = usize::from_str_radix(d, 10).unwrap();
            if i.contains('x') {
                Instr::X(number)
            } else {
                Instr::Y(number)
            }
        })
        .collect::<Vec<_>>();
    instructions.reverse();

    Puzzle {
        points,
        instr: instructions,
    }
}

fn main() {
    let mut data = read_data("input.txt");
    let s = data.step();
    println!("s: {}", s);
    data.solve();
    println!("{}", data);
}
