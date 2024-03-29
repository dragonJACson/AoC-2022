use std::fs;

fn main() {
    let mut input = fs::read_to_string("day-2/src/input.txt").expect("Failed to read file!");
    input.pop();
    let lines: Vec<String> = Vec::from_iter(input.split("\n").map(String::from));
    let score: u32 = lines.iter().fold(0 as u32, |acc, x|
        acc + match x.trim() {
            "A X" => 4,
            "A Y" => 8,
            "A Z" => 3,
            "B X" => 1,
            "B Y" => 5,
            "B Z" => 9,
            "C X" => 7,
            "C Y" => 2,
            "C Z" => 6,
            _ => 0,
        }
    );
    let second_score: u32 = lines.iter().fold(0 as u32, |acc, x|
    acc + match x.trim() {
            "A X" => 3,
            "A Y" => 4,
            "A Z" => 8,
            "B X" => 1,
            "B Y" => 5,
            "B Z" => 9,
            "C X" => 2,
            "C Y" => 6,
            "C Z" => 7,
            _ => 0,
        }
    );
    println!("{score}");
    println!("{second_score}");
}
