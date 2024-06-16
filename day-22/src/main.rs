const FILENAME: &str = "./input";
//const FILENAME: &str = "./test";

use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::collections::HashMap;

#[derive(Clone,Debug,PartialEq)]
enum Tile {
    Open,
    Wall,
    Nope,
}

impl Tile {
    fn from_char(c: char) -> Tile {
        match c {
            '.' => Tile::Open,
            '#' => Tile::Wall,
            _ => Tile::Nope
        }
    }
}

fn map_to_string(map: &Vec<Vec<Tile>>) -> String {
    let mut rval = String::new();

    for row in map.iter() {
        for tile in row.iter() {
            rval.push(match tile {
                Tile::Nope => ' ',
                Tile::Open => '.',
                Tile::Wall => '#',
            })
        }
        rval.push('\n');
    }


    rval
}

#[derive(Clone,Debug)]
enum Path {
    Move(i32),
    TurnRight,
    TurnLeft
}

#[derive(Clone,Copy,Debug,PartialEq,Eq,Hash)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

impl Direction {
    fn turn_right(&self) -> Self {
        match self {
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Up => Direction::Right
        }
    }

    fn turn_left(&self) -> Self {
        match self {
            Direction::Left => Direction::Down,
            Direction::Up => Direction::Left,
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right
        }
    }
}

fn read_file(filename: &str) -> (Vec<Vec<Tile>>, Vec<Path>) {
    // Open the file
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    // Read file line by line
    let mut rval = Vec::new();
    let mut read_password = false;
    let mut password = String::new();
    for line in reader.lines() {
        let line = line.unwrap();

        // TODO: check for empty line before password
        if line.is_empty() {
            read_password = true;
            continue
        }

        if read_password {
            password = line;
            break;
        }

        let mut row = Vec::new();
        for c in line.chars() {
            row.push(Tile::from_char(c));
        }
        rval.push(row);
    }

    // TODO: read password
    let path = path_from_str(&password);

    (rval, path)
}

fn path_from_str(line: &str) -> Vec<Path> {
    let mut rval = Vec::new();

    let mut num_str = String::new();
    for c in line.chars() {
        match c {
            'R' => {
                if !num_str.is_empty() {
                    let num = num_str.parse().unwrap();
                    rval.push(Path::Move(num));
                    num_str = String::new();
                }
                rval.push(Path::TurnRight);
            },
            'L' => {
                if !num_str.is_empty() {
                    let num = num_str.parse().unwrap();
                    rval.push(Path::Move(num));
                    num_str = String::new();
                }
                rval.push(Path::TurnLeft);
            },
            _ => num_str.push(c)
        };
    }

    // catch last move, if needed
    if !num_str.is_empty() {
        let num = num_str.parse().unwrap();
        rval.push(Path::Move(num));
    }

    rval
}

fn find_first_x(y: &usize, map: &Vec<Vec<Tile>>) -> usize {
    for x in 0..map.len() {
        let row = map.get(x).unwrap();
        match row.get(*y) {
            Some(Tile::Open) => {
                return x;
            },
            Some(Tile::Wall) => {
                return x;
            },
            _ => ()
        };
    }

    panic!("No valid x for {}", y);
}

fn find_first_y(x: &usize, map: &Vec<Vec<Tile>>) -> usize {
    let row = map.get(*x).unwrap();
    for y in 0..row.len() {
        match row.get(y) {
            Some(Tile::Open) => {
                return y;
            },
            Some(Tile::Wall) => {
                return y;
            },
            _ => ()
        };
    }

    panic!("No valid y for {}", x);
}

fn find_last_y(x: &usize, map: &Vec<Vec<Tile>>) -> usize {
    let row = map.get(*x).unwrap();
    for y in (0..row.len()).rev() {
        match row.get(y) {
            Some(Tile::Open) => {
                return y;
            },
            Some(Tile::Wall) => {
                return y;
            },
            _ => ()
        };
    }
    panic!("No valid y for {}", x);
}

fn find_last_x(y: &usize, map: &Vec<Vec<Tile>>) -> usize {
    for x in (0..map.len()).rev() {
        let row = map.get(x).unwrap();
        match row.get(*y) {
            Some(Tile::Open) => {
                return x;
            },
            Some(Tile::Wall) => {
                return x;
            },
            _ => ()
        };
    }

    panic!("No valid x for {}", y);
}

