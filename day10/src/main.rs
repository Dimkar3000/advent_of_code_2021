use std::fs;

struct Line {
    data: Vec<ParenType>,
}

impl Line {
    fn new(line: &str) -> Self {
        let data = line
            .chars()
            .map(|x| match x {
                '(' => ParenType::LParen,
                ')' => ParenType::RParen,
                '[' => ParenType::LBracket,
                ']' => ParenType::RBracket,
                '{' => ParenType::LCurly,
                '}' => ParenType::RCurly,
                '<' => ParenType::LArrow,
                '>' => ParenType::RArrow,
                a => unreachable!("{}", a),
            })
            .collect();
        Line { data }
    }

    fn status(&self) -> Status {
        let mut stack = Vec::new();
        for i in &self.data {
            if i.is_left() {
                stack.push(i);
            } else {
                if let Some(t) = stack.pop() {
                    if !t.matches(i) {
                        let score = i.score();
                        return Status::Illegal(score);
                    }
                } else {
                    return Status::Incomplete;
                }
            }
        }

        if stack.len() == 0 {
            Status::Legal
        } else {
            Status::Incomplete
        }
    }

    fn auto_score(&self) -> usize {
        let mut stack = Vec::new();
        for i in &self.data {
            if i.is_left() {
                stack.push(i);
            } else {
                if let Some(t) = stack.pop() {
                    if !t.matches(i) {
                        return 0;
                    }
                } else {
                    return 0;
                }
            }
        }

        if stack.len() == 0 {
            0
        } else {
            let mut scores = 0;
            while let Some(l) = stack.pop() {
                let s = match l {
                    ParenType::LParen => 1,
                    ParenType::LBracket => 2,
                    ParenType::LCurly => 3,
                    ParenType::LArrow => 4,
                    _ => 0,
                };

                scores = scores * 5 + s;
            }

            scores
        }
    }
}

enum Status {
    Legal,
    Illegal(usize),
    Incomplete,
}

impl Status {
    fn is_illegal(&self) -> bool {
        match self {
            &Status::Illegal(_) => true,
            _ => false,
        }
    }

    fn is_incomplete(&self) -> bool {
        match self {
            &Status::Incomplete => true,
            _ => false,
        }
    }

    fn score(&self) -> usize {
        match self {
            &Status::Illegal(a) => a,
            _ => 0,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum ParenType {
    LParen,
    LBracket,
    LCurly,
    LArrow,
    RParen,
    RBracket,
    RCurly,
    RArrow,
}

impl ParenType {
    fn is_left(&self) -> bool {
        self == &ParenType::LParen
            || self == &ParenType::LBracket
            || self == &ParenType::LCurly
            || self == &ParenType::LArrow
    }

    fn matches(&self, other: &ParenType) -> bool {
        match (self, other) {
            (ParenType::LParen, ParenType::RParen) => true,
            (ParenType::LBracket, ParenType::RBracket) => true,
            (ParenType::LCurly, ParenType::RCurly) => true,
            (ParenType::LArrow, ParenType::RArrow) => true,

            _ => false,
        }
    }

    fn score(&self) -> usize {
        match self {
            ParenType::RParen => 3,
            ParenType::RBracket => 57,
            ParenType::RCurly => 1197,
            ParenType::RArrow => 25137,
            a => unreachable!("{:?}", a),
        }
    }
}

fn read_data(filename: &str) -> Vec<Line> {
    fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(Line::new)
        .collect()
}

fn main() {
    let data = read_data("input.txt");
    let score: usize = data
        .iter()
        .map(|x| x.status())
        .filter(|x| x.is_illegal())
        .map(|x| x.score())
        .sum();
    let mut d2: Vec<usize> = data
        .iter()
        .filter(|&x| x.status().is_incomplete())
        .map(|x| x.auto_score())
        .collect();
    d2.sort();
    let score2 = d2[d2.len() / 2];

    println!("Score: {}", score);
    println!("Score 2: {}", score2);
}
