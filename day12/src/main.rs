use std::{fs, path};

#[derive(Debug, PartialEq)]
enum Node {
    Start,
    End,
    Big(String),
    Small(String),
}

impl Node {
    fn is_small(&self) -> bool {
        match self {
            Node::Small(_) => true,
            _ => false,
        }
    }
    fn is_end(&self) -> bool {
        match self {
            Node::End => true,
            _ => false,
        }
    }

    fn from_str(text: &str) -> Self {
        match text {
            "start" => Node::Start,
            "end" => Node::End,
            a => {
                if a == a.to_lowercase() {
                    Node::Small(a.to_string())
                } else {
                    Node::Big(a.to_string())
                }
            }
        }
    }
}

#[derive(Debug)]
struct Edge(usize, usize);

#[derive(Debug, Default, Clone)]
struct Path {
    data: Vec<usize>,
    passed_small: bool,
    finished: bool,
}

impl PartialEq for Path {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl Path {
    fn new(strating_index: usize) -> Self {
        Path {
            data: vec![strating_index],
            passed_small: false,
            finished: false,
        }
    }
    fn last(&self) -> usize {
        *self.data.last().unwrap()
    }

    // Forking maybe imposible because either the path contains already a small cave or the path is already complete
    fn fork(&self, node: &Node, index: usize) -> Option<Self> {
        if node == &Node::Start {
            return None;
        }
        if (self.data.contains(&index) && node.is_small() && self.passed_small) || self.finished {
            return None;
        }
        let passed = if self.data.contains(&index) && node.is_small() && !self.passed_small {
            true
        } else {
            self.passed_small
        };

        let mut data = self.data.to_vec();
        data.push(index);
        let finished = node.is_end();

        Some(Path {
            data,
            finished,
            passed_small: passed,
        })
    }
}

#[derive(Debug)]
struct Graph {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
}

impl Graph {
    fn from_file(filename: &str) -> Graph {
        let mut result = Graph {
            nodes: vec![],
            edges: vec![],
        };

        let file = fs::read_to_string(filename).expect("failed to read file");
        //         let file = "start-A
        // start-b
        // A-c
        // A-b
        // b-d
        // A-end
        // b-end";
        for line in file.lines() {
            let (f, s) = line.split_once('-').unwrap();

            let f_node = Node::from_str(f);
            let mut f1 = result.nodes.len();
            if result.nodes.contains(&f_node) {
                f1 = result
                    .nodes
                    .iter()
                    .enumerate()
                    .find(|(_, v)| f_node == **v)
                    .map(|x| x.0)
                    .unwrap();
            } else {
                result.nodes.push(f_node);
            }
            let s_node = Node::from_str(s);
            let mut s1 = result.nodes.len();
            if result.nodes.contains(&s_node) {
                s1 = result
                    .nodes
                    .iter()
                    .enumerate()
                    .find(|(_, v)| s_node == **v)
                    .map(|x| x.0)
                    .unwrap();
            } else {
                result.nodes.push(s_node);
            }

            result.edges.push(Edge(f1, s1));
        }
        result
    }

    fn walk(&self) -> usize {
        // First Find the start and the end index.
        let mut start_index = 0;

        for (index, value) in self.nodes.iter().enumerate() {
            if value == &Node::Start {
                start_index = index;
                break;
            }
        }

        let mut paths: Vec<Path> = vec![Path::new(start_index)];
        loop {
            let mut new_paths = Vec::new();
            for path in &paths {
                if path.finished {
                    new_paths.push(path.clone());
                    continue;
                }

                let edges_forward: Vec<&Edge> =
                    self.edges.iter().filter(|x| x.0 == path.last()).collect();
                let edges_backward: Vec<&Edge> =
                    self.edges.iter().filter(|x| x.1 == path.last()).collect();

                for edge in edges_forward {
                    if let Some(p) = path.fork(&self.nodes[edge.1], edge.1) {
                        new_paths.push(p);
                    }
                }
                for edge in edges_backward {
                    if let Some(p) = path.fork(&self.nodes[edge.0], edge.0) {
                        new_paths.push(p);
                    }
                }
            }
            paths = new_paths.to_vec();

            if paths.iter().all(|x| x.finished) {
                break;
            }
        }
        paths.len()
    }
}

fn main() {
    let graph = Graph::from_file("input.txt");
    let r = graph.walk();

    println!("r: {}", r)
}