fn move_on_map(
        mut x: usize,
        mut y: usize,
        dist: &i32,
        dir: &Direction,
        map: &Vec<Vec<Tile>>) -> (usize, usize) {
    let mut new_x;
    let mut new_y;

    //println!("Moving {} {:?}", dist, dir);

    for _ in 0..*dist {
        new_x = x;
        new_y = y;

        // increment coordinate
        match dir {
            Direction::Up => {
                if new_x > 0 {
                    new_x -= 1;
                } else {
                    new_x = find_last_x(&new_y, map);
                }
                new_x = match map.get(new_x).unwrap().get(new_y) {
                    Some(Tile::Nope) => find_last_x(&new_y, map),
                    None => find_last_x(&new_y, map),
                    _ => new_x
                }
            },
            Direction::Right => {
                new_y += 1;
                if new_y >= map.get(new_x).unwrap().len() {
                    new_y = find_first_y(&new_x, map);
                }
            },
            Direction::Down => {
                new_x += 1;
                if new_x >= map.len() {
                    //println!("Moved off of map! {} -> {}", x, new_x);
                    new_x = find_first_x(&new_y, map);
                } else {
                    new_x = match map.get(new_x).unwrap().get(new_y) {
                        Some(Tile::Nope) => find_first_x(&new_y, map),
                        None => find_first_x(&new_y, map),
                        _ => new_x
                    };
                }
            },
            Direction::Left => {
                if new_y > 0 {
                    new_y -= 1;
                } else {
                    new_y = find_last_y(&new_x, map);
                }

                if map.get(new_x).unwrap().get(new_y).unwrap()
                        == &Tile::Nope {
                    new_y = find_last_y(&new_x, map);
                }
            },

        }

        // check for valid move
        let row = map.get(new_x).unwrap();
        match row.get(new_y) {
            Some(Tile::Wall) => {
                // done moving if we hit a wall, return previous coords
                return (x, y);
            },
            Some(Tile::Open) => (),
            _ => {
                println!("At {}, {}", new_x, new_y);
                panic!("Unexpected problem: {:?}", row);
            }
        }

        x = new_x;
        y = new_y;
        //println!("Moved to {}, {}", x, y);
    }

    (x, y)
}

fn valid_move(p: (usize, usize), d: Direction, m: usize, map: &Vec<Vec<Tile>>) -> Option<(usize, usize)> {
    let (mut x, mut y) = p;

    // get new coords, checking for map bounds
    match d {
        Direction::Down => {
            y += m;
        },
        Direction::Up => {
            if m > y {
                return None;
            } else {
                y -= m;
            }
        }
        Direction::Left => {
            if m > x {
                return None;
            } else {
                x -= m;
            }
        },
        Direction::Right => {
            x += m;
        }
    }

    // verify new coord is not Tile::None
    match map.get(y) {
        Some(r) => {
            match r.get(x) {
                Some(t) => {
                    match t {
                        Tile::Nope => return None,
                        _ => return Some((x, y))
                    };
                },
                _ => return None,
            };
        },
        _ => return None
    };
}

fn zip_edges_pair(
    e1: &((usize, usize), (usize, usize), Direction),
    e2: &((usize, usize), (usize, usize), Direction),
    map: &Vec<Vec<Tile>>
        ) -> HashMap<((usize, usize), Direction), ((usize, usize), Direction)> {
    let mut rval = HashMap::new();

    let mut p1 = e1.1;
    let mut p2 = e2.0;
    let e1_dir = e1.2.turn_left();
    let e2_dir = e2.2.turn_right();
    let p1_dir = e1_dir.turn_left();
    let p2_dir = e2_dir.turn_right();

    loop {
        //println!("    ({}, {}) {:?} -> ({}, {}) {:?}",
        //    p1.0, p1.1, e1.2, p2.0, p2.1, p2_dir);
        rval.insert(((p1.0, p1.1), e1.2), ((p2.0, p2.1), p2_dir));
        //println!("    ({}, {}) {:?} -> ({}, {}) {:?}",
        //    p2.0, p2.1, e2.2, p1.0, p1.1, p1_dir);
        rval.insert(((p2.0, p2.1), e2.2), ((p1.0, p1.1), p1_dir));

        // check for end condition
        if p1 == e1.0 {
            break;
        }

        // increment points
        p1 = valid_move(p1, e1_dir, 1, map).unwrap();
        p2 = valid_move(p2, e2_dir, 1, map).unwrap();
    }
    
    rval
}

