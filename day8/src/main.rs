use std::collections::HashMap;
use std::fs;

fn sort_string(word: &str) -> String {
    let mut letters: Vec<char> = word.chars().collect();
    letters.sort();
    letters.iter().collect()
}

#[derive(Debug)]
struct LineAnalyzer {
    left: Vec<String>,
    right: Vec<String>,
}

impl LineAnalyzer {
    fn new(line: &str) -> Self {
        let (left, right) = line.split_once(" | ").unwrap();
        LineAnalyzer {
            left: left.split_whitespace().map(|x| x.to_string()).collect(),
            right: right.split_whitespace().map(|x| x.to_string()).collect(),
        }
    }
    fn solve(&self) -> usize {
        let one = self
            .left
            .iter()
            .find(|&x| x.len() == 2)
            .expect("failed to decode 1");

        let four = self
            .left
            .iter()
            .find(|&x| x.len() == 4)
            .expect("failed to decode 4");
        let seven = self
            .left
            .iter()
            .find(|&x| x.len() == 3)
            .expect("failed to decode 7");
        let eight = self
            .left
            .iter()
            .find(|&x| x.len() == 7)
            .expect("failed to decode 8");
        let six = self
            .left
            .iter()
            .find(|&x| x.len() == 6 && one.chars().filter(|c| x.contains(*c)).count() == 1)
            .expect("failed to decode 6");
        let nine = self
            .left
            .iter()
            .find(|&x| x.len() == 6 && four.chars().all(|c| x.contains(c)) && x != six)
            .expect("failed to decode 9");
        let zero = self
            .left
            .iter()
            .find(|&x| x.len() == 6 && x != six && x != nine)
            .expect("failed to decode 0");
        let five = self
            .left
            .iter()
            .find(|&x| x.len() == 5 && six.chars().filter(|c| !x.contains(*c)).count() == 1)
            .expect("failed to decode 5");
        let three = self
            .left
            .iter()
            .find(|&x| {
                x.len() == 5 && nine.chars().filter(|c| !x.contains(*c)).count() == 1 && x != five
            })
            .expect("failed to decode 3");
        let two = self
            .left
            .iter()
            .find(|&x| x.len() == 5 && x != three && x != five)
            .expect("failed to decode 0");

        // Create a dictionary to easily find the proper value
        let mut dict: HashMap<String, usize> = HashMap::new();
        dict.insert(sort_string(zero), 0);
        dict.insert(sort_string(one), 1);
        dict.insert(sort_string(two), 2);
        dict.insert(sort_string(three), 3);
        dict.insert(sort_string(four), 4);
        dict.insert(sort_string(five), 5);
        dict.insert(sort_string(six), 6);
        dict.insert(sort_string(seven), 7);
        dict.insert(sort_string(eight), 8);
        dict.insert(sort_string(nine), 9);

        println!("\n{:?}:", self);
        println!("one found: {}", one);
        println!("four found: {}", four);
        println!("seven found: {}", seven);
        println!("eight found: {}", eight);
        println!("six found: {}", six);
        println!("nine found: {}", nine);
        println!("zero found: {}", zero);
        println!("five found: {}", five);
        println!("three found: {}", three);
        println!("two found: {}", two);

        let mut value = 0;
        for digit in &self.right {
            let s = sort_string(digit);
            println!("{}", digit);
            let v = dict[s.as_str()];
            value = value * 10 + v;
        }
        value
    }
}

fn read_input() -> Vec<LineAnalyzer> {
    let data = fs::read_to_string("input.txt").expect("failed to read file");
    //     let data =
    //         "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
    // edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
    // fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
    // fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
    // aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
    // fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
    // dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
    // bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
    // egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
    // gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";
    data.split('\n').map(LineAnalyzer::new).collect()
}
fn main() {
    let data = read_input();
    let sum: usize = data.iter().map(|x| x.solve()).sum();
    println!("Sum: {}", sum);
}
