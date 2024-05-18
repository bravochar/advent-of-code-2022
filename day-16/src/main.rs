
use core::cmp::Ordering;
use std::fmt;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::collections::HashMap;

//const FILENAME: &str = "./input";
const FILENAME: &str = "./test";
const NUM_MIN: i32 = 30;

#[derive(Clone, Debug)]
struct Valve {
    name: String,
    flow_rate: i32,
    tunnels: Vec<String>,
}

impl fmt::Display for Valve {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let tunnel_names = self.tunnels.join(", ");
        let tunnel_desc = match self.tunnels.len() {
            1 => "tunnel leads to valve ",
            _ => "tunnels lead to valves"
        };
        write!(f, "Valve {} has flow rate {}: {} {}",
            self.name,
            self.flow_rate,
            tunnel_desc,
            tunnel_names)
    }
}

impl Valve {
    fn new(name: String, flow_rate: i32, tunnels: Vec<String>) -> Valve {
        Valve{
            name,
            flow_rate,
            tunnels}
    }
}

impl Ord for Valve {
    fn cmp(&self, other: &Self) -> Ordering {
        let rval = self.flow_rate.cmp(&other.flow_rate);

        if rval == Ordering::Equal {
            self.name.cmp(&other.name)
        } else {
            rval
        }
    }
}
impl PartialOrd for Valve {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for Valve {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}
impl Eq for Valve { }

fn read_file() -> Vec<Valve> {
    let mut rval: Vec<Valve> = Vec::new();

    // Open the file
    let file = File::open(FILENAME).unwrap();
    let mut reader = BufReader::new(file);
    // Read file line by line
    loop {
        let mut line = Default::default();
        let line_size = reader.read_line(&mut line).unwrap();
        match line_size  {
            0 => break, // EOF
            _ => (),
        };

        // pull out all of the coords
        let line = line.trim();
        let (_, line) = line.trim().split_once("Valve ").unwrap();
        let (name, line) = line.split_once(" has flow rate=").unwrap();
        let (flow_rate, line) = line.split_once("; ").unwrap();
        println!("{}", line);
        let tunnel_iter = line.split(" ");
        let tunnels = tunnel_iter.skip(4).map(|s| {
            s.trim_end_matches(",").to_string()}).collect();

        // make vec of String
        rval.push(Valve::new(
            name.to_string(),
            flow_rate.parse().unwrap(),
            tunnels));
    }

    rval
}

#[derive(Clone, Debug)]
enum Step<'v> {
    Move(&'v Valve),
    Open(&'v Valve)
}

impl <'v> fmt::Display for Step<'v> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Step::Move(s) => {
                    write!(f, "You move to valve {}.", s.name)
                },
                Step::Open(s) => {
                    write!(f, "You open valve {}.", s.name)
                }
            }
    }
}

#[derive(Clone, Debug)]
struct Path<'v> {
    steps: Vec<Step<'v>>,
    open_valves: Vec<&'v Valve>,
    current_flow: i32,
    total_flow: i32,
}

impl <'v> Path<'v> {
    fn new() -> Path<'v> {
        Path {
            steps: Vec::new(),
            open_valves: Vec::new(),
            current_flow: 0,
            total_flow: 0,
        }
    }

    fn to_string(&self, valve_map: HashMap<String, &Valve>) -> String {
        let mut rval = String::new();

        let mut open_valves: Vec<String> = Vec::new();
        let mut current_flow = 0;
        let mut total_flow = 0;
        for (i, s) in self.steps.iter().enumerate() {
            total_flow += current_flow;
            rval += &format!("== Minute {} ==\n", i);
            match open_valves.len() {
                0 => rval.push_str("No valves are open\n"),
                1 => rval += &format!("Valve {} is open, releasing {} pressure.\n",
                        open_valves.first().unwrap(),
                        current_flow),
                _ => {
                    rval += &format!("Valves {:?} are open, releasing {} pressure.\n",
                        open_valves,
                        current_flow)
                }
            }
            rval += &format!("{}\n", s);
            match s {
                Step::Move(_) => {
                },
                Step::Open(v) => {
                    open_valves.push(v.name.to_owned());
                    current_flow += v.flow_rate;
                }
            }
            rval += "\n";
        }

        rval + &format!("Total_flow: {total_flow}")
    }

    fn add_step(&mut self, s: Step<'v>) {
        self.total_flow += self.current_flow;
        match s {
            Step::Open(v) => {
                self.open_valves.push(v);
                self.current_flow += v.flow_rate;
            },
            _ => ()
        }
        self.steps.push(s);
    }

    fn next_steps(&self) -> Vec<String> {
        let mut rval = Vec::new();
        let last_step = self.steps.last().unwrap();

        let v = match last_step {
            Step::Move(v) => {
                if !self.open_valves.iter().any(
                    |x| {x.eq(v)}) {
                    if v.flow_rate > 0 {
                        rval.push(v.name.clone());
                    }
                }
                v
            }, 
            Step::Open(v) => v
        };

        for t in v.tunnels.iter() {
            rval.push(t.clone());
        }

        rval
    }

    fn path_len(&self) -> i32 {
        return self.steps.len() as i32 - 1
    }

    // TODO: need add_step, cmp, and best_move_n
}

fn best_path_to(p: &Path, v: &Valve, valve_map: &HashMap<String, &Valve>) -> Vec<String> {
    let rval: Vec<String> = Vec::new();

    // breadth-first search to find shortest path
    // XXX: We will likely need to change this to look for the highest value
    //      path in the future - but start with the basic search
    let mut poss_steps: Vec<Vec<String>> = Vec::new();
    for s in p.next_steps() {
        poss_steps.push(vec![s]);
    }
    loop {
        let mut new_steps: Vec<Vec<Step>> = Vec::new();
        for p in poss_steps.iter() {
            break;
        }

    }

    rval
}

fn part_1() {
    let valves = read_file();
    let first_step = valves.iter().next().unwrap();

    // hashmap to find next move
    let mut valve_map = HashMap::new();
    for v in valves.iter() {
        valve_map.insert(v.name.clone(), v);
        println!("{}", v);
    }

    /*
     * With 30 decisions to make, most with at least two options,
     * we cannot do a breadth-first search, and depth first would
     * involve a _lot_ of calculations. We'll need to implement a
     * greedy search to look at the best options first?
     * 
     * We can start with a simple 1-optimal step, but we'll likely
     * need to up that to 2- or 3-optimal before we're really done.
     * 
     * WRONG: We need to be able to move toward the valve with the largest flow
     * rate in order to get it open ASAP. 
     * 
     * TODO: calculate the opportunity cost of runing on a "large" valve along
     * that path - will it generate enough output to justify delaying opening
     * the larger valve by one minute. Actually, that's pretty simple!
     */
    let mut p = Path::new();
    p.add_step(
        Step::Move(first_step));

    let mut valves_left: Vec<&Valve> = valves.iter().filter(
        |v| {v.flow_rate > 0}).collect();
    valves_left.sort();
    while p.path_len() < NUM_MIN && !valves_left.is_empty(){
        // find largest unopened valve
        let next_valve = valves_left.pop().unwrap();
        println!("Moving to {} for {}", next_valve.name, next_valve.flow_rate);

        // TODO: actually loop
        break;
    }

    println!("{}", p.to_string(valve_map));
}

fn part_2() {
}

fn main() {
    println!("Advent of Code, Day 16");

    part_1();
    part_2();
}