fn zip_edges(
            mut i: usize,
            edges: &Vec<((usize, usize), (usize, usize), Direction)>,
            map: &Vec<Vec<Tile>>
        ) -> HashMap<((usize, usize), Direction), ((usize, usize), Direction)> {
    let mut rval = HashMap::new();

    //println!("Zipping form inside corner");

    let mut j = (i + 1) % edges.len();
    let mut edge_1 = edges.get(i).unwrap();
    let mut edge_2 = edges.get(j).unwrap();


    loop {
        // zip together edges
        //println!("  Zip edges: {:?} {:?}", edge_1, edge_2);
        rval.extend(zip_edges_pair(edge_1, edge_2, map));

        // get next edges
        i = if i > 0 {
            i - 1
        } else {
            edges.len() - 1
        };
        j = (j + 1) % edges.len();

        let new_1 = edges.get(i).unwrap();
        let new_2 = edges.get(j).unwrap();

        if new_1.2 != edge_1.2 && new_2.2 != edge_2.2 {
            // both next edges were around corners - we're done
            break;
        }

        edge_1 = new_1;
        edge_2 = new_2;
    }

    rval
}

fn build_wrap_map(map: &Vec<Vec<Tile>>
        ) -> HashMap<((usize, usize), Direction), ((usize, usize), Direction)> {
    let mut rval = HashMap::new();
    let mut x: usize = 0;
    let mut edge_dir = Direction::Up;
    let mut trace_dir = Direction::Right;

    // find starting x
    for (i, t) in map.get(0).unwrap().iter().enumerate() {
        match t {
            Tile::Open => {
                x = i;
                break;
            },
            _ => ()
        };
    }

    // divine length of side of cube
    let map_height = map.len();
    let mut map_widths = Vec::new();
    for row in map.iter() {
        if !map_widths.contains(&row.len()) {
            map_widths.push(row.len());
        }
    }
    let map_width = map_widths.iter().max().unwrap().clone();
    let side_length = if map_width / 3 * 4 == map_height {
        map_width / 3
    } else if map_height / 3 * 4 == map_width {
        map_height / 3
    } else {
        panic!("MATH HAS FAILED US");
    };
    let edge_add = side_length - 1;

    // find all of the edges and the direction "off" that edge
    let mut edges = Vec::new();
    let start = (x, 0);
    let mut cur = start.clone();
    let mut edge_end = (cur.0 + edge_add, cur.1);
    edges.push( (cur, edge_end, edge_dir) );
    loop {
        // find start and direction of the next edge
        match valid_move(edge_end, trace_dir, 1, map) {
            None => {
                // outside corner
                cur = edge_end;
                edge_dir = edge_dir.turn_right();
                trace_dir = trace_dir.turn_right();            },
            Some(p) => {
                match valid_move(p, edge_dir, 1, map) {
                    None => {
                        // parallel edge
                        cur = p;
                    },
                    Some(p) => {
                        // inside corner
                        cur = p;
                        edge_dir = edge_dir.turn_left();
                        trace_dir = trace_dir.turn_left();
                    }
                }

            }
        };
        edge_end = valid_move(cur, trace_dir, edge_add, map).unwrap();
        
        if cur == start {
            break;
        } else {
            edges.push( (cur, edge_end, edge_dir) );
        }
    }

    for edge in edges.iter() {
        //println!("Edge from {:?} to {:?} pointed {:?}",
        //    edge.0, edge.1, edge.2);
    }

    // find an inside corner and "zip" it up
    for (i, e_t) in edges.windows(2).enumerate() {
        let edge_1 = e_t[0];
        let edge_2 = e_t[1];

        if edge_1.2.turn_left() == edge_2.2 {
            rval.extend(zip_edges(i, &edges, map));
        }
    }

    rval
}

