use std::fs;
use std::collections::HashSet;
use itertools::Itertools;

fn triple_common_chars(s1: &str, s2: &str, s3: &str) -> char {
    let a: HashSet<char> = s1.chars().into_iter().collect();
    let b: HashSet<char> = s2.chars().into_iter().collect();
    for c in s3.chars() {
        if a.contains(&c) && b.contains(&c) {
            return c;
        }
    }
    'a'
}


fn common_chars(s1: &str, s2: &str) -> char {
    const ALPHABET_LEN: usize = 52;
    const CHAR_CODE: usize = 97;
    let mut alphabet = [0; ALPHABET_LEN];
    for c in s1.chars() {
        if c.is_lowercase() {
            alphabet[c as usize - CHAR_CODE] += 1;
        } else {
            alphabet[c as usize - 39] += 1;
        }
    }
    for c in s2.chars() {
        let pos: usize;
        if c.is_lowercase() {
            pos = c as usize - CHAR_CODE;
        } else {
            pos = c as usize - 39;
        }
        if alphabet[pos] != 0 {
            return c;
        }
    }
    'a'
}

fn main() {
    let mut input = fs::read_to_string("day-3/src/input.txt").expect("Failed to read file!");
    input.pop();
    let lines: Vec<String> = Vec::from_iter(input.split("\n").map(String::from));

    let sum = lines.iter().map(|s| common_chars(&s[..(s.len() / 2)], &s[(s.len() / 2)..]))
                   .map(|c| match c {
                       c if c.is_lowercase() => c as usize - 'a' as usize + 1,
                       c if c.is_uppercase() => c as usize - 'A' as usize + 27,
                       _ => 0,
                   }).fold(0, |acc, x| acc + x);

    let pri = lines.iter().chunks(3).into_iter().map(|mut chunk| triple_common_chars(&chunk.nth(0).unwrap(), &chunk.nth(0).unwrap(), &chunk.nth(0).unwrap()))
                   .map(|c| match c {
                       c if c.is_lowercase() => c as usize - 'a' as usize + 1,
                       c if c.is_uppercase() => c as usize - 'A' as usize + 27,
                       _ => 0,
                   }).fold(0, |acc, x| acc + x);

    println!("{sum}");
    println!("{pri}");
}
