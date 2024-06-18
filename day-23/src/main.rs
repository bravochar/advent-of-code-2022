const FILENAME: &str = "./input";
//const FILENAME: &str = "./test";

use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::collections::HashSet;
use std::collections::HashMap;

#[derive(Clone,Copy,Debug,PartialEq,Eq,Hash)]
enum Direction {
    North,
    South,
    East,
    West
}

#[derive(Clone,Copy,Debug,PartialEq,Eq,Hash)]
struct Point {
    x: i32,
    y: i32
}

impl Point {
    fn move_dir(&self, dir: &Direction) -> Self {
        match dir {
            Direction::North => {
                Self{x: self.x, y: self.y - 1}
            },
            Direction::East => {
                Self{x: self.x + 1, y: self.y}
            },
            Direction::South => {
                Self{x: self.x, y: self.y + 1}
            },
            Direction::West => {
                Self{x: self.x - 1, y: self.y}
            }
        }
    }
}

#[derive(Clone,Copy,Debug,PartialEq,Eq,Hash)]
struct Elf {
    loc: Point
}

impl Elf {

    fn are_elves_dir(&self, dir: &Direction, map: &HashSet<Point>) -> bool {
        let mut points = Vec::new();
        match dir {
            Direction::North => {
                points.push(Point{x: self.loc.x - 1, y: self.loc.y - 1});
                points.push(Point{x: self.loc.x,     y: self.loc.y - 1});
                points.push(Point{x: self.loc.x + 1, y: self.loc.y - 1});
            },
            Direction::East => {
                points.push(Point{x: self.loc.x + 1, y: self.loc.y - 1});
                points.push(Point{x: self.loc.x + 1, y: self.loc.y});
                points.push(Point{x: self.loc.x + 1, y: self.loc.y + 1});
            },
            Direction::South => {
                points.push(Point{x: self.loc.x - 1, y: self.loc.y + 1});
                points.push(Point{x: self.loc.x,     y: self.loc.y + 1});
                points.push(Point{x: self.loc.x + 1, y: self.loc.y + 1});
            },
            Direction::West => {
                points.push(Point{x: self.loc.x - 1, y: self.loc.y - 1});
                points.push(Point{x: self.loc.x - 1, y: self.loc.y});
                points.push(Point{x: self.loc.x - 1, y: self.loc.y + 1});
            }
        }

        points.iter().any(|p| {map.contains(p)})
    }

    fn are_adjacent_elves(&self, map: &HashSet<Point>) -> bool {
        let mut points = Vec::new();
        for x in self.loc.x-1..=self.loc.x+1 {
            for y in self.loc.y-1..=self.loc.y+1 {
                if !(x == self.loc.x && y == self.loc.y) {
                    points.push(Point{x, y});
                }
            }
        }

        points.iter().any(|p| {map.contains(p)})
    }

    fn move_in_dir(&self, dir: &Direction, map: &HashSet<Point>) -> Option<Point> {
        if !self.are_elves_dir(dir, map) {
            Some(self.loc.move_dir(dir))
        } else {
            None
        }
    }

    fn decide_move(&self, dir_order: &Vec<Direction>, map: &HashSet<Point>) -> Option<Point> {
        if self.are_adjacent_elves(map) {
            for d in dir_order {
                if let Some(p) = self.move_in_dir(d, map) {
                     return Some(p);
                }
            }
        }
        None
    }
}

fn read_file(filename: &str) -> Vec<Elf> {
    // Open the file
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    // Read file line by line
    let mut rval = Vec::new();
    for (y, line) in reader.lines().enumerate() {
        let line = line.unwrap();

        for (x, c) in line.chars().enumerate() {
            match c {
                '.' => (),
                '#' => rval.push(Elf{ loc: Point{x: x as i32, y: y as i32} }),
                _ => panic!("Extraneous character: {}", c)
            };
        }
    }

    rval
}

