use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

static INPUT_FILE: &'static str = "input.txt";

fn read_file(file_name:&str) -> Vec<usize> {
    let f = File::open(file_name).expect("failed to open file");
    let reader = BufReader::new(f);

    // Info: Assumes 12 bits per line.
    let binary_string_to_number = |x:String| {
        let mut result = 0;
        for i in 0..12 {
            result <<=1;
            if x.chars().nth(i).unwrap() == '1' {
                result |= 1;
            }
        }

        result
    };
    reader.lines().filter(|x| x.is_ok()).map(|x| x.unwrap()).map(binary_string_to_number).collect()
    
}
type Handler = dyn Fn(usize,usize) -> bool;

fn generate_value(numbers: &[usize], more_ones: &Handler,less_ones:&Handler) -> usize {
    let mut clone = numbers.to_vec();
    let mut position = 0;
    while clone.len() > 1 {
        let ones_count = clone.iter().filter(|&&x|{x& (1 <<(11-position)) > 0}).count();
        if 2*ones_count >= clone.len()   {
            clone = clone.iter().filter(|&&x|more_ones(x,position)).map(|&x|x).collect();
        } 
        else {
            clone = clone.iter().filter(|&&x|less_ones(x,position)).map(|&x|x).collect();

        }
        position += 1;
    }

    clone[0]

}

fn main() {

    let numbers = read_file(INPUT_FILE);
    
    let one_at_pos = |x:usize,position:usize| {x& (1 <<(11-position)) > 0};
    let zero_at_pos = |x:usize,position:usize| {x& (1 <<(11-position)) == 0};
    let oxygen = generate_value(&numbers,&one_at_pos, &zero_at_pos);
    let co2 = generate_value(&numbers,&zero_at_pos,&one_at_pos);

    
    println!("Oxygen Generator Rating: {}", oxygen);
    println!("CO2 Scrubber Rating: {}", co2);
    println!("Result: {}", oxygen * co2);

    

    // println!("First number: {}", clone1[0]);
    // println!("Second number: {}", clone2[0]);
    // println!("Result: {}", clone1[0] * clone2[0]);
}
