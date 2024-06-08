const FILENAME: &str = "./input";
//const FILENAME: &str = "./test";

use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::collections::HashMap;
use std::cmp::Ordering;

#[derive(Clone)]
enum MonkeyType{
    Number(i64),
    Add(String, String),
    Subtract(String, String),
    Multiply(String, String),
    Divide(String, String)
}

#[derive(Clone)]
struct Monkey {
    id: String,
    op: MonkeyType
}

impl Monkey {
    fn from_str(line: &str) -> Monkey {
        let (id, line) = line.split_once(": ").unwrap();
        let id = id.to_owned();

        let op: MonkeyType;
        if line.contains("+") {
            let (id_1, id_2) = line
                .split_once(" + ").unwrap();
            op = MonkeyType::Add(id_1.to_owned(), id_2.to_owned());
        } else if line.contains("*") {
            let (id_1, id_2) = line
                .split_once(" * ").unwrap();
            op = MonkeyType::Multiply(id_1.to_owned(), id_2.to_owned());
        } else if line.contains("-") {
            let (id_1, id_2) = line
                .split_once(" - ").unwrap();
            op = MonkeyType::Subtract(id_1.to_owned(), id_2.to_owned());
        } else if line.contains("/") {
            let (id_1, id_2) = line
                .split_once(" / ").unwrap();
            op = MonkeyType::Divide(id_1.to_owned(), id_2.to_owned());
        } else {
            let x = line.parse().unwrap();
            op = MonkeyType::Number(x);
        }

        Monkey{
            id,
            op
        }
    }

    fn to_string(&self) -> String {
        let mut rval = format!("{}: ", self.id);
        match &self.op {
            MonkeyType::Number(x) => rval.push_str(&x.to_string()),
            MonkeyType::Add(a, b) =>
                rval.push_str(&format!("{} + {}", a, b)),
            MonkeyType::Subtract(a, b) =>
                rval.push_str(&format!("{} - {}", a, b)),
            MonkeyType::Multiply(a, b) =>
                rval.push_str(&format!("{} * {}", a, b)),
            MonkeyType::Divide(a, b) =>
                rval.push_str(&format!("{} / {}", a, b)),
        }

        rval
    }
}

fn get_monkey_val(id: &String, map: &HashMap<String, Monkey>) -> i64 {
    let monkey = map.get(id).expect("Invalid monkey");
    let rval;

    match &monkey.op {
        MonkeyType::Number(x) => rval = x.to_owned(),
        MonkeyType::Add(a, b) =>
            rval = get_monkey_val(a, map) + get_monkey_val(b, map),
        MonkeyType::Subtract(a, b) =>
            rval = get_monkey_val(a, map) - get_monkey_val(b, map),
        MonkeyType::Multiply(a, b) =>
            rval = get_monkey_val(a, map) * get_monkey_val(b, map),
        MonkeyType::Divide(a, b) =>
            rval = get_monkey_val(a, map) / get_monkey_val(b, map),
    }

    rval
}

fn read_file(filename: &str) -> Vec<Monkey> {
    // Open the file
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    // Read file line by line
    let mut rval = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let line = line.trim();

        rval.push(Monkey::from_str(line));
    }
    rval}

fn part_1(monkeys: Vec<Monkey>) -> i64 {
    let mut map = HashMap::new();
    for m in monkeys {
        println!("{}", m.to_string());
        map.insert(m.id.clone(), m);
    }
    
    get_monkey_val(&"root".to_owned(), &map)
}

fn part_2(monkeys: Vec<Monkey>) -> i64 {
    let mut map = HashMap::new();
    for m in monkeys {
        println!("{}", m.to_string());
        map.insert(m.id.clone(), m);
    }
    let root = map.get("root").unwrap();
    let (id_1, id_2) = match &root.op {
        MonkeyType::Add(a, b) => (a.clone(), b.clone()),
        _=>panic!("root was wrong")
    };

    let human = map.get_mut("humn").unwrap();
    human.op = MonkeyType::Number(1);

    let val_2 = get_monkey_val(&id_2, &map);
    let step_size = 2_i64.pow(0);
    let val_1 = get_monkey_val(&id_1, &map);
    let val_2 = get_monkey_val(&id_2, &map);
    let init_cmp = val_1.cmp(&val_2);
    let mut prev_cmp;
    let mut i = 1;

    loop {
        let human = map.get_mut("humn").unwrap();
        human.op = MonkeyType::Number(2_i64.pow(i));

        let val_1 = get_monkey_val(&id_1, &map);
        let val_2 = get_monkey_val(&id_2, &map);
        prev_cmp = val_1.cmp(&val_2);

        if prev_cmp != init_cmp {
            break;
        }
        i += 1;
    }
    i -= 1;

    let mut h = 2_i64.pow(i);
    i -= 1;
    loop {
        let human = map.get_mut("humn").unwrap();
        human.op = MonkeyType::Number(h);

        let val_1 = get_monkey_val(&id_1, &map);
        let val_2 = get_monkey_val(&id_2, &map);
        let new_cmp = val_1.cmp(&val_2);

        println!("{} != {} ({} i: {})", val_1, val_2, h, i);
        if new_cmp == Ordering::Equal {
            loop {
                h -= 1;
                let human = map.get_mut("humn").unwrap();
                human.op = MonkeyType::Number(h);

                let val_1 = get_monkey_val(&id_1, &map);
                let val_2 = get_monkey_val(&id_2, &map);
                let new_cmp = val_1.cmp(&val_2);
                if new_cmp == Ordering::Equal {
                    println!("{} == {} ({} i: {})", val_1, val_2, h, i);
                } else {
                    break;
                }
            }
            let rval = h + 1;
            return rval;
        }
        if new_cmp == init_cmp {
            h = h + 2_i64.pow(i);
        } else {
            h = h - 2_i64.pow(i);
        }
        i -= 1;
    }
    panic!("SHouldn't get here");
}

fn main() {
    println!("Advent of Code, Day 19");

    // read in the input
    let numbers = read_file(FILENAME);

    let now = Instant::now();
    use std::time::Instant;
    let answer = part_1(numbers.clone());
    let elapsed = now.elapsed();
    println!("Part 1: {}", answer);
    println!("Took {:.5?}", elapsed);

    if FILENAME == "./test" {
        assert_eq!(answer, 152);
    } else {
        //assert_eq!(answer, 2203);
    }

    let now = Instant::now();
    let answer = part_2(numbers.clone());
    let elapsed = now.elapsed();
    println!("Part 2: {}", answer);
    println!("Took {:.5?}", elapsed);

    if FILENAME == "./test" {
        assert_eq!(answer, 301);
    } else {
        assert_eq!(answer, 3769668716709);
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn print_test_strat() {

    }
}