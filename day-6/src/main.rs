use std::collections::HashSet;

fn main() {
    let line: Vec<char> = include_str!("input.txt").trim().chars().collect();
    let mut cnt: usize = 0;
    let mut st: HashSet<char> = HashSet::new();
    for (idx, i) in line.clone().into_iter().enumerate() {
        if !st.contains(&i) && cnt < 4 {
            st.insert(i);
            cnt += 1;
        } else if st.contains(&i) {
            while let ch = line[idx - cnt] {
                match ch {
                    ch if ch != i => {
                        st.remove(&ch);
                        cnt -= 1;
                    },
                    ch if ch == i => {
                        break;
                    }
                    _ => continue,
                }
            }
        }
        if cnt == 4 {
            println!("{}", idx + 1);
            break;
        }
    }
    st.clear();
    cnt = 0;
    for (idx, i) in line.clone().into_iter().enumerate() {
        if !st.contains(&i) && cnt < 14 {
            st.insert(i);
            cnt += 1;
        } else if st.contains(&i) {
            while let ch = line[idx - cnt] {
                match ch {
                    ch if ch != i => {
                        st.remove(&ch);
                        cnt -= 1;
                    },
                    ch if ch == i => {
                        break;
                    }
                    _ => continue,
                }
            }
        }
        if cnt == 14 {
            println!("{}", idx + 1);
            break;
        }
    }
}
