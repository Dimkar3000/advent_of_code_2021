#[derive(Debug)]
enum Packet {
    Literal {
        version: u32,
        _type_id: u32,
        value: u64,
    },
    Operation {
        version: u32,
        id: u32,
        sub_packages: Vec<Packet>,
    },
}

impl Packet {
    fn add_versions(&self) -> u64 {
        match self {
            Packet::Literal {
                version: l,
                _type_id: _,
                value: _,
            } => *l as u64,
            Packet::Operation {
                version: l,
                id: _,
                sub_packages: pkg,
            } => (*l as u64) + pkg.iter().map(|x| x.add_versions()).sum::<u64>(),
        }
    }

    fn calculate(&self) -> u64 {
        match self {
            Packet::Literal {
                version: _,
                _type_id: _,
                value: v,
            } => *v as u64,
            Packet::Operation {
                version: _,
                id: 0,
                sub_packages: pkg,
            } => pkg.iter().map(|x| x.calculate()).sum::<u64>(),
            Packet::Operation {
                version: _,
                id: 1,
                sub_packages: pkg,
            } => pkg.iter().map(|x| x.calculate()).product::<u64>(),
            Packet::Operation {
                version: _,
                id: 2,
                sub_packages: pkg,
            } => pkg.iter().map(|x| x.calculate()).min().unwrap(),
            Packet::Operation {
                version: _,
                id: 3,
                sub_packages: pkg,
            } => pkg.iter().map(|x| x.calculate()).max().unwrap(),
            Packet::Operation {
                version: _,
                id: 5,
                sub_packages: pkg,
            } => {
                if pkg[0].calculate() > pkg[1].calculate() {
                    1
                } else {
                    0
                }
            }
            Packet::Operation {
                version: _,
                id: 6,
                sub_packages: pkg,
            } => {
                if pkg[0].calculate() < pkg[1].calculate() {
                    1
                } else {
                    0
                }
            }
            Packet::Operation {
                version: _,
                id: 7,
                sub_packages: pkg,
            } => {
                if pkg[0].calculate() == pkg[1].calculate() {
                    1
                } else {
                    0
                }
            }
            _ => unreachable!(),
        }
    }

    // Consumes the chars it need then it returns the new offset
    fn single_packet(words: &[char], base: usize) -> (Self, usize) {
        let version_str: String = words.iter().skip(base).take(3).collect();
        let version_type_str: String = words.iter().skip(base + 3).take(3).collect();

        let packet_version = u32::from_str_radix(&version_str, 2).unwrap();
        let packet_type_id = u32::from_str_radix(&version_type_str, 2).unwrap();

        if packet_type_id == 4 {
            let mut buffer: Vec<char> = vec![];
            let mut it = words.iter().skip(base + 6);
            let mut consumed = 0;
            while let Some(t) = it.next() {
                buffer.push(*it.next().unwrap());
                buffer.push(*it.next().unwrap());
                buffer.push(*it.next().unwrap());
                buffer.push(*it.next().unwrap());
                consumed += 5;
                if t == &'0' {
                    break;
                }
            }
            let result = u64::from_str_radix(&buffer.iter().collect::<String>(), 2).unwrap();

            (
                Packet::Literal {
                    _type_id: packet_type_id,
                    version: packet_version,
                    value: result,
                },
                base + 6 + consumed,
            )
        } else {
            let i = words.iter().skip(base + 6).next().unwrap();
            // println!("I: {}", i);
            if i == &'0' {
                // If the length type ID is 0, then the next 15 bits are a number that represents the total length in bits of the sub-packets contained by this packet.
                let length = usize::from_str_radix(
                    &words.iter().skip(base + 7).take(15).collect::<String>(),
                    2,
                )
                .unwrap();

                let new_word: Vec<char> = words
                    .iter()
                    .skip(base + 22)
                    .take(length as usize)
                    .copied()
                    .collect();
                let mut new_base = 0;
                let mut sub_packages = vec![];
                while new_base < new_word.len() {
                    let (p, b) = Packet::single_packet(&new_word, new_base);
                    sub_packages.push(p);
                    new_base = b;
                }
                (
                    Packet::Operation {
                        id: packet_type_id,
                        version: packet_version,
                        sub_packages,
                    },
                    base + 7 + 15 + length,
                )
            } else {
                // If the length type ID is 1, then the next 11 bits are a number that represents the number of sub-packets immediately contained by this packet.
                let length = usize::from_str_radix(
                    &words.iter().skip(base + 7).take(11).collect::<String>(),
                    2,
                )
                .unwrap();
                // println!("Count of Sub-Packets: {}", length);
                let mut sub_packages: Vec<Packet> = Vec::with_capacity(length);
                let mut new_base = base + 7 + 11;
                for _ in 0..length {
                    let (p, b) = Packet::single_packet(words, new_base);
                    // println!("{:?}", p);
                    // println!("new_base: {}", new_base);
                    sub_packages.push(p);
                    new_base = b;
                }
                (
                    Packet::Operation {
                        id: packet_type_id,
                        version: packet_version,
                        sub_packages,
                    },
                    new_base,
                )
            }
        }
    }

    fn from_str(words: &[char]) -> Self {
        Packet::single_packet(words, 0).0
    }
}

use std::fs;
use std::str::Chars;

fn hex_to_char(c: char) -> Chars<'static> {
    match c {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => unreachable!(),
    }
    .chars()
}
fn read_data(filename: &str) -> Vec<char> {
    let file = fs::read_to_string(filename).expect("failed to read file");
    file.chars().flat_map(hex_to_char).collect()
}

fn main() {
    let d = read_data("input.txt");
    println!("size: {:?}", d.len());

    let p = Packet::from_str(&d);
    println!("p: {:?}", p);

    let sum = p.add_versions();
    println!("sum: {}", sum);

    let calc = p.calculate();
    println!("calc: {}", calc);
}
