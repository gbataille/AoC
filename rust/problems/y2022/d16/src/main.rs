use regex::Regex;
use std::collections::{HashMap, HashSet, VecDeque};
use std::env;
use std::fmt::Debug;
use std::fs;
use std::str::FromStr;
use std::vec::Vec;

#[derive(Debug)]
struct WorldState {
    time: u32,
    flow_rate: u32,
    released: u32,
    position: String,
    open_valves: HashSet<String>,
    paths_taken: HashSet<(String, String)>,
}

impl WorldState {
    fn new() -> Self {
        WorldState {
            time: 0,
            flow_rate: 0,
            released: 0,
            position: "AA".to_string(),
            open_valves: HashSet::new(),
            paths_taken: HashSet::new(),
        }
    }

    fn is_done(&self) -> bool {
        self.time == 30
    }

    fn add_one_minute(&self, valve_system: &ValveSystem) -> (Vec<Self>, bool) {
        println_dbg(format!("\n\n===================\nAdding on minute to {:?}", self).as_str());
        let mut new_worlds = Vec::new();

        let new_time = self.time + 1;

        // try to open a valve
        if self.open_valves.get(&self.position).is_none() {
            println_dbg(format!("Opening valve at {}", &self.position).as_str());
            let added_flow_rate = valve_system.valve_flow.get(&self.position).unwrap();
            if *added_flow_rate != 0 {
                let mut new_open_valves = self.open_valves.clone();
                new_open_valves.insert(self.position.clone());
                new_worlds.push(WorldState {
                    time: new_time,
                    flow_rate: self.flow_rate + added_flow_rate,
                    released: self.released + self.flow_rate,
                    position: self.position.clone(),
                    open_valves: new_open_valves,
                    paths_taken: self.paths_taken.clone(),
                });
                println_dbg(
                    format!(
                        "Opened valve at {}, and produced {:?}",
                        &self.position,
                        &new_worlds[new_worlds.len() - 1]
                    )
                    .as_str(),
                );
            } else {
                println_dbg(
                    format!("No flow for valve at {}, not opening", &self.position).as_str(),
                );
            }
        }

        // try to move
        for valve in valve_system.valve_paths.get(&self.position).unwrap().iter() {
            if self
                .paths_taken
                .get(&(self.position.clone(), valve.clone()))
                .is_some()
            {
                println_dbg(
                    format!(
                        "Already taken path at {} -> {}, not going there",
                        &self.position.clone(),
                        &valve.clone()
                    )
                    .as_str(),
                );
                continue;
            }
            let mut new_paths_taken = self.paths_taken.clone();
            new_paths_taken.insert((self.position.clone(), valve.clone()));
            new_worlds.push(WorldState {
                time: new_time,
                flow_rate: self.flow_rate,
                released: self.released + self.flow_rate,
                position: valve.clone(),
                open_valves: self.open_valves.clone(),
                paths_taken: new_paths_taken,
            });
            println_dbg(
                format!(
                    "Followed path {}, and produced {:?}",
                    &valve,
                    &new_worlds[new_worlds.len() - 1]
                )
                .as_str(),
            );
        }

        if new_worlds.len() == 0 {
            // don't move, do nothing
            new_worlds.push(WorldState {
                time: new_time,
                flow_rate: self.flow_rate,
                released: self.released + self.flow_rate,
                position: self.position.clone(),
                open_valves: self.open_valves.clone(),
                paths_taken: self.paths_taken.clone(),
            });
        }

        (new_worlds, new_time == 30)
    }
}

#[derive(Debug)]
struct ValveSystem {
    valve_flow: HashMap<String, u32>,
    useful_valves: HashSet<String>,
    valve_paths: HashMap<String, Vec<String>>,
}

impl ValveSystem {
    fn new() -> Self {
        ValveSystem {
            valve_flow: HashMap::new(),
            useful_valves: HashSet::new(),
            valve_paths: HashMap::new(),
        }
    }

    fn add_valve(&mut self, desc: &str) {
        let re = Regex::new(r"Valve (.{2}) has flow rate=(\d+); tunnels? leads? to valves? (.*)")
            .unwrap();

        println_dbg(desc);
        let cap = re.captures_iter(desc).next().unwrap();
        let valve_name = cap[1].to_string();
        let valve_flow = cap[2].parse::<u32>().unwrap();
        let mut valve_paths: Vec<String> = Vec::new();
        for item in cap[3].split(", ") {
            valve_paths.push(item.to_string());
        }

        self.valve_flow.insert(valve_name.clone(), valve_flow);
        if valve_flow > 0 {
            self.useful_valves.insert(valve_name.clone());
        }
        self.valve_paths.insert(valve_name.clone(), valve_paths);
    }
}

fn print_dbg(msg: &str) {
    if env::var("DEBUG").is_ok() {
        print!("{}", msg);
    }
}

fn println_dbg(msg: &str) {
    if env::var("DEBUG").is_ok() {
        println!("{}", msg);
    }
}

fn dbg_dbg(data: &dyn Debug) {
    if env::var("DEBUG").is_ok() {
        dbg!(data);
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    part1(&contents);
    part2(&contents);
}

fn part1(contents: &String) {}

fn part2(contents: &String) {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parts() {
        // env::set_var("DEBUG", "true");

        let test_contents = String::from_str(
            "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II",
        )
        .unwrap();

        part1(&test_contents);
        part2(&test_contents);
    }
}
