use std::{collections::{HashMap, BTreeSet}, str::FromStr};

pub struct Valve {
    flow_rate: u64,
    connections: Vec<String>,
}

impl Valve {
    pub fn new(flow_rate: u64, connections: Vec<String>) -> Self {
        Self { flow_rate, connections }
    }
    
    #[cfg(test)]
    pub fn flow_rate(&self) -> u64 {
        self.flow_rate
    }
    
    #[cfg(test)]
    pub fn connections(&self) -> Vec<String> {
        self.connections.clone()
    }
}

pub struct TunnelSystem {
    valves: HashMap<String, Valve>,
}

impl TunnelSystem {
    pub fn valve(&self, key: &str) -> Option<&Valve> {
        self.valves.get(key)
    }

    #[cfg(test)]
    pub fn valve_list(&self) -> Vec<&String> {
        self.valves.keys().collect()
    }
    
}

impl FromStr for TunnelSystem {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.split("\n").filter(|line| !line.is_empty()).collect();
        let mut valves = HashMap::new();
        
        for line in lines {
            let (valve_name_part, rest) = line.split_once(" has flow ").unwrap();
            let valve_name = valve_name_part.strip_prefix("Valve ").unwrap();
            let (flow_rate_part, connections_part) = rest.split_once("; tunnels lead to valve").unwrap_or(rest.split_once("; tunnel leads to valve").unwrap_or_default());
            let flow_rate = flow_rate_part.strip_prefix("rate=").unwrap().parse().unwrap();
            let connections = connections_part.strip_prefix("s ").unwrap_or(connections_part.strip_prefix(" ").unwrap_or_default()).split(", ").map(|str| str.to_owned()).collect();
            
            let valve = Valve::new(flow_rate, connections);

            valves.insert(valve_name.to_owned(), valve);
        }
        
        Ok(TunnelSystem { valves })
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct SolveState {
    current_pos: String,
    last: bool,
    remaining_time: u64,
    open_set: BTreeSet<String>,
}

impl SolveState {
    pub fn new(current_pos: String, last: bool, remaining_time: u64, open_set: BTreeSet<String>) -> Self {
        Self { current_pos, last, remaining_time, open_set }
    }
}

pub fn solve(tunnel_system: &TunnelSystem, start_time: u64, start_pos: String) -> u64 {
    solve_recursive(tunnel_system, start_pos.clone(), start_time, BTreeSet::new(), &mut HashMap::new(), true, start_pos, start_time)
}

pub fn solve_with_elephant(tunnel_system: &TunnelSystem, start_time: u64, start_pos: String) -> u64 {
    solve_recursive(tunnel_system, start_pos.clone(), start_time, BTreeSet::new(), &mut HashMap::new(), false, start_pos, start_time)
}

fn solve_recursive(tunnel_system: &TunnelSystem, pos: String, time: u64, open_set: BTreeSet<String>, state_cache: &mut HashMap<SolveState, u64>, last: bool, start_pos: String, start_time: u64) -> u64 {
    
    if time == 0 {
        if !last {
            let open_set_step = open_set.clone();
            let value = solve_recursive(tunnel_system, start_pos.clone(), start_time, open_set_step, state_cache, true, start_pos.clone(), start_time);
            value
        } else {
            0
        }
    } else {
        let current_solve_state = SolveState::new(pos.clone(), last, time, open_set.clone());
    
        if let Some(value) = state_cache.get(&current_solve_state) {
            return *value;
        }

        let current_valve = tunnel_system.valve(&pos).unwrap();
        let mut paths = Vec::new();

        if !open_set.contains(&pos) && current_valve.flow_rate > 0 {
            let mut open_set_step = open_set.clone();
            open_set_step.insert(pos.clone());
            paths.push(current_valve.flow_rate * (time - 1) + solve_recursive(tunnel_system, pos.clone(), time - 1, open_set_step, state_cache, last, start_pos.clone(), start_time));            
        }
        
        for connection in &current_valve.connections {
            paths.push(solve_recursive(tunnel_system, connection.clone(), time - 1, open_set.clone(), state_cache, last, start_pos.clone(), start_time));
        }
        
        paths.sort();
        paths.reverse();
        
        state_cache.insert(current_solve_state, paths[0]);

        paths[0]
    }
}