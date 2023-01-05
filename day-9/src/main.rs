use std::fs;
use std::collections::HashSet;

fn main() {
    let mut input = fs::read_to_string("./src/input.txt").expect("Failed to read file!");
    input.pop();
    let lines: Vec<(String, i32)> = Vec::from_iter(input.split("\n").map(String::from)).into_iter().map(
        |x| (x.clone().split_once(" ").unwrap().0.to_owned(), x.clone().split_once(" ").unwrap().1.parse().unwrap())
    ).collect();
    let mut tail_positions: HashSet<(i32, i32)> = HashSet::new();
    let mut nine_positions: HashSet<(i32, i32)> = HashSet::new();
    let mut tails = vec![(0, 0); 10];
    tail_positions.insert(tails[1]);
    nine_positions.insert(tails[9]);
    for line in lines {
        match line {
            (dir, steps) => {
                for _i in 0..steps {
                    match dir.as_str() {
                        "U" => tails[0].1 += 1,
                        "D" => tails[0].1 -= 1,
                        "R" => tails[0].0 += 1,
                        "L" => tails[0].0 -= 1,
                        _ => continue,
                    }
                    for j in 1..=9 {
                        match (tails[j - 1].0 - tails[j].0, tails[j - 1].1 - tails[j].1) {
                            (2, 0) => tails[j].0 += 1,
                            (0, 2) => tails[j].1 += 1,
                            (-2, 0) => tails[j].0 -= 1,
                            (0, -2) => tails[j].1 -= 1,
                            (2, 1) | (1, 2) | (2, 2) => {
                                tails[j].0 += 1;
                                tails[j].1 += 1;
                            },
                            (-2, 1) | (-1, 2) | (-2, 2) => {
                                tails[j].0 -= 1;
                                tails[j].1 += 1;
                            },
                            (1, -2) | (2, -1) | (2, -2) => {
                                tails[j].0 += 1;
                                tails[j].1 -= 1;
                            },
                            (-2, -1) | (-1, -2) | (-2, -2) => {
                                tails[j].0 -= 1;
                                tails[j].1 -= 1;
                            },
                            (_, _) => continue,
                        }
                    }
                    tail_positions.insert(tails[1]);
                    nine_positions.insert(tails[9]);
                }
            }
        }
    }
    println!("Result of part one is {}", tail_positions.len());
    println!("Result of part two is {}", nine_positions.len());
}
