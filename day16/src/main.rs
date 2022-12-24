mod puzzle;

use puzzle::*;

fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let input = day_util::read_input_safe("day16");
    let tunnel_system: TunnelSystem = input.parse().unwrap();    
    let pressure = solve(&tunnel_system, 30, "AA".to_owned());

    println!("Pressure released: {pressure}");
}

fn part_two() {
    let input = day_util::read_input_safe("day16");
    let tunnel_system: TunnelSystem = input.parse().unwrap();    
    let pressure = solve_with_elephant(&tunnel_system, 26, "AA".to_owned());

    println!("Pressure released: {pressure}");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    const TEST_INPUT: &str = r#"Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II
"#;

    #[test]
    fn parse_test() {
        let tunnel_system: TunnelSystem = TEST_INPUT.parse().unwrap();
        
        assert_eq!(tunnel_system.valve_list().len(), 10);
        assert_eq!(tunnel_system.valve("AA").unwrap().flow_rate(), 0);
        assert_eq!(tunnel_system.valve("AA").unwrap().connections().len(), 3);
        assert_eq!(tunnel_system.valve("AA").unwrap().connections()[0], "DD");
    }
    
    #[test]
    fn part_one_test() {
        let tunnel_system: TunnelSystem = TEST_INPUT.parse().unwrap();    
        let pressure = solve(&tunnel_system, 30, "AA".to_owned());

        assert_eq!(pressure, 1651);
    }

    #[test]
    fn part_two_test() {
        let tunnel_system: TunnelSystem = TEST_INPUT.parse().unwrap();    
        let pressure = solve_with_elephant(&tunnel_system, 26, "AA".to_owned());

        assert_eq!(pressure, 1707);
    }
}
