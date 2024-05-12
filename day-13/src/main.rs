
use std::cmp::Ordering;
use std::fs::File;
use std::io::{prelude::*, BufReader};

const FILENAME: &str = "./input";
//const FILENAME: &str = "./test";

// Packet data is either an integer or a tuple
#[derive(Clone)]
#[derive(Debug)]
enum PacketData {
    Integer( i32 ),
    List( Vec<PacketData> ),
}

fn compare_packet_lists(left: &Vec<PacketData>, right: &Vec<PacketData>) -> Ordering {
    let mut left_iter = left.iter();
    let mut right_iter = right.iter();
    let mut rval = Ordering::Equal;

    while rval == Ordering::Equal {
        if let Some(l) = left_iter.next() {
            if let Some(r) = right_iter.next() {
                rval = compare_packets(&l, &r);
            } else {
                return Ordering::Greater;
            }
        } else {
            if let Some(_) = right_iter.next() {
                return Ordering::Less;
            }

            break;
        }
    }

    return rval;
}
fn compare_packets(left: &PacketData, right: &PacketData) -> Ordering {
    match left {
        PacketData::Integer(l) => {
            match right {
                PacketData::Integer(r) => {
                    return l.cmp(r);
                },
                PacketData::List(r) => {
                    let l_list = vec!(left.clone());
                    return compare_packet_lists(&l_list, r);
                }
            }
        },
        PacketData::List(l) => {
            match right {
                PacketData::List(r) => {
                    return compare_packet_lists(l, r);
                },
                PacketData::Integer(_) => {
                    let r_list = vec!(right.clone());
                    return compare_packet_lists(l, &r_list);
                }
            }
        },
    }

    return Ordering::Equal;
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
fn read_packets() -> Vec<PacketData> {
    let mut rval: Vec<PacketData> = Vec::new();
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

        rval.push(packet);
    }
    rval
}

fn part_1() {
    let packets = read_packets();

    let mut num_ordered = 0;
    let mut ordered_indices = Vec::new();
    let mut i = 1;
    for packet_pair in packets.chunks_exact(2) {
        let (left, right) = (
            &packet_pair[0], &packet_pair[1]);

        // TODO: compare left and right
        let c = compare_packets(left, right);
        if c == Ordering::Less {
            num_ordered += 1;
            ordered_indices.push(i);
        }
        i += 1;
    }

    let mut sum = 0;
    for i in ordered_indices.iter() {
        sum += i;
    }

    println!("Sum of ordered pair indices: {}", sum);
}

fn part_2() {
    let mut packets = read_packets();

    let div_1 = PacketData::List(vec![PacketData::Integer(2)]);
    let div_2 = PacketData::List(vec![PacketData::Integer(6)]);
    packets.push(div_1.clone());
    packets.push(div_2.clone());

    packets.sort_by(compare_packets);

    let mut i = 1;
    let mut decoder_key = 1;
    for p in packets.iter() {
        if compare_packets(p, &div_1) == Ordering::Equal {
            decoder_key *= i;
        }
        if compare_packets(p, &div_2) == Ordering::Equal {
            decoder_key *= i;
        }
        
        i += 1;
    }

    println!("Decoder key: {}", decoder_key);
}

fn main() {
    println!("Advent of Code, Day 13");

    part_1();
    part_2();
}