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
type ClosedValveBitmap = Bitmap<MAX_VALVES>;

struct Solver {
    valves: Valves,
    travel_times: TravelTimes,
}

const VALVE_OPEN_TIME: Time = 1;
type Pressure = usize;
type BestDecisions = HashMap<ClosedValveBitmap, Pressure>;

impl Solver {
    fn maximize_pressure_release(
        &self,
        from: ValveId,
        time_left: Time,
        closed_valves: ClosedValveBitmap,
        best_decisions: &mut BestDecisions,
        current_pressure_release: Pressure,
    ) -> Pressure {
        best_decisions.insert(
            closed_valves,
            *best_decisions
                .get(&closed_valves)
                .unwrap_or(&0)
                .max(&current_pressure_release),
        );
        closed_valves
            .into_iter()
            .filter_map(|to| {
                let valve_time = self.travel_times[from][to] + VALVE_OPEN_TIME;
                if valve_time <= time_left {
                    let pressure_release = current_pressure_release
                        + self.valves[&to].flow_rate * (time_left - valve_time);
                    let mut valves = closed_valves;
                    valves.set(to, false);
                    Some(self.maximize_pressure_release(
                        to,
                        time_left - valve_time,
                        valves,
                        best_decisions,
                        pressure_release,
                    ))
                } else {
                    None
                }
            })
            .max()
            .unwrap_or(current_pressure_release)
    }

    fn new(valves: Valves, travel_times: TravelTimes) -> Self {
        Self {
            valves,
            travel_times,
        }
    }
}

fn all_closed_valves(valves: &Valves, start_valve_id: ValveId) -> ClosedValveBitmap {
    // ignore valves with zero flow rate
    let mut valve_bitmap = ClosedValveBitmap::new();
    for (id, _) in valves
        .iter()
        .filter(|(id, valve)| valve.flow_rate > 0 && **id != start_valve_id)
    {
        valve_bitmap.set(*id, true);
    }
    valve_bitmap
}

fn find_best_pair_result(
    elf_best_decisions: BestDecisions,
    elephant_best_decisions: BestDecisions,
    initial_valves: ClosedValveBitmap,
) -> Flow {
    elf_best_decisions
        .iter()
        .fold(0, |max_pressure_release, (elf_mask, elf_pressure)| {
            elephant_best_decisions.iter().fold(
                max_pressure_release,
                |max_pressure_release, (elephant_mask, elephant_pressure)| {
                    let elf_mask = *elf_mask.as_value();
                    let elephant_mask = *elephant_mask.as_value();
                    if (!elf_mask) & (!elephant_mask) & initial_valves.as_value() == 0 {
                        max_pressure_release.max(elf_pressure + elephant_pressure)
                    } else {
                        max_pressure_release
                    }
                },
            )
        })
}

fn calculate_solution(scan_output: &str) -> (Flow, Flow) {
    let (valves, start_valve_id) = parse_valves(scan_output);
    let travel_times = floyd_warshall(&valves);
    let valves_count = valves.len();
    assert!(valves_count <= MAX_VALVES);
    let closed_valves = all_closed_valves(&valves, start_valve_id);
    let solver = Solver::new(valves, travel_times);
    let mut best_decisions = BestDecisions::new();
    let best_result =
        solver.maximize_pressure_release(start_valve_id, 30, closed_valves, &mut best_decisions, 0);

    let mut elf_best_decisions = BestDecisions::new();
    let _ = solver.maximize_pressure_release(
        start_valve_id,
        26,
        closed_valves,
        &mut elf_best_decisions,
        0,
    );
    let elephant_best_decisions = elf_best_decisions.clone();
    let best_result_with_elephant =
        find_best_pair_result(elf_best_decisions, elephant_best_decisions, closed_valves);
    (best_result, best_result_with_elephant)
}

fn main() {
    println!("Solution {:?}", calculate_solution(data::SCAN_OUTPUT));
}
