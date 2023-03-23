use std::cell::RefCell;
use std::fs;
use std::collections::HashMap;

#[derive(Debug, Clone)]
enum Method {
    Add(u64),
    Mul(u64),
    MulSelf,
}

#[derive(Debug, Clone)]
pub struct Monkey {
    id: u64,
    items: Vec<u64>,
    how: Method,
    div: u64,
    true_to: u64,
    false_to: u64,
    count: u64,
}

peg::parser!{
    grammar assign_parser() for str {
        rule number() -> u64
            = n:$(['0'..='9']+) {? n.parse().or(Err("u64")) }

        rule method() -> Method = precedence! {
            "+ " y:number() { Method::Add(y) }
            "* " y:number() { Method::Mul(y) }
            "* old" { Method::MulSelf }
        }

        pub rule count_monkeys() -> Monkey
            = "Monkey " id:number() ":\n"
              "  Starting items: " items:(number() ** ", ") "\n"
              "  Operation: new = old " how:method() "\n"
              "  Test: divisible by " div:number() "\n"
              "    If true: throw to monkey " true_to:number() "\n"
              "    If false: throw to monkey " false_to:number()
              { Monkey { id, items, how, div, true_to, false_to, count: 0 } }
    }
}

fn main() {
    let mut input = fs::read_to_string("day-11/src/input.txt").expect("Failed to read file!");
    input.pop();
    let lines: Vec<String> = Vec::from_iter(input.split("\n\n").map(String::from));
    let mut monkeys: HashMap<u64, RefCell<Monkey>> = HashMap::new();
    for line in &lines {
        match assign_parser::count_monkeys(&line) {
            Ok(mon) => monkeys.insert(mon.id, RefCell::new(mon)),
            Err(err) => panic!("Parse error: {:?}", err),
        };
    }
    let div_prod = monkeys.values().map(|m| m.borrow().div).product::<u64>();
    let monkeys_clone = monkeys.clone();
    for _ in 0..20 {
        for i in 0..monkeys_clone.len() {
            let mut monkey = monkeys_clone.get(&(i as u64)).unwrap().borrow_mut();
            while let Some(mut item) = monkey.items.pop() {
                monkey.count += 1;
                match monkey.how {
                    Method::Add(a) => item = (item + a) / 3,
                    Method::Mul(a) => item = (item * a) / 3,
                    Method::MulSelf => item = (item * item) / 3,
                }
                if item % monkey.div == 0 {
                    monkeys_clone.get(&monkey.true_to).unwrap().borrow_mut().items.push(item);
                } else {
                    monkeys_clone.get(&monkey.false_to).unwrap().borrow_mut().items.push(item);
                }
            }
        }
    }
    let mut counts: Vec<u64> = monkeys_clone.values().map(|m| m.borrow().count).collect();
    counts.sort_by(|a, b| b.cmp(a));
    println!("{}", counts[0] * counts[1]);
    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            let mut monkey = monkeys.get(&(i as u64)).unwrap().borrow_mut();
            while let Some(mut item) = monkey.items.pop() {
                monkey.count += 1;
                item %= div_prod;
                match monkey.how {
                    Method::Add(a) => item = item + a,
                    Method::Mul(a) => item = item * a,
                    Method::MulSelf => item = item * item,
                }
                if item % monkey.div == 0 {
                    monkeys.get(&monkey.true_to).unwrap().borrow_mut().items.push(item);
                } else {
                    monkeys.get(&monkey.false_to).unwrap().borrow_mut().items.push(item);
                }
            }
        }
    }
    let mut counts: Vec<u64> = monkeys.values().map(|m| m.borrow().count).collect();
    counts.sort_by(|a, b| b.cmp(a));
    println!("{}", counts[0] * counts[1]);
}
