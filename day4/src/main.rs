use std::borrow::Borrow;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Board {
    squares: [u8; 25],
    marked: [bool; 25],
}

impl Board {
    fn new<T: Borrow<[u8]>>(s: T) -> Board {
        let mut squares = [0; 25];
        // Ensure that up to 25 values will be used but no more
        for (i, &v) in s.borrow().into_iter().enumerate().take(25) {
            squares[i] = v;
        }

        Board {
            squares,
            marked: [false; 25],
        }
    }
    fn mark_number(&mut self, n: u8) {
        for (index, value) in self.squares.iter().enumerate() {
            if *value == n {
                self.marked[index] = true;
            }
        }
    }
    fn reset(&mut self) {
        self.marked = [false; 25];
    }

    fn wins(&self) -> bool {
        let row1 =
            self.marked[0] && self.marked[1] && self.marked[2] && self.marked[3] && self.marked[4];
        let row2 =
            self.marked[5] && self.marked[6] && self.marked[7] && self.marked[8] && self.marked[9];
        let row3 = self.marked[10]
            && self.marked[11]
            && self.marked[12]
            && self.marked[13]
            && self.marked[14];
        let row4 = self.marked[15]
            && self.marked[16]
            && self.marked[17]
            && self.marked[18]
            && self.marked[19];
        let row5 = self.marked[20]
            && self.marked[21]
            && self.marked[22]
            && self.marked[23]
            && self.marked[24];
        let col1 = self.marked[0]
            && self.marked[5]
            && self.marked[10]
            && self.marked[15]
            && self.marked[20];
        let col2 = self.marked[1]
            && self.marked[6]
            && self.marked[11]
            && self.marked[16]
            && self.marked[21];
        let col3 = self.marked[2]
            && self.marked[7]
            && self.marked[12]
            && self.marked[17]
            && self.marked[22];
        let col4 = self.marked[3]
            && self.marked[8]
            && self.marked[13]
            && self.marked[18]
            && self.marked[23];
        let col5 = self.marked[4]
            && self.marked[9]
            && self.marked[14]
            && self.marked[19]
            && self.marked[24];
        row1 || row2 || row3 || row4 || row5 || col1 || col2 || col3 || col4 || col5
    }
}

#[derive(Debug)]
struct Game {
    moves: Vec<u8>,
    boards: Vec<Board>,
}

impl Game {

    fn reset(&mut self) {
        self.boards.iter_mut().for_each(|x| x.reset());
    }

    fn play_to_lose(&mut self) -> u32 {

        self.reset();
        
        let mut losser_index = 0;
        let mut loser_found = false;

        for &m in self.moves.iter() {
            if !loser_found {
                // Play the round
                for b in self.boards.iter_mut() {
                    b.mark_number(m);
                }
                // Get how many boards didn't won this round and there index
                let losers: Vec<_> = self
                    .boards
                    .iter()
                    .enumerate()
                    .filter(|(_, x)| !x.wins())
                    .collect();
                // If there is a loser keep his info for later
                if losers.len() == 1 {
                    loser_found = true;
                    losser_index = losers[0].0;
                }
            } 
            // If the loser is known we can process only his board
            else {
                self.boards[losser_index].mark_number(m);
                // When he eventually won, calculate his score
                if self.boards[losser_index].wins() {
                    let b = &self.boards[losser_index];
                    let mut s = 0u32;
                    for (i, bo) in b.marked.iter().enumerate() {
                        if !bo {
                            s += b.squares[i] as u32;
                        }
                    }
                    return s * m as u32;
                }
            }
        }
        0
    }
    fn play_to_win(&mut self) -> u32 {
        self.reset();

        for &m in self.moves.iter() {
            for b in self.boards.iter_mut() {
                b.mark_number(m);
                if b.wins() {
                    let mut s = 0u32;
                    for (i, bo) in b.marked.iter().enumerate() {
                        if !bo {
                            s += b.squares[i] as u32;
                        }
                    }
                    return s * m as u32;
                }
            }
        }
        0
    }
}

fn read_data() -> Game {
    let filename = "input.txt";
    let file = File::open(filename).expect("failed to open file");
    let reader = BufReader::new(file);

    let mut lines = reader.lines();
    let first_line = lines.next().unwrap().unwrap();
    let moves: Vec<u8> = first_line
        .split(',')
        .map(|x| u8::from_str_radix(x, 10).unwrap())
        .collect();

    let mut boards: Vec<Board> = Vec::new();

    while let Some(Ok(_)) = lines.next() {
        let line1 = lines.next().unwrap().unwrap();
        let line2 = lines.next().unwrap().unwrap();
        let line3 = lines.next().unwrap().unwrap();
        let line4 = lines.next().unwrap().unwrap();
        let line5 = lines.next().unwrap().unwrap();
        let mut squares: Vec<u8> = line1
            .split_ascii_whitespace()
            .take(5)
            .map(|x| u8::from_str_radix(x.trim(), 10).unwrap())
            .collect();
        squares.extend(
            line2
                .split_ascii_whitespace()
                .take(5)
                .map(|x| u8::from_str_radix(x, 10).unwrap()),
        );
        squares.extend(
            line3
                .split_ascii_whitespace()
                .take(5)
                .map(|x| u8::from_str_radix(x, 10).unwrap()),
        );
        squares.extend(
            line4
                .split_ascii_whitespace()
                .take(5)
                .map(|x| u8::from_str_radix(x, 10).unwrap()),
        );
        squares.extend(
            line5
                .split_ascii_whitespace()
                .take(5)
                .map(|x| u8::from_str_radix(x, 10).unwrap()),
        );

        boards.push(Board::new(squares));
    }

    Game { moves, boards }
}

fn main() {
    let mut game = read_data();
    let result1 = game.play_to_win();
    let result2 = game.play_to_lose();
    println!("Result1: {}", result1);
    println!("Result2: {}", result2);
}
