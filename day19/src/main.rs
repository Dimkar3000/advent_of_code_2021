struct Board {
    world: Vec<Vec<Vec<bool>>>,
}

impl Board {
    fn from_points(points: Vec<Point>, board_size: usize) -> Self {
        let mut world = vec![vec![vec![false; board_size]; board_size]; board_size];
        for p in points {
            world[p.x as usize + board_size / 2][p.y as usize + board_size / 2]
                [p.z as usize + board_size / 2] = true
        }
        Board { world }
    }
}

// X,Y,Z are relative Values
#[derive(Debug, Clone, Default)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl Point {
    fn get_all_rotations(self: Point) -> Vec<Self> {
        let x = self.x;
        let y = self.y;
        let z = self.z;
        vec![
            Point { x, y, z },
            Point { x: -y, y: x, z },
            Point { x: -x, y: -y, z },
            Point { x: y, y: -x, z },
            Point { x: x, y: -z, z: y },
            Point { x: z, y: x, z: y },
            Point { x: -x, y: z, z: y },
            Point { x: -z, y: -x, z: y },
            Point { x: x, y: -y, z: -z },
            Point { x: y, y: x, z: -z },
            Point { x: -x, y, z: -z },
            Point { x: x, y: z, z: -y },
            Point { x: -z, y: x, z: -y },
            Point { x: z, y: -x, z: -y },
            Point { x: -z, y, z: x },
            Point { x: -y, y: -z, z: x },
            Point { x: z, y: -y, z: x },
            Point { x: y, y: z, z: x },
            Point { x: -y, y: z, z: -x },
            Point { x: y, y: -z, z: -x },
            Point { x: z, y, z: -x },
            Point {
                x: -y,
                y: -x,
                z: -z,
            },
            Point {
                x: -x,
                y: -z,
                z: -y,
            },
            Point {
                x: -z,
                y: -y,
                z: -x,
            },
        ]
    }
}

impl Scanner {
    fn rotate(&mut self) {}
    // fn abs_points(&self) -> Vec<Point> {
    //     assert!(self.center.is_some());

    //     let center = self.center.unwrap();
    //     self.beacons
    //         .iter()
    //         .map(|p| Point { x: center.x + p.x })
    //         .collect()
    // }
}

#[derive(Debug)]
struct Scanner {
    name: String,

    center: Option<Point>,

    // Values centered to the scanner
    beacons: Vec<Point>,
}

fn read_input(text: &str) -> Vec<Scanner> {
    let mut results = Vec::new();
    let mut lines = text.lines().peekable();
    loop {
        if lines.peek().is_none() {
            break;
        }
        // --- scanner 0 ---
        let name_line = lines.next().unwrap();
        let name = name_line[4..(name_line.len() - 4)].to_owned();
        let mut beacons = Vec::new();
        while let Some(line) = lines.next() {
            println!("line: {}", line);
            if line.trim().is_empty() {
                break;
            }

            let mut values = line.trim().splitn(3, ',');
            let x = i64::from_str_radix(values.next().unwrap(), 10).unwrap();
            let y = i64::from_str_radix(values.next().unwrap(), 10).unwrap();
            let z = i64::from_str_radix(values.next().unwrap(), 10).unwrap();

            beacons.push(Point { x, y, z })
        }

        results.push(Scanner {
            name,
            center: None,
            beacons,
        })
    }
    results
}

fn test_scanners(scanners: &mut Vec<Scanner>) {
    let mut origin = scanners.remove(0);
    origin.center = Some(Point::default());

    // Every beacon of beacon 0 is unique
    let mut unique_points = origin.beacons.to_vec();
}

fn main() {
    let file = "--- scanner 0 ---
    -1,-1,1
    -2,-2,2
    -3,-3,3
    -2,-3,1
    5,6,-4
    8,0,7
    
    --- scanner 0 ---
    1,-1,1
    2,-2,2
    3,-3,3
    2,-1,3
    -5,4,-6
    -8,-7,0
    
    --- scanner 0 ---
    -1,-1,-1
    -2,-2,-2
    -3,-3,-3
    -1,-3,-2
    4,6,5
    -7,0,8
    
    --- scanner 0 ---
    1,1,-1
    2,2,-2
    3,3,-3
    1,3,-2
    -4,-6,5
    7,0,8
    
    --- scanner 0 ---
    1,1,1
    2,2,2
    3,3,3
    3,1,2
    -6,-4,-5
    0,7,-8";
    let mut d = read_input(file);
    println!("{:?}", d);

    let mut m = d.get_mut(0).unwrap();
    println!("{:?}", d);
    m.rotate();
}
