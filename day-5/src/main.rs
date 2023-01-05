use std::fs;

peg::parser!{
    grammar assign_parser() for str {
        rule number() -> u32
            = n:$(['0'..='9']+) {? n.parse().or(Err("u32")) }

        pub rule moves() -> (u32, u32, u32)
            = "move " amount:number() " from " src:number() " to " dest:number()
              { (amount, src, dest) }
    }
}

fn main() {
    let mut input = fs::read_to_string("./src/input.txt").expect("Failed to read file!");
    input.pop();
    let lines: Vec<String> = Vec::from_iter(input.split("\n\n").map(String::from));
    let mut stack: Vec<&str> = lines.iter().nth(0).unwrap().split("\n").collect();
    let moves: Vec<&str> = lines.iter().nth(1).unwrap().split("\n").collect();
    let mut stacks: Vec<Vec<char>> = Vec::new();
    stack.reverse();
    for (idx, i) in stack.iter().nth(0).unwrap().chars().enumerate() {
        if i.is_numeric() {
            let mut tmp: Vec<char> = Vec::new();
            for line in stack.iter().skip(1) {
                match line.chars().nth(idx) {
                    Some(a) if a.is_alphabetic() => tmp.push(a),
                    Some(_) => break,
                    None => break,
                }
            }
            stacks.push(tmp);
        }
    }
    let mut part_two_stacks = stacks.clone();
    for line in &moves {
        match assign_parser::moves(&line).unwrap() {
            (amount, src, dest) => {
                for _i in 0..amount {
                    let ch = stacks[(src - 1) as usize].pop().unwrap();
                    stacks[(dest - 1) as usize].push(ch);
                }
            },
        }
    }
    for line in &moves {
        match assign_parser::moves(&line).unwrap() {
            (amount, src, dest) => {
                if amount == 1 {
                    let ch = part_two_stacks[(src - 1) as usize].pop().unwrap();
                    part_two_stacks[(dest - 1) as usize].push(ch);
                } else {
                    let len = part_two_stacks[(src - 1) as usize].len();
                    let mut chs: Vec<char> = part_two_stacks[(src - 1) as usize].splice((len - amount as usize)..len, vec![]).collect();
                    part_two_stacks[(dest - 1) as usize].append(&mut chs);
                }
            },
        }
    }
    println!("Part one:");
    for stack in &stacks {
        print!("{}", stack[stack.len() - 1])
    }
    println!("\nPart two:");
    for stack in &part_two_stacks {
        print!("{}", stack[stack.len() - 1])
    }
}