fn part_1(map: Vec<Vec<Tile>>, password: Vec<Path>) -> i64 {
    let mut dir = Direction::Right;
    let mut x = 0;
    let mut y = 0;

    // find starting y
    for (i, t) in map.get(0).unwrap().iter().enumerate() {
        match t {
            Tile::Open => {
                y = i;
                break;
            },
            _ => ()
        };
    }
    
    println!("Starting at {}, {}", x, y);

    for m in password.iter() {
        match m {
            Path::TurnRight => {
                dir = dir.turn_right();
            },
            Path::TurnLeft => {
                dir = dir.turn_left();
            },
            Path::Move(dist) => {
                (x, y) = move_on_map(x, y, dist, &dir, &map);
            }
        };
    }

    1000 * (x as i64 + 1) + 4 * (y as i64 + 1) + match dir {
        Direction::Right => 0,
        Direction::Down => 1,
        Direction::Left => 2,
        Direction::Up => 3
    }
}

fn move_around_cube(
        mut x: usize,
        mut y: usize,
        dist: &i32,
        mut dir: Direction,
        map: &Vec<Vec<Tile>>,
        wrap: &HashMap<((usize, usize), Direction), ((usize, usize), Direction)>
    ) -> (usize, usize, Direction) {
    let mut new_x;
    let mut new_y;
    let mut new_dir = dir.clone();

    //println!("Moving {} {:?}", dist, dir);

    for _ in 0..*dist {
        new_x = x;
        new_y = y;

        if wrap.contains_key(&((x, y), dir.clone())) {
            //println!("Getting wrap point for {}, {}", x, y);
            let p = wrap.get(&((x, y), dir.clone())).unwrap();
            new_x = p.0.0.clone();
            new_y = p.0.1.clone();
            new_dir = p.1.clone();
        } else {
            match dir {
                Direction::Up => { new_y -= 1; },
                Direction::Right => { new_x += 1; },
                Direction::Down => { new_y += 1; },
                Direction::Left => { new_x -= 1; },
            }
        }

        // check for valid move
        let row = map.get(new_y).unwrap();
        match row.get(new_x) {
            Some(Tile::Wall) => {
                // done moving if we hit a wall, return previous coords
                //println!("Hit wall at {}, {}", new_x, new_y);
                break;
            },
            Some(Tile::Open) => (),
            Some(Tile::Nope) => {
                println!("NOPE At {}, {}", new_x, new_y);
                panic!("Off of map: {:?}", row);
            },
            None => {
                println!("Out of bounds at {}, {}", new_x, new_y);
                panic!("Out of bounds: {:?}", row);
            }
        }

        x = new_x;
        y = new_y;
        dir = new_dir;
        //println!("Moved to ({}, {}) {:?}", x, y, dir);
    }

    (x, y, dir)
}

fn part_2(map: Vec<Vec<Tile>>, password: Vec<Path>) -> i64 {
    let wrap = build_wrap_map(&map);
    let mut dir = Direction::Right;
    let mut x = 0;
    let mut y = 0;

    //println!("{:?}", wrap);

    // find starting y
    for (i, t) in map.get(0).unwrap().iter().enumerate() {
        match t {
            Tile::Open => {
                x = i;
                break;
            },
            _ => ()
        };
    }
    
    println!("Starting at {}, {}", x, y);

    for m in password.iter() {
        match m {
            Path::TurnRight => {
                dir = dir.turn_right();
            },
            Path::TurnLeft => {
                dir = dir.turn_left();
            },
            Path::Move(dist) => {
                (x, y, dir) = move_around_cube(x, y, dist, dir, &map, &wrap);
            }
        };
    }

    1000 * (y as i64 + 1) + 4 * (x as i64 + 1) + match dir {
        Direction::Right => 0,
        Direction::Down => 1,
        Direction::Left => 2,
        Direction::Up => 3
    }
}

fn main() {
    println!("Advent of Code, Day 19");

    // read in the input
    let (input, password) = read_file(FILENAME);

    //println!("{}", map_to_string(&input));
    //println!("{:?}", password);

    let now = Instant::now();
    use std::time::Instant;
    let answer = part_1(input.clone(), password.clone());
    let elapsed = now.elapsed();
    println!("Part 1: {}", answer);
    println!("Took {:.5?}", elapsed);

    if FILENAME == "./test" {
        assert_eq!(answer, 6032);
    } else {
        assert_eq!(answer, 11464);
    }

    let now = Instant::now();
    let answer = part_2(input.clone(), password.clone());
    let elapsed = now.elapsed();
    println!("Part 2: {}", answer);
    println!("Took {:.5?}", elapsed);

    if FILENAME == "./test" {
        assert_eq!(answer, 5031);
    } else {
        assert_eq!(answer, 197122);
    }
}