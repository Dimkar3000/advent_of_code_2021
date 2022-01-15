use std::collections::VecDeque;
use std::fmt::Display;
use std::ops::{Add, AddAssign};

#[derive(Debug, Clone)]
pub enum Node {
    Leaf(u64),
    Junction(Vec<Node>),
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::Leaf(v) => write!(f, "{}", *v),
            Node::Junction(data) => {
                write!(f, "[").unwrap();
                for i in 0..data.len() {
                    write!(f, "{}", data[i]).unwrap();
                    if i < data.len() - 1 {
                        write!(f, ",").unwrap();
                    }
                }
                write!(f, "]")
            }
        }
    }
}

impl Node {
    pub fn magnitude(&self) -> u64 {
        match self {
            Node::Leaf(v) => *v,
            Node::Junction(data) => {
                3 * data.first().unwrap().magnitude() + 2 * data.last().unwrap().magnitude()
            }
        }
    }

    fn push(&mut self, n: Node) -> bool {
        match self {
            Node::Leaf(_) => false,
            Node::Junction(d) => {
                d.push(n);
                true
            }
        }
    }

    fn split(&mut self) -> bool {
        match self {
            Node::Leaf(v) => {
                if *v >= 10 {
                    *self = Node::Junction(vec![Node::Leaf(*v / 2), Node::Leaf(*v - *v / 2)]);

                    true
                } else {
                    false
                }
            }
            Node::Junction(nodes) => {
                for i in nodes {
                    if i.split() {
                        return true;
                    }
                }
                false
            }
        }
    }

    fn explode(&mut self, indentation: usize) -> Option<(u64, u64)> {
        match self {
            Node::Leaf(_) => None,
            Node::Junction(data) => {
                if indentation < 4 {
                    for i in 0..data.len() {
                        if let Some(mut res) = data[i].explode(indentation + 1) {
                            if i > 0 && res.0 > 0 {
                                data[i - 1].propagate_right(res.0);
                                res.0 = 0;
                            }

                            if i < data.len() - 1 && res.1 > 0 {
                                data[i + 1].propagate_left(res.1);
                                res.1 = 0;
                            }
                            return Some(res);
                        }
                    }
                    None
                } else {
                    if data.len() == 2 {
                        match (&mut data[0].clone(), &mut data[1].clone()) {
                            (Node::Leaf(v1), Node::Leaf(v2)) => {
                                *self = Node::Leaf(0);
                                Some((*v1, *v2))
                            }
                            (Node::Leaf(v), p2) => {
                                if let Some(res) = p2.explode(indentation + 1) {
                                    *v = res.0;
                                    return Some((0, res.1));
                                }
                                None
                            }
                            (p1, Node::Leaf(v)) => {
                                if let Some(res) = p1.explode(indentation + 1) {
                                    *v += res.1;
                                    return Some((res.0, 0));
                                }
                                None
                            }
                            (p1, p2) => {
                                let v = p1.explode(indentation + 1);
                                if v.is_none() {
                                    return p2.explode(indentation + 1);
                                } else {
                                    None
                                }
                            }
                        }
                    } else {
                        // Not a botom node so the same with higher indentation
                        for i in 0..data.len() {
                            if let Some(mut res) = data[i].explode(indentation + 1) {
                                if i > 0 && res.0 > 0 {
                                    data[i - 1].propagate_right(res.0);
                                    res.0 = 0;
                                }

                                if i < data.len() - 1 && res.1 > 0 {
                                    data[i + 1].propagate_left(res.1);
                                    res.1 = 0;
                                }
                                return Some(res);
                            }
                        }
                        None
                    }
                }
            }
        }
    }

    fn propagate_right(&mut self, value: u64) {
        match self {
            Node::Leaf(v) => *v += value,
            Node::Junction(data) => data.last_mut().unwrap().propagate_right(value),
        }
    }

    fn propagate_left(&mut self, value: u64) {
        match self {
            Node::Leaf(v) => *v += value,
            Node::Junction(data) => data.first_mut().unwrap().propagate_left(value),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SnailFish {
    pub root: Node,
}

impl Display for SnailFish {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.root)
    }
}

impl Add for SnailFish {
    type Output = SnailFish;

    fn add(self, rhs: SnailFish) -> Self::Output {
        SnailFish {
            root: Node::Junction(vec![self.root, rhs.root]),
        }
    }
}

impl AddAssign for SnailFish {
    fn add_assign(&mut self, rhs: Self) {
        *self = self.clone() + rhs;
    }
}

impl<'a> SnailFish {
    pub fn explode(&mut self) -> bool {
        self.root.explode(0).is_some()
    }

    pub fn split(&mut self) -> bool {
        self.root.split()
    }

    pub fn reduce(&mut self) {
        loop {
            while self.explode() {}
            if !self.split() {
                break;
            }
        }
    }

    pub fn raw_multiple_line<T: AsRef<str>>(input: T) -> Self {
        let input = input.as_ref();
        let mut data: VecDeque<SnailFish> = input.lines().map(SnailFish::single).collect();

        if let Some(mut result) = data.pop_front() {
            while let Some(other) = data.pop_front() {
                result += other;
            }
            result
        } else {
            panic!("multiline faild: {:?}", data)
        }
    }

    pub fn multiple_line<T: AsRef<str>>(input: T) -> Self {
        let input = input.as_ref();
        let mut data: VecDeque<SnailFish> = input.lines().map(SnailFish::single).collect();

        if let Some(mut result) = data.pop_front() {
            while let Some(other) = data.pop_front() {
                result += other;
                result.reduce();
            }
            result
        } else {
            panic!("multiline faild: {:?}", data)
        }
    }

    pub fn single<T: AsRef<str>>(input: T) -> Self {
        let input: &str = input.as_ref().trim();
        let mut stack: Vec<Node> = Vec::new();
        let mut index = 0;

        while let Some(c) = input.chars().nth(index) {
            if c == '[' {
                stack.push(Node::Junction(Vec::new()));
                index += 1;
            } else if c.is_digit(10) {
                let number = input
                    .chars()
                    .skip(index)
                    .take_while(|c| c.is_digit(10))
                    .collect::<String>();
                let n = u64::from_str_radix(&number, 10).unwrap();
                stack.push(Node::Leaf(n));
                index += number.len()
            } else if c == ',' {
                let child = stack.pop().unwrap();
                let mut father = stack.pop().unwrap();
                father.push(child);
                stack.push(father);
                index += 1;
            } else if c == ']' {
                index += 1;

                let child = stack.pop().unwrap();
                let mut father = stack.pop().unwrap();
                father.push(child);
                stack.push(father);

                if index == input.len() {
                    break;
                }
            }
        }
        SnailFish {
            root: stack.pop().unwrap(),
        }
    }
}
