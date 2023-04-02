use std::{fs, vec};
use std::collections::VecDeque;

const DIRS: [[i32; 2]; 4] = [[1, 0], [-1, 0], [0, 1], [0, -1]];

fn search(map: & Vec<Vec<char>>, vis: &mut Vec<Vec<bool>>, pos_x: usize, pos_y: usize, problem2: bool) -> u32 {
    let width = map[0].len();
    let height = map.len();
    let mut cur: u32 = 0;
    let mut positions = VecDeque::new();
    positions.push_back((pos_x, pos_y));
    while !positions.is_empty() {
        let mut sz = positions.len();
        while sz != 0 {
            let (x, y) = positions.pop_front().unwrap();
            for i in 0..4 {
                let new_x: usize = (x as i32 + DIRS[i][0]) as usize;
                let new_y: usize = (y as i32 + DIRS[i][1]) as usize;
                if problem2 == false && new_x < width && new_y < height
                                     && ((map[new_y][new_x] != 'E' && (map[new_y][new_x]) as i32 - (map[y][x]) as i32 <= 1)
                                        || (map[new_y][new_x] == 'E' && (map[y][x] == 'y' || map[y][x] == 'z'))
                                        || (map[y][x] == 'S' && (map[new_y][new_x] == 'a' || map[new_y][new_x] == 'b')))
                                     && vis[new_y][new_x] == false {
                    vis[new_y][new_x] = true;
                    if map[new_y][new_x] == 'E' {
                        return cur + 1;
                    }
                    positions.push_back((new_x, new_y));
                }
                if problem2 == true && new_x < width && new_y < height
                                    && ((map[y][x] != 'E' && ((map[new_y][new_x]) as i32 - (map[y][x]) as i32 >= -1))
                                        || (map[y][x] == 'E' && (map[new_y][new_x] == 'y' || map[new_y][new_x] == 'z')))
                                    && vis[new_y][new_x] == false {
                    vis[new_y][new_x] = true;
                    if map[new_y][new_x] == 'a' {
                        return cur + 1;
                    }
                    positions.push_back((new_x, new_y));
                }
            }
            sz -= 1;
        }
        cur += 1;
    }
    cur
}

fn main() {
    let mut input = fs::read_to_string("day-12/src/input.txt").expect("Failed to read file!");
    input.pop();
    let lines: Vec<Vec<char>> = Vec::from_iter(input.split("\n").map(String::from)).iter().map(|s| s.chars().collect()).collect::<Vec<Vec<char>>>();
    let mut vis: Vec<Vec<bool>> = vec![vec![false; lines[0].len()]; lines.len()];
    let mut vis2: Vec<Vec<bool>> = vec![vec![false; lines[0].len()]; lines.len()];
    let mut min = 0;
    let mut min2 = 0;
    for (i, line) in lines.iter().enumerate() {
        for (j, pos) in line.iter().enumerate() {
            if *pos == 'S' {
                vis[i][j] = true;
                min = search(&lines, &mut vis, j, i, false);
            }
            if *pos == 'E' {
                vis2[i][j] = true;
                min2 = search(&lines, &mut vis2, j, i, true);
            }
        }
    }
    println!("{min}");
    println!("{min2}");
}

// I forgot a lot about DFS and BFS, at first I tried to solve this problem with DFS, and I got a looooooooong run
