use std::{collections::{HashMap, HashSet}, str::FromStr};

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

pub fn solve(tunnel_system: &TunnelSystem, start_time: u64, start_pos: String) -> u64 {
    solve_recursive(tunnel_system, start_pos, start_time, HashSet::new(), HashMap::new())
}

fn solve_recursive(tunnel_system: &TunnelSystem, pos: String, time: u64, open_set: HashSet<String>, state_cache: HashMap<(String, u64, HashSet<String>), u64>) -> u64 {
    
    if let value = state_cache.get((pos, time, open_set)) {
        return value;
    }

    if time > 1 {
        let current_valve = tunnel_system.valve(&pos).unwrap();
        let mut paths = Vec::new();

        if !open_set.contains(&pos) && current_valve.flow_rate > 0 {
            let mut open_set_step = open_set.clone();
            open_set_step.insert(pos.clone());
            paths.push(current_valve.flow_rate * (time - 1) + solve_recursive(tunnel_system, pos.clone(), time - 1, open_set_step));            
        }
        
        for connection in &current_valve.connections {
            paths.push(solve_recursive(tunnel_system, connection.clone(), time - 1, open_set.clone()));
        }
        
        paths.sort();
        paths.reverse();

        paths[0]

    } else {
        0
    }
}