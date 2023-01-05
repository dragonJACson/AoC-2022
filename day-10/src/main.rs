use std::{fs, vec};
use std::collections::HashMap;

fn main() {
    let mut input = fs::read_to_string("./src/input.txt").expect("Failed to read file!");
    input.pop();
    let lines: Vec<String> = Vec::from_iter(input.split("\n").map(String::from));
    let mut cnt = 0;
    let mut register = 1;
    let mut signals: HashMap<i32, i32> = HashMap::new();
    let mut screen = vec![String::from("........................................"); 6];
    for line in lines {
        if line.starts_with("noop") {
            cnt += 1;
            signals.insert(cnt, register);
        } else if line.starts_with("addx") {
            cnt += 1;
            signals.insert(cnt, register);
            cnt += 1;
            signals.insert(cnt, register);
            register += line.split_once(" ").unwrap().1.parse::<i32>().unwrap();
        }
    }
    let answer_1 = signals.get(&20).unwrap() * 20 + signals.get(&60).unwrap() * 60
        + signals.get(&100).unwrap() * 100 + signals.get(&140).unwrap() * 140
        + signals.get(&180).unwrap() * 180 + signals.get(&220).unwrap() * 220;
    println!("Result of part one is {answer_1}");
    for i in 0..240 {
        if (i % 40) >= signals.get(&(i + 1)).unwrap() - 1 && (i % 40) <= signals.get(&(i + 1)).unwrap() + 1 {
            screen[(i / 40) as usize].replace_range(((i % 40) as usize)..((i % 40 + 1) as usize), "#");
        }
    }
    println!("{:#?}", screen);
}
