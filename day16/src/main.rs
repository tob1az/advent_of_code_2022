mod data;

use bitmaps::Bitmap;
use itertools::iproduct;
use regex::RegexBuilder;
use std::collections::HashMap;

type ValveId = usize;

#[derive(Debug)]
struct Valve {
    flow_rate: usize,
    connected_valves: Vec<ValveId>,
}

type Valves = HashMap<ValveId, Valve>;
const START_VALVE: &str = "AA";

fn parse_valves(scan_output: &str) -> (Valves, ValveId) {
    let mut name_to_id = HashMap::new();

    let regex =
        RegexBuilder::new("Valve (.+) has flow rate=(\\d+); tunnels? leads? to valves? (.+)")
            .multi_line(true)
            .build()
            .unwrap();
    let valves = regex
        .captures_iter(scan_output)
        .map(|captures| {
            let next_id = name_to_id.len();
            let id = *name_to_id
                .entry(captures[1].to_owned())
                .or_insert_with(|| next_id);
            let flow_rate = captures[2].parse().unwrap();
            let connected_valves = captures[3]
                .split(", ")
                .map(|name| {
                    let next_id = name_to_id.len();
                    *name_to_id.entry(name.to_owned()).or_insert_with(|| next_id)
                })
                .collect();
            (
                id,
                Valve {
                    flow_rate,
                    connected_valves,
                },
            )
        })
        .collect();
    (valves, name_to_id[START_VALVE])
}

type Time = usize;
type TravelTimes = Vec<Vec<Time>>;
const INF: Time = 100000;
const TUNNEL_DISTANCE: Time = 1;

// find time needed to travel between each pair of valves
fn floyd_warshall(valves: &Valves) -> TravelTimes {
    let size = valves.len();
    let mut times = vec![vec![INF; size]; size];
    for (id, v) in valves {
        times[*id][*id] = 0;
        for &c in &v.connected_valves {
            times[*id][c] = TUNNEL_DISTANCE;
            times[c][*id] = TUNNEL_DISTANCE;
        }
    }

    for (mid, from, to) in iproduct!(valves, valves, valves) {
        times[*from.0][*to.0] =
            times[*from.0][*to.0].min(times[*from.0][*mid.0] + times[*mid.0][*to.0]);
    }
    times
}

type Flow = usize;
const MAX_VALVES: usize = 64;
type ClosedValves = Bitmap<MAX_VALVES>;

struct Solver {
    valves: Valves,
    travel_times: TravelTimes,
}

const VALVE_OPEN_TIME: Time = 1;

impl Solver {
    fn maximize_flow(
        &mut self,
        from: ValveId,
        time_left: Time,
        closed_valves: ClosedValves,
    ) -> Flow {
        closed_valves
            .clone()
            .into_iter()
            .filter_map(|to| {
                let valve_time = self.travel_times[from][to] + VALVE_OPEN_TIME;
                if valve_time <= time_left {
                    let flow = self.valves[&to].flow_rate * (time_left - valve_time);
                    let mut closed_valves = closed_valves;
                    closed_valves.set(to, false);
                    Some(flow + self.maximize_flow(to, time_left - valve_time, closed_valves))
                } else {
                    None
                }
            })
            .max()
            .unwrap_or(0)
    }

    fn solve(&mut self, time_left: Time, start_valve_id: ValveId) -> Flow {
        assert!(self.valves.len() <= MAX_VALVES);
        // ignore valves with zero flow rate
        let mut closed_valves: Bitmap<MAX_VALVES> = Bitmap::new();
        for (id, _) in self
            .valves
            .iter()
            .filter(|(id, valve)| **id != start_valve_id && valve.flow_rate > 0)
        {
            closed_valves.set(*id, true);
        }
        self.maximize_flow(start_valve_id, time_left, closed_valves)
    }

    fn new(valves: Valves, travel_times: TravelTimes) -> Self {
        Self {
            valves,
            travel_times,
        }
    }
}

fn calculate_solution(scan_output: &str) -> Flow {
    let (valves, start_valve_id) = parse_valves(scan_output);
    let travel_times = floyd_warshall(&valves);
    Solver::new(valves, travel_times).solve(30, start_valve_id)
}

fn main() {
    println!("Solution {:?}", calculate_solution(data::SCAN_OUTPUT));
}