fn build_elf_map(elves: &[Elf]) -> HashSet<Point>{
    let mut rval = HashSet::new();

    for e in elves.iter() {
        rval.insert(e.loc);
    }

    rval
}

fn get_map_bounds(map: &HashSet<Point>) -> [i32; 4] {
    let mut max_x = 0;
    let mut min_x = 0;
    let mut max_y = 0;
    let mut min_y = 0;

    for p in map.iter() {
        if p.x < min_x {
            min_x = p.x
        } else if p.x > max_x {
            max_x = p.x
        }
        if p.y < min_y {
            min_y = p.y
        } else if p.y > max_y {
            max_y = p.y
        }
    }

    [min_x, max_x, min_y, max_y]
}

#[allow(dead_code)]
fn print_elf_map(map: &HashSet<Point>) {
    let bounds = get_map_bounds(map);

    println!();
    for y in bounds[2]-1..=bounds[3]+1 {
        let mut line = String::new();
        for x in bounds[0]-1..=bounds[1]+1 {
            match map.get(&Point{x, y}) {
                Some(_) => line.push('#'),
                None => line.push('.')
            };
        }
        println!("{line}");
    }
    println!();
}

fn full_turn(elves: &[Elf], dir_order: &Vec<Direction>) -> Option<Vec<Elf>> {
    // build current map
    let map = build_elf_map(elves);

    let mut moves = HashMap::new();
    let mut stands = Vec::new();
    for e in elves.iter() {
        match e.decide_move(dir_order, &map) {
            Some(p) => {
                moves.entry(p).or_insert(Vec::new()).push(e)
            },
            None => {
                stands.push(e)
            }
        };
    }

    if moves.is_empty() {
        return None;
    }

    let mut rval: Vec<Elf> = Vec::new();
    rval.extend(stands.iter().copied());
    for (p, mut movers) in moves.into_iter() {
        if movers.len() == 1 {
            let mut e = *movers.pop().expect("We just checked for 1");
            e.loc = p;
            rval.push(e);
        } else {
            rval.extend(movers);
        }
    }

    Some(rval)
}

fn part_1(mut elves: Vec<Elf>) -> i32 {
    let mut dir_order = vec![Direction::North, Direction::South, Direction::West, Direction::East];

    for _ in 0..10 {
        elves = full_turn(&elves, &dir_order).expect("Don't expect convergence in 10 rounds");

        let d = dir_order.remove(0);
        dir_order.push(d);
    }

    // build current map
    let map = build_elf_map(&elves);
    //print_elf_map(&map);

    let bounds = get_map_bounds(&map);
    let total_area = (bounds[1] - bounds[0] + 1) * (bounds[3] - bounds[2] + 1);

    total_area - elves.len() as i32
}

fn part_2(mut elves: Vec<Elf>) -> i32 {
    let mut dir_order = vec![Direction::North, Direction::South, Direction::West, Direction::East];

    let mut rounds = 1;
    while let Some(new_elves) = full_turn(&elves, &dir_order) {

        elves = new_elves;

        let d = dir_order.remove(0);
        dir_order.push(d);
        rounds += 1;
    }

    rounds
}

fn main() {
    println!("Advent of Code, Day 19");

    // read in the input
    let elves = read_file(FILENAME);

    let now = Instant::now();
    use std::time::Instant;
    let answer = part_1(elves.clone());
    let elapsed = now.elapsed();
    println!("Part 1: {}", answer);
    println!("Took {:.5?}", elapsed);

    if FILENAME == "./test" {
        assert_eq!(answer, 110);
    } else {
        assert_eq!(answer, 3871);
    }

    let now = Instant::now();
    let answer = part_2(elves.clone());
    let elapsed = now.elapsed();
    println!("Part 2: {}", answer);
    println!("Took {:.5?}", elapsed);

    if FILENAME == "./test" {
        assert_eq!(answer, 20);
    } else {
        assert_eq!(answer, 925);
    }
}