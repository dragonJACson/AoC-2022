use std::borrow::Borrow;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;
use std::rc::Rc;

#[derive(Clone, Debug)]
struct File {
    name: String,
    size: usize,
}

#[derive(Clone, Default, Debug)]
struct Dir {
    path: String,
    subdirs: Vec<Rc<RefCell<Dir>>>,
    files: Vec<File>,
    size: usize,
}

impl Dir {
    fn get_size(&self) -> usize {
        let mut cur_size = 0;
        for file in &self.files {
            cur_size += file.size;
        }
        for subdir in &self.subdirs {
            cur_size += subdir.borrow_mut().get_size();
        }
        cur_size
    }
}

fn main() {
    let mut input = fs::read_to_string("day-7/src/input.txt").expect("Failed to read file!");
    input.pop();
    let lines: Vec<String> = Vec::from_iter(input.split("\n").map(String::from));
    let mut dirs: HashMap<String, Rc<RefCell<Dir>>> = HashMap::new();
    let mut cur_path = String::new();
    for line in lines {
        if line.starts_with("$ cd") {
            if line.clone().replace("$ cd ", "").eq("..") {
                cur_path = cur_path.rsplit_once("/").unwrap().0.to_string();
            } else {
                if cur_path == "" {
                    cur_path += &line.clone().replace("$ cd ", "");
                } else {
                    cur_path += &line.clone().replace("$ cd ", "/");
                }
                if !dirs.contains_key(&cur_path) {
                    let tmp = Rc::new(RefCell::new(Dir {
                        path: cur_path.clone(),
                        subdirs: Vec::new(),
                        files: Vec::new(),
                        size: 0,
                    }));
                    dirs.insert(cur_path.clone(), Rc::clone(&tmp));
                }
            }
        } else if line.starts_with("$ ls") {
            continue;
        } else {
            if line.starts_with("dir ") {
                let tmp = Rc::new(RefCell::new(Dir {
                    path: format!("{cur_path}/{}", &line.clone().split(" ").nth(1).unwrap()),
                    subdirs: Vec::new(),
                    files: Vec::new(),
                    size: 0,
                }));
                dirs.get_mut(&cur_path).unwrap().borrow_mut().subdirs.push(Rc::clone(&tmp));
                dirs.insert(format!("{cur_path}/{}", &line.clone().split(" ").nth(1).unwrap()), Rc::clone(&tmp));
            } else {
                let file_info: Vec<&str> = line.split(" ").collect();
                dirs.get_mut(&cur_path).unwrap().borrow_mut().files.push(File {
                    name: file_info[1].to_string(),
                    size: file_info[0].parse::<usize>().unwrap(),
                });
            }
        }
    }
    let mut sum: usize = 0;
    let needed = 30000000 - (70000000 - dirs.get("/").unwrap().borrow_mut().get_size());
    let mut choice = dirs.get("/").unwrap().borrow_mut().get_size();
    for (_path, dir) in dirs.iter() {
        let cur_size = dir.borrow_mut().get_size();
        if cur_size >= needed && cur_size < choice {
            choice = cur_size;
        }
        if cur_size <= 100000 {
            sum += cur_size;
        }
    }
    println!("{sum}");
    println!("{choice}");
}
