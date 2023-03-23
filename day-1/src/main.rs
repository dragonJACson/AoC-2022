use std::fs;

fn main() {
    let mut input = fs::read_to_string("day-1/src/input.txt").expect("Failed to read file!");
    input.pop();
    let lines: Vec<String> = Vec::from_iter(input.split("\n\n").map(String::from));
    let mut carolies: Vec<u32> = lines.iter().map(|str| str.split('\n').fold(0 as u32, |acc, x| acc + x.parse::<u32>().unwrap())).collect();
    carolies.sort_by(|a, b| b.cmp(a));
    println!("{}", carolies[0]);
    println!("{}", carolies[0] + carolies[1] + carolies[2]);
}
