use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Solver {
    columns: usize,
    rows: usize,
    data: Vec<u8>,
}

impl Solver {
    fn new_from_files(filename: &str) -> Self {
        let file = File::open(filename).expect(&format!("failed to open file {}", filename));
        let reader = BufReader::new(file);
        let mut data: Vec<u8> = Vec::new();
        let mut rows = 0;
        let mut columns = 0;

        //         let reader = "2199943210
        // 3987894921
        // 9856789892
        // 8767896789
        // 9899965678";

        for l in reader.lines() {
            let l = l.unwrap();
            data.extend(l.chars().map(|x| x.to_digit(10).unwrap() as u8));
            columns = l.len();
            rows += 1;
        }

        // let rows = d;
        Solver {
            columns,
            rows,
            data,
        }
    }

    fn find_basins(&self) -> Vec<usize> {
        // Create a mask of points that the algorithm has visited
        let mut visited = vec![false; self.data.len()];
        let mut basins: Vec<usize> = Vec::new();

        let create_basin = |index: usize, visited: &mut [bool]| -> usize {
            let first_row: Vec<usize> = (0..self.columns).collect();
            let last_row: Vec<usize> =
                ((self.columns * (self.rows - 1))..self.columns * self.rows).collect();
            let first_column = (0..(self.columns * self.rows))
                .step_by(self.columns)
                .collect::<Vec<usize>>();
            let last_column = (self.columns - 1..(self.columns * self.rows))
                .step_by(self.columns)
                .collect::<Vec<usize>>();

            let mut current = vec![index];
            let mut size = 0;
            loop {
                let mut new_neighbors: Vec<usize> = Vec::new();
                for i in current {
                    // First mark the index we work with as visited so no more processinf for it.
                    if visited[i] || self.data[i] == 9 {
                        continue;
                    } else {
                        visited[i] = true;
                        size += 1;
                    }
                    // Next find all his valid neighbords
                    // Corners
                    if i == 0 {
                        new_neighbors.push(i + 1);
                        new_neighbors.push(i + self.columns);
                    } else if i == self.columns - 1 {
                        new_neighbors.push(i - 1);
                        new_neighbors.push(i + self.columns);
                    } else if i == self.columns * (self.rows - 1) {
                        new_neighbors.push(i + 1);
                        new_neighbors.push(i - self.columns);
                    } else if i == self.columns * self.rows - 1 {
                        new_neighbors.push(i - 1);
                        new_neighbors.push(i - self.columns);
                    }
                    // Sides
                    else if first_row.contains(&i) {
                        new_neighbors.push(i + 1);
                        new_neighbors.push(i - 1);
                        new_neighbors.push(i + self.columns);
                    } else if last_row.contains(&i) {
                        new_neighbors.push(i + 1);
                        new_neighbors.push(i - 1);
                        new_neighbors.push(i - self.columns);
                    } else if first_column.contains(&i) {
                        new_neighbors.push(i + 1);
                        new_neighbors.push(i - self.columns);
                        new_neighbors.push(i + self.columns);
                    } else if last_column.contains(&i) {
                        new_neighbors.push(i - 1);
                        new_neighbors.push(i - self.columns);
                        new_neighbors.push(i + self.columns);
                    }
                    // Center
                    else {
                        new_neighbors.push(i + 1);
                        new_neighbors.push(i - 1);
                        new_neighbors.push(i - self.columns);
                        new_neighbors.push(i + self.columns);
                    }
                }
                new_neighbors.sort();
                new_neighbors.dedup();
                if new_neighbors.len() == 0 {
                    break;
                } else {
                    current = new_neighbors;
                }
            }

            size
        };

        for index in 0..self.data.len() {
            if self.data[index] < 9 && !visited[index] {
                basins.push(create_basin(index, &mut visited));
            }
        }
        basins
    }

    fn find_lowest_points(&self) -> Vec<u8> {
        let columns = self.columns;
        let rows = self.rows;
        let data = &self.data;
        let mut results = Vec::new();
        let first_row: Vec<usize> = (0..columns).collect();
        let last_row: Vec<usize> = ((columns * (rows - 1))..columns * rows).collect();
        let first_column = (0..(columns * rows))
            .step_by(columns)
            .collect::<Vec<usize>>();
        let last_column = (columns - 1..(columns * rows))
            .step_by(columns)
            .collect::<Vec<usize>>();

        for (index, &value) in data.iter().enumerate() {
            // first line
            if first_row.contains(&index) {
                if index == 0 {
                    if value < data[columns] && value < data[1] {
                        results.push(value);
                    }
                } else if index == first_row[first_row.len() - 1] {
                    if value < data[columns - 2] && value < data[2 * columns - 1] {
                        results.push(value);
                    }
                } else {
                    if value < data[index - 1]
                        && value < data[index + 1]
                        && value < data[index + columns]
                    {
                        results.push(value);
                    }
                }
            }
            // first column
            else if first_column.contains(&index) {
                // last element
                if index == first_column[first_column.len() - 1] {
                    if value < data[first_column[first_column.len() - 2]]
                        && value < data[first_column[first_column.len() - 1] + 1]
                    {
                        results.push(value)
                    }
                } else {
                    if value < data[index + 1]
                        && value < data[index - columns]
                        && value < data[index + columns]
                    {
                        results.push(value);
                    }
                }
            }
            // last column
            else if last_column.contains(&index) {
                // last element
                if index == last_column[last_column.len() - 1] {
                    if value < data[last_column[last_column.len() - 2]]
                        && value < data[last_column[last_column.len() - 1] - 1]
                    {
                        results.push(value);
                    }
                } else {
                    if value < data[index - 1]
                        && value < data[index + columns]
                        && value < data[index - columns]
                    {
                        results.push(value)
                    }
                }
            }
            // last row
            else if last_row.contains(&index) {
                if value < data[index - 1]
                    && value < data[index + 1]
                    && value < data[index - columns]
                {
                    results.push(value)
                }
            }
            // inside
            else if value < data[index - 1]
                && value < data[index + 1]
                && value < data[index - columns]
                && value < data[index + columns]
            {
                results.push(value)
            }
        }
        results
    }
}

fn main() {
    let data = Solver::new_from_files("input.txt");

    let results = data.find_lowest_points();
    let s: u32 = results.iter().map(|x| (*x + 1) as u32).sum();
    let mut results2 = data.find_basins();
    results2.sort();
    let s2 =
        results2[results2.len() - 1] * results2[results2.len() - 2] * results2[results2.len() - 3];

    println!("lowest points: {}", s);
    println!("basins : {}", s2);
}
