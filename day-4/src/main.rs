use std::fs;

peg::parser!{
    grammar assign_parser() for str {
        rule number() -> u32
            = n:$(['0'..='9']+) {? n.parse().or(Err("u32")) }

        pub rule range() -> [(u32, u32); 2]
            = first:number() "-" second:number() "," third:number() "-" fourth:number()
              { [(first, second), (third, fourth)] }
    }
}

fn main() {
    let mut input = fs::read_to_string("day-4/src/input.txt").expect("Failed to read file!");
    input.pop();
    let lines: Vec<String> = Vec::from_iter(input.split("\n").map(String::from));
    let mut cnt_part1 = 0;
    let mut cnt_part2 = 0;
    for line in lines {
        match assign_parser::range(&line).unwrap() {
            [(a, b), (c, d)] => {
                if (a >= c && b <= d) || (c >= a && d <= b) {
                    cnt_part1 += 1;
                }
                if (a >= c && a <= d) || (b >= c && b <= d) || (c >= a && c <= b) || (d >= a && d <= b) {
                    cnt_part2 += 1;
                }
            }
        }
    }
    println!("{cnt_part1}");
    println!("{cnt_part2}");
}
