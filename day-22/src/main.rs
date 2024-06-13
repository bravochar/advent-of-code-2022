const FILENAME: &str = "./input";
//const FILENAME: &str = "./test";

use std::fs::File;
use std::io::{prelude::*, BufReader};

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

#[derive(Clone,Debug)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
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

    println!("Moving {} {:?}", dist, dir);

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
                    println!("Moved off of map! {} -> {}", x, new_x);
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
        println!("Moved to {}, {}", x, y);
    }

    (x, y)
}

fn part_1(map: Vec<Vec<Tile>>, password: Vec<Path>) -> i64 {
    let mut dir = Direction::Right;
    let mut x = 0;
    let mut y = 0;

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
                dir = match dir {
                    Direction::Right => Direction::Down,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Up,
                    Direction::Up => Direction::Right,
                };
            },
            Path::TurnLeft => {
                dir = match dir {
                    Direction::Left => Direction::Down,
                    Direction::Up => Direction::Left,
                    Direction::Right => Direction::Up,
                    Direction::Down => Direction::Right,
                };
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

fn part_2(_map: Vec<Vec<Tile>>) -> i64 {
    0
}

fn main() {
    println!("Advent of Code, Day 19");

    // read in the input
    let (input, password) = read_file(FILENAME);

    println!("{}", map_to_string(&input));
    println!("{:?}", password);

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
    let answer = part_2(input.clone());
    let elapsed = now.elapsed();
    println!("Part 2: {}", answer);
    println!("Took {:.5?}", elapsed);

    if FILENAME == "./test" {
        assert_eq!(answer, 5031);
    } else {
        //assert_eq!(answer, 3769668716709);
    }
}