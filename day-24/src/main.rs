const FILENAME: &str = "./input";
//const FILENAME: &str = "./test";

use std::fs::File;
use std::fmt;
use std::io::{prelude::*, BufReader};
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};

#[derive(Clone,Copy,Debug,PartialEq,Eq,Hash)]
enum Direction {
    North,
    South,
    East,
    West
}

impl fmt::Display for Direction {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Direction::East => '>',
            Direction::West => '<',
            Direction::North => '^',
            Direction::South => 'v'
        };
        write!(fmt, "{s}")
    }
}

#[derive(Clone,Copy,Debug,PartialEq,Eq,Hash)]
struct Point {
    x: usize,
    y: usize
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
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

    fn valid_moves(&self, length: usize, height: usize) -> Vec<Point> {
        let mut rval = vec![*self];

        if self.y < height || (self.y == height && self.x == length) {
            rval.push(self.move_dir(&Direction::South));
        }

        if self.y > 1 || (self.y == 1 && self.x == 1) {
            rval.push(self.move_dir(&Direction::North));
        }

        if self.y >= 1 && self.y <= height {
            if self.x < length {
                rval.push(self.move_dir(&Direction::East));
            }

            if self.x > 1 {
                rval.push(self.move_dir(&Direction::West));
            }
        }

        rval
    }
}

fn read_file(filename: &str) -> HashMap<Point, Vec<Direction>> {
    // Open the file
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    // Read file line by line
    let mut rval = HashMap::new();
    for (y, line) in reader.lines().enumerate() {
        let line = line.unwrap();

        for (x, c) in line.chars().enumerate() {
            match c {
                '.' => (),
                '#' => (),
                '>' => {
                    rval.insert(Point{x, y}, vec![Direction::East]);
                }
                'v' => {
                    rval.insert(Point{x, y}, vec![Direction::South]);
                }
                '<' => {
                    rval.insert(Point{x, y}, vec![Direction::West]);
                }
                '^' => {
                    rval.insert(Point{x, y}, vec![Direction::North]);
                }
                _ => panic!("Extraneous character: {}", c)
            };
        }
    }

    rval
}

fn get_map_bounds(map:  &HashMap<Point, Vec<Direction>>) -> (usize, usize) {
    let mut max_x = 0;
    let mut max_y = 0;

    for b in map.keys() {
        if b.x > max_x {
            max_x = b.x
        }
        if b.y > max_y {
            max_y = b.y
        }
    }

    (max_x, max_y)
}

fn bliz_to_char(b: &[Direction]) -> char {
    if b.len() == 1 {
        match b.first().unwrap() {
            Direction::North => '^',
            Direction::East => '>',
            Direction::West => '<',
            Direction::South => 'v'
        }
    } else {
        format!("{}", b.len()).pop().unwrap()
    }
}

fn wrap_bliz(mut p: Point, max_x: usize, max_y: usize) -> Point {
    if p.x > max_x {
        p.x = 1;
    } else if p.x == 0 {
        p.x = max_x;
    }
    if p.y > max_y {
        p.y = 1;
    } else if p.y == 0 {
        p.y = max_y;
    }
    p
}
fn next_bliz_map(map: &HashMap<Point, Vec<Direction>>, max_x: usize, max_y: usize) -> HashMap<Point, Vec<Direction>> {
    let mut rval = HashMap::new();

    for (p, blizzards) in map.iter() {
        for b in blizzards {
            let new_point = wrap_bliz(p.move_dir(b), max_x, max_y);
            rval.entry(new_point).or_insert(Vec::new()).push(*b);
        }
    }

    rval
}

#[allow(dead_code)]
fn print_map(map: &HashMap<Point, Vec<Direction>>, max_x: usize, max_y: usize) {
    for y in 0..=max_y+1 {
        let mut line = String::new();
        for x in 0..=max_x+1 {
            if x == 0 || x == max_x + 1 {
                line.push('#');
                continue;
            } else if y == 0 {
                match x {
                    1 => line.push('.'),
                    _ => line.push('#')
                }
                continue;
            } else if y == max_y + 1 {
                if x == max_x  {
                    line.push('.');
                } else {
                    line.push('#');
                }
                continue;
            }
            match map.get(&Point{x, y}) {
                Some(b) => line.push(bliz_to_char(b)),
                None => line.push('.')
            };
        }
        println!("{line}");
    }
    println!();
}

#[derive(Clone)]
struct Blizzards {
    length: usize,
    height: usize,
    maps: Vec<HashMap<Point, Vec<Direction>>>
}

impl Blizzards {
    fn from_map(map: HashMap<Point, Vec<Direction>>) -> Self {
        let (length, height) = get_map_bounds(&map);
        Blizzards {
            length,
            height,
            maps: vec![map]
        }
    }

    fn gen_next_map(&mut self) {
        self.maps.push(next_bliz_map(
            self.maps.last().expect("No maps in blizzard"),
            self.length,
            self.height)
        );
    }

    fn get_map(&mut self, minute: usize) -> &HashMap<Point, Vec<Direction>> {
        while self.maps.len() <= minute {
            self.gen_next_map();
        }

        self.maps.get(minute).expect("Map did not exist")
    }

    fn print_map(&mut self, minute:usize) {
        let l = self.length;
        let h = self.height;
        let map = self.get_map(minute);

        print_map(map, l, h);
    }
}

#[derive(Clone,Debug)]
struct Path {
    moves: Vec<Point>,
    length: usize,
    height: usize
}

impl Path {
    fn from_blizzards(b: &Blizzards) -> Self {
        Self {
            moves: vec![Point{x: 1, y: 0}],
            length: b.length,
            height: b.height
        }
    }

    fn next_paths(path: Self, b: &mut Blizzards) -> Vec<Self> {
        let mut rval = Vec::new();
        let t = path.moves.len();
        if t > 0 {
            let map_p = b.get_map(t - 1);

            assert!(!map_p.contains_key(path.moves.last().unwrap()));
        }
        let length = b.length;
        let height = b.height;
        let map = b.get_map(t);

        for p in path.moves.last().unwrap().valid_moves(length, height) {
            if !map.contains_key(&p) {
                let mut new_path = path.clone();
                new_path.moves.push(p);
                rval.push(new_path);
            }
        }

        rval
    }

    fn ideal_score(&self) -> usize {
        let mut rval = self.moves.len() - 1;
        let cur_pos = self.moves.last().expect("moves was empty?");
        rval += self.length - cur_pos.x;
        rval += self.height + 1 - cur_pos.y;

        rval
    }

    fn is_done(&self) -> bool {
        let cur_pos = self.moves.last().expect("moves was empty?");

        cur_pos.x == self.length && cur_pos.y == self.height + 1
    }

    #[allow(dead_code)]
    fn print_path(&self, b: &mut Blizzards) {
        for (t, p) in self.moves.iter().enumerate() {
            let map = b.get_map(t);
            println!("Minute {}, move to {:?}|", t, p);
            for y in 0..=self.height+1 {
                let mut line = String::new();
                for x in 0..=self.length+1 {
                    if x == p.x && y == p.y {
                        line.push('E');
                        continue;
                    }
                    if x == 0 || x == self.length + 1 {
                        line.push('#');
                        continue;
                    } else if y == 0 {
                        match x {
                            1 => line.push('.'),
                            _ => line.push('#')
                        }
                        continue;
                    } else if y == self.height + 1 {
                        if x == self.length  {
                            line.push('.');
                        } else {
                            line.push('#');
                        }
                        continue;
                    }
                    match map.get(&Point{x, y}) {
                        Some(b) => line.push(bliz_to_char(b)),
                        None => line.push('.')
                    };
                }
                println!("{line}");
            }
            println!();
        }

    }
}

impl Ord for Path {
    fn cmp(&self, other: &Self) -> Ordering {
        self.ideal_score().cmp(&other.ideal_score())
    }
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Path {
    fn eq(&self, other: &Self) -> bool {
        self.ideal_score() == other.ideal_score()
    }
}

impl Eq for Path { }

fn part_1(mut blizzards: Blizzards) -> i32 {
    let p = Path::from_blizzards(&blizzards);
    let mut paths = BinaryHeap::new();
    //let mut paths = Vec::new();
    paths.push(Reverse(p));

    let mut best_path: Option<Path> = None;
    let mut visited = HashSet::new();
    while let Some(Reverse(p)) = paths.pop() {
        // check for completed path
        if p.is_done() {
            if let Some(ref b) = best_path {
                if p < *b {
                    println!("Best path now {}", p.ideal_score());
                    best_path = Some(p);
                }
            } else {
                println!("Best path now {}", p.ideal_score());
                best_path = Some(p);
            }
            continue;
        }

        // prune paths if their ideal score is greater than best path
        if let Some(ref b) = best_path {
            if p > *b {
                continue;
            }
        }


        let new_paths = Path::next_paths(p, &mut blizzards);
        for p in new_paths {
            if !visited.contains(&(*p.moves.last().unwrap(), p.moves.len())) {
                visited.insert((*p.moves.last().unwrap(), p.moves.len()));
                paths.push(Reverse(p));
            }
        }
    }

    let best_path = best_path.unwrap();
    //best_path.print_path(&mut blizzards);
    
    best_path.ideal_score() as i32
}

fn part_2(mut _blizzards: Blizzards) -> i32 {
    0
}

fn main() {
    println!("Advent of Code, Day 19");

    // read in the input
    let map = read_file(FILENAME);

    let mut blizzards = Blizzards::from_map(map);
    println!("Read map of {} x {}", blizzards.length, blizzards.height);
    blizzards.print_map(0);
    //blizzards.print_map(blizzards.length * blizzards.height);

    let now = Instant::now();
    use std::time::Instant;
    let answer = part_1(blizzards.clone());
    let elapsed = now.elapsed();
    println!("Part 1: {}", answer);
    println!("Took {:.5?}", elapsed);

    if FILENAME == "./test" {
        assert_eq!(answer, 18);
    } else {
        assert_eq!(answer, 255);
    }

    let now = Instant::now();
    let answer = part_2(blizzards.clone());
    let elapsed = now.elapsed();
    println!("Part 2: {}", answer);
    println!("Took {:.5?}", elapsed);

    if FILENAME == "./test" {
        //assert_eq!(answer, 20);
    } else {
        //assert_eq!(answer, 925);
    }
}