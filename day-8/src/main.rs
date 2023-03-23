use std::fs;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Status {
    Visible,
    NorthVisible,
    SouthVisible,
    WestVisible,
    EastVisible,
    Invisible,
}

fn main() {
    let mut input = fs::read_to_string("day-8/src/input.txt").expect("Failed to read file!");
    input.pop();
    let lines: Vec<Vec<u32>> = Vec::from_iter(input.split("\n").map(String::from)).into_iter().map(
        |x| x.chars().map(|ch| ch.to_digit(10).unwrap()).collect()
    ).collect();
    let row_len = lines.len();
    let col_len = lines[0].len();
    let mut status: Vec<Vec<(Status, usize)>> = vec![vec![(Status::Invisible, 1); col_len]; row_len];
    for (row, line) in lines.iter().enumerate() {
        for (col, tree) in line.iter().enumerate() {
            if row == 0 || col == 0 || row == row_len - 1 || col == col_len - 1 {
                status[row][col].0 = Status::Visible;
            }
            let mut flag = true;
            let mut cnt = 0;
            for i in (0..row).rev() {
                cnt += 1;
                if lines[i][col] < *tree {
                    continue;
                } else {
                    flag = false;
                    break;
                }
            }
            if flag == true {
                status[row][col].0 = Status::NorthVisible;
            }
            status[row][col].1 *= match cnt {
                cnt if cnt == 0 => 1,
                cnt => cnt,
            };
            flag = true;
            cnt = 0;
            for i in row+1..row_len {
                cnt += 1;
                if lines[i][col] < *tree {
                    continue;
                } else {
                    flag = false;
                    break;
                }
            }
            if flag == true {
                status[row][col].0 = Status::SouthVisible;
            }
            status[row][col].1 *= match cnt {
                cnt if cnt == 0 => 1,
                cnt => cnt,
            };
            flag = true;
            cnt = 0;
            for j in (0..col).rev() {
                cnt += 1;
                if lines[row][j] < *tree {
                    continue;
                } else {
                    flag = false;
                    break;
                }
            }
            if flag == true {
                status[row][col].0 = Status::WestVisible;
            }
            status[row][col].1 *= match cnt {
                cnt if cnt == 0 => 1,
                cnt => cnt,
            };
            flag = true;
            cnt = 0;
            for j in col+1..col_len {
                cnt += 1;
                if lines[row][j] < *tree {
                    continue;
                } else {
                    flag = false;
                    break;
                }
            }
            if flag == true {
                status[row][col].0 = Status::EastVisible;
            }
            status[row][col].1 *= match cnt {
                cnt if cnt == 0 => 1,
                cnt => cnt,
            };
        }
    }
    let sum = status.iter().flatten().filter(|x| (**x).0 != Status::Invisible).count();
    let biggest = status.iter().flatten().map(|(_status, view)| view).max().unwrap();
    println!("{sum}");
    println!("{biggest}");
}
