
use std::fs::File;
use std::io::{prelude::*, BufReader};

//const FILENAME: &str = "./input";
const FILENAME: &str = "./test";

// Packet data is either an integer or a tuple
#[derive(Debug)]
enum PacketData {
    Integer( i32 ),
    List( Vec<PacketData> ),
}

fn packet_from_list(line: &str) -> PacketData {
    let mut rval: Vec<PacketData> = Vec::new();

    // verify line starts and ends with '[' and']'
    let mut chars = line.trim().chars();
    match chars.next().unwrap() {
        '[' => (),
        _ => panic!("List did not start with '[': {}", line),
    }

    // depth aware comma separation
    let mut depth = 0;
    let mut list_strs = Vec::new();
    let mut cur_str = String::new();

    loop {
        // pop next character
        let c = chars.next().unwrap();
        cur_str.push(c);
        match c {
            '[' => depth += 1,
            ']' => {
                if depth == 0 {
                    cur_str.pop();
                    list_strs.push(cur_str);
                    break;
                } else {
                    depth -= 1;
                }
            },
            ',' => {
                if depth == 0 {
                    cur_str.pop();
                    list_strs.push(cur_str);
                    cur_str = String::new();
                }
            },
            _ => (),
        }
    }

    for s in list_strs {
        if s.starts_with('[') {
            rval.push(
                packet_from_list(&s));
        } else if let Ok(i) = s.parse() {
            rval.push(PacketData::Integer(i));

        } else if !s.is_empty() {
            panic!("Was not integer: {}", s);
        }
    }

    PacketData::List(rval)
}

// read in the file and store in a 2-D vector
fn read_packets() {
    // Open the file
    let file = File::open(FILENAME).unwrap();
    let mut reader = BufReader::new(file);
    // Read file line by line
    loop {
        let mut line = Default::default();
        let line_size = reader.read_line(&mut line).unwrap();
        match line_size  {
            0 => break, // EOF
            1 => continue,
            _ => (),
        };

        let packet = packet_from_list(&line);

        println!("{:?}", packet);
    }
}

fn part_1() {
    read_packets();
}

fn part_2() {
}

fn main() {
    println!("Advent of Code, Day 13");

    part_1();
    part_2();
}