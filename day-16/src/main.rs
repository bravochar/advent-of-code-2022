
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
                Step::Move(v) => {
                    write!(f, "You move to valve {}.", v.name)
                },
                Step::Open(v) => {
                    write!(f, "You open valve {}.", v.name)
                }
            }
    }
}

#[derive(Clone, Debug)]
struct Path<'v> {
    steps: Vec<Step<'v>>,
    open_valves: Vec<&'v Valve>,
    closed_valves: Vec<&'v Valve>,
    current_flow: i32,
    total_flow: i32,
    cur_valve: &'v Valve,
    rem_time: i32,
}

impl <'v> Path<'v> {
    fn new(closed_valves: Vec<&'v Valve>, v: &'v Valve, rem_time: i32) -> Path<'v> {
        let mut p = Path {
            steps: Vec::new(),
            open_valves: Vec::new(),
            closed_valves,
            current_flow: 0,
            total_flow: 0,
            cur_valve: v,
            rem_time: rem_time + 1
        };

        p.add_step(Step::Move(v));

        p
    }

    #[allow(dead_code)]
    fn to_string(&self) -> String {
        let mut rval = String::new();

        let mut open_valves: Vec<String> = Vec::new();
        let mut current_flow = 0;
        for (i, s) in self.steps.iter().enumerate() {
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

        rval + &format!("Total_flow: {}", self.final_score())
    }

    fn open_valve(&mut self, v: &'v Valve) {
        self.closed_valves.retain(|x| {
            !&x.name.contains(&v.name)});
        self.open_valves.push(v);
        self.current_flow += v.flow_rate;
    }

    fn add_step(&mut self, s: Step<'v>) {
        self.total_flow += self.current_flow;
        match s {
            Step::Open(v) => {
                self.open_valve(v);
                self.cur_valve = v
            },
            Step::Move(v) => self.cur_valve = v
        }
        self.steps.push(s);
        self.rem_time -= 1;
    }

    fn final_score(&self) -> i32 {
        self.total_flow + self.current_flow * self.rem_time
    }

    fn ideal_score(&self) -> i32 {
        let mut rval = self.total_flow;
        let mut flow = self.current_flow;
        
        for i in 0..self.rem_time {
            rval += flow;
            match self.closed_valves.get(i as usize) {
                Some(v) => flow += v.flow_rate,
                _ => ()
            }
        }
        rval
    }

    fn next_steps(&self) -> Vec<String> {
        let mut rval = Vec::new();
        let last_step = self.steps.last().unwrap();

        let v = match last_step {
            Step::Move(v) => {
                v
            }, 
            Step::Open(v) => v
        };

        for t in v.tunnels.iter() {
            rval.push(t.clone());
        }

        rval
    }

    #[allow(dead_code)]
    fn path_len(&self) -> i32 {
        return self.steps.len() as i32 - 1
    }

    fn can_open(&self) -> bool {
        let v = self.cur_valve;

        if v.flow_rate > 0 && !self.open_valves.iter().any(|x| {
            x.name.contains(&v.name)}) {
                true
        } else {
            false
        }
    }

    // TODO: need add_step, cmp, and best_move_n
}

fn open_next_valve<'v>(
    p: Path<'v>,
    valve_map: &HashMap<String,
    &'v Valve>) -> Vec<Path<'v>> {
    let mut rvals: Vec<Path> = Vec::new();

    /*
     * breadth-first search to open another unopened valve
     */
    let mut poss_paths: Vec<Path> = Vec::new();
    let mut visited_valves: Vec<String> = Vec::new();
    poss_paths.push(p.clone());

    while !poss_paths.is_empty() {
        let mut new_steps: Vec<Path> = Vec::new();
        for p in poss_paths.into_iter() {
            if p.can_open() {
                let mut new_path = p.clone();
                new_path.add_step(Step::Open(p.cur_valve));
                rvals.push(new_path);
                
            } else if p.closed_valves.len() == 0 {
                //println!("No more valves to open");
                continue
            }

            for v_name in p.next_steps().iter() {
                if visited_valves.iter().any(|s| {s.eq(v_name)}) {
                    continue;
                } else if p.steps.len() as i32 > (NUM_MIN - 1) {
                    continue;
                }
                visited_valves.push(v_name.to_owned());

                let v = valve_map.get(v_name).unwrap();
                let mut new_path = p.clone();
                new_path.add_step(Step::Move(v));

                new_steps.push(new_path);
            }

        }

        poss_paths = new_steps;
        /*
        println!("{} paths after {} iterations", poss_paths.len(), i);
        for p in poss_paths.iter() {
            println!("  {} steps to end at {}", p.steps.len(), p.cur_pos());
        }
         */
    }

    rvals
}

fn part_1() {
    let valves = read_file();

    // hashmap to find next move
    let mut valve_map = HashMap::new();
    let mut closed_valves = Vec::new();
    for v in valves.iter() {
        valve_map.insert(v.name.clone(), v);
        if v.flow_rate > 0 {
            closed_valves.push(v);
        }
        //println!("{}", v);
    }

    println!("Need to open {} valves...", closed_valves.len());
    closed_valves.sort_by(|a, b| {
        b.flow_rate.cmp(&a.flow_rate)
    });
    for v in closed_valves.iter() {
        println!("  {}: {}", v.name, v.flow_rate);
    }
    closed_valves.sort_by(|a,b| {b.flow_rate.cmp(&a.flow_rate)});
    let first_step = valve_map.get("AA").unwrap();

    let p = Path::new(closed_valves, &first_step, NUM_MIN);

    let mut valves_left: Vec<&Valve> = valves.iter().filter(
        |v| {v.flow_rate > 0}).collect();
    valves_left.sort();
    
    let mut high_score = 0;
    let mut best_path = p.clone();
    let mut paths = vec![p];
    while !paths.is_empty() {
        let mut next_paths: Vec<Path> = Vec::new();
        for path in paths.into_iter() {
            if path.ideal_score() <= high_score {
                continue;

            } else if path.final_score() > high_score {
                high_score = path.final_score();
                best_path = path.clone();
            }

            let new_paths = open_next_valve(path, &valve_map);

            if !new_paths.is_empty() {
                next_paths.extend(new_paths);
            }
        }

        paths = next_paths;
        println!("Working with {} paths; best {}", paths.len(), high_score);
    }
    println!("Best path of {} steps: {}",
        best_path.steps.len(),
        high_score);
    
    if FILENAME.eq("./test") {
        assert_eq!(high_score, 1651);
    } else {
        assert_eq!(high_score, 1673);
    }
    //println!("{}", best_path.to_string());
}

fn part_2() {
}

fn main() {
    println!("Advent of Code, Day 16");

    part_1();
    part_2();
}
