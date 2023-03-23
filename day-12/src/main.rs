use std::{fs, vec, u32::MAX};

const DIRS: [[i32; 2]; 4] = [[1, 0], [-1, 0], [0, 1], [0, -1]];

fn search(map: &Vec<Vec<char>>, vis: &[&Vec<bool>], mut cur: u32, mut min: &u32, pos_x: usize, pos_y: usize) {
    let width = map[0].len();
    let height = map.len();
    if map[pos_y][pos_x] == 'E' {
        min = min.min(&cur.clone());
        return;
    }
    for i in 0..4 {
        let new_x: usize = (pos_x as i32 + DIRS[i][0]) as usize;
        let new_y: usize = (pos_y as i32 + DIRS[i][1]) as usize;
        if new_x >= 0 && new_x < width && new_y >= 0 && new_y < height && (map[new_y][new_x] as i32 - map[pos_y][pos_x] as i32) <= 1 {
            vis[new_y][new_x] = true;
            cur += 1;
            search(map, vis, cur, min, new_x, new_y);
            cur -= 1;
            vis[new_y][new_x] = false;
        }
    }
}

fn main() {
    let mut input = fs::read_to_string("./src/input.txt").expect("Failed to read file!");
    input.pop();
    let lines: Vec<Vec<char>> = Vec::from_iter(input.split("\n").map(String::from)).iter().map(|s| s.chars().collect()).collect::<Vec<Vec<char>>>();
    let min = MAX;

    println!("{:#?}", lines);
}
