use std::{collections::HashMap, str::FromStr};

pub struct Valve {
    flow_rate: u64,
    connections: Vec<String>,
}

impl Valve {
    pub fn new(flow_rate: u64, connections: Vec<String>) -> Self {
        Self { flow_rate, connections }
    }
    
    pub fn flow_rate(&self) -> u64 {
        self.flow_rate
    }
    
    pub fn connections(&self) -> Vec<String> {
        self.connections.clone()
    }
}

pub struct TunnelSystem {
    valves: HashMap<String, Valve>,
    active_valves: HashMap<String, u64>,
    current_valve: String,
    time_left: u64,
}

impl TunnelSystem {
    pub fn valve_list(&self) -> Vec<&String> {
        self.valves.keys().collect()
    }
    
    pub fn valve(&self, key: &str) -> Option<&Valve> {
        self.valves.get(key)
    }
    
    pub fn is_active(&self, key: &str) -> bool {
        self.active_valves.contains_key(key)
    }
    
    pub fn move_to_valve(&mut self, key: &str) -> bool {
        let current_valve = self.valves.get(&self.current_valve).unwrap();
        if current_valve.connections.contains(&key.to_owned()) {
            self.time_left -= 1;
            self.current_valve = key.to_owned();
            true
        } else {
            false
        }
    }
    
    pub fn activate_valve(&mut self) -> bool {
        if !self.active_valves.contains_key(&self.current_valve) {
            let current_valve = self.valves.get(&self.current_valve).unwrap();
            self.time_left -= 1;
            self.active_valves.insert(self.current_valve.clone(), current_valve.flow_rate * self.time_left);
            true
        } else {
            false
        }
    }
    
    pub fn total_pressure(&self) -> u64 {
        self.active_valves.iter().map(|(_, value)| value).sum()
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
        
        Ok(TunnelSystem { valves, active_valves: HashMap::new(), current_valve: "AA".to_owned(), time_left: 30 })
    }
}