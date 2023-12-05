mod data;

use regex::RegexBuilder;

type Number = usize;

const ORE_INDEX: usize = 0;
const CLAY_INDEX: usize = 1;
const OBSIDIAN_INDEX: usize = 2;
const GEODE_INDEX: usize = 3;
const MATERIAL_COUNT: usize = 4;
type Robots = [Number; MATERIAL_COUNT];
type Cost = [Number; MATERIAL_COUNT];
type Inventory = [Number; MATERIAL_COUNT];

struct Blueprint {
    number: Number,
    costs: [Cost; MATERIAL_COUNT],
}

impl Blueprint {
    fn material_demand(&self, index: usize) -> Number {
        let demand = self.costs.iter().map(|c| c[index]).max().unwrap();
        if demand > 0 {
            demand
        } else {
            usize::MAX
        }
    }
}

fn parse_blueprints(blueprints: &str) -> Vec<Blueprint> {
    let regex =
        RegexBuilder::new(
"Blueprint (\\d+): Each ore robot costs (\\d+) ore. Each clay robot costs (\\d+) ore. Each obsidian robot costs (\\d+) ore and (\\d+) clay. Each geode robot costs (\\d+) ore and (\\d+) obsidian.")
            .multi_line(true)
            .build()
            .unwrap();
    regex
        .captures_iter(blueprints)
        .map(|captures| Blueprint {
            number: captures[1].parse().unwrap(),
            costs: [
                [captures[2].parse().unwrap(), 0, 0, 0],
                [captures[3].parse().unwrap(), 0, 0, 0],
                [
                    captures[4].parse().unwrap(),
                    captures[5].parse().unwrap(),
                    0,
                    0,
                ],
                [
                    captures[6].parse().unwrap(),
                    0,
                    captures[7].parse().unwrap(),
                    0,
                ],
            ],
        })
        .collect()
}

#[derive(Default, Clone, Debug)]
struct State {
    minutes_left: Number,
    inventory: Inventory,
    robots: Robots,
}

impl State {
    fn mine(&self, minutes: Number) -> Self {
        let mut new = self.clone();
        new.minutes_left = new
            .minutes_left
            .checked_sub(minutes)
            .or_else(|| {
                println!("minutes {minutes}, left {}", self.minutes_left);
                None
            })
            .unwrap();
        for i in 0..MATERIAL_COUNT {
            new.inventory[i] += new.robots[i] * minutes;
        }
        new
    }
    fn material_name(index: usize) -> &'static str {
        match index {
            0 => "ore",
            1 => "clay",
            2 => "obsidian",
            3 => "geode",
            _ => unreachable!(),
        }
    }

    fn try_make_robot(&self, index: usize, blueprint: &Blueprint) -> Option<Self> {
        let cost = blueprint.costs[index];
        let mut mining_time = 1;
        for i in 0..MATERIAL_COUNT {
            // have enough material or can mine it (minus one to account for building a robot with current inventory)
            if cost[i] > 0 && self.inventory[i] + (self.minutes_left - 1) * self.robots[i] < cost[i]
            {
                return None;
            }
            if self.inventory[i] < cost[i] {
                let need = cost[i] - self.inventory[i];
                let mut time = 1 + need / self.robots[i];
                if need % self.robots[i] > 0 {
                    time += 1;
                }
                mining_time = mining_time.max(time);
            }
        }
        let mut new = self.mine(mining_time);
        for i in 0..MATERIAL_COUNT {
            new.inventory[i] = new.inventory[i]
                .checked_sub(cost[i])
                .or_else(|| {
                    println!(
                        "{}: inv {} -> {}, need {}, {} robots, spend time {mining_time}",
                        Self::material_name(i),
                        self.inventory[i],
                        new.inventory[i],
                        cost[i],
                        self.robots[i],
                    );
                    None
                })
                .unwrap();
        }
        new.robots[index] += 1;
        Some(new)
    }
}

fn go_to_next_state(blueprint: &Blueprint, max_materials: &Cost, state: State) -> State {
    if state.minutes_left == 0 {
        return state;
    }

    let mut max = state.clone();
    let mut can_build_robots = false;
    for i in 0..MATERIAL_COUNT {
        // make robots only if their number is less than maximum demand of their produce per robot
        if state.robots[i] >= max_materials[i] {
            continue;
        }
        if let Some(new) = state.try_make_robot(i, blueprint) {
            can_build_robots = true;
            let result = go_to_next_state(blueprint, max_materials, new);
            if result.inventory[GEODE_INDEX] >= max.inventory[GEODE_INDEX] {
                max = result;
            }
        }
    }
    if !can_build_robots {
        max.inventory[GEODE_INDEX] =
            max.inventory[GEODE_INDEX].max(max.robots[GEODE_INDEX] * max.minutes_left);
    }

    max
}

fn max_open_geodes(blueprint: &Blueprint, time_left: Number) -> Number {
    let state = State {
        minutes_left: time_left,
        robots: [1, 0, 0, 0],
        ..Default::default()
    };
    let max_materials = [
        blueprint.material_demand(ORE_INDEX),
        blueprint.material_demand(CLAY_INDEX),
        blueprint.material_demand(OBSIDIAN_INDEX),
        blueprint.material_demand(GEODE_INDEX),
    ];
    let result = go_to_next_state(blueprint, &max_materials, state);
    result.inventory[GEODE_INDEX]
}

fn part_1_solution(blueprints: &[Blueprint]) -> Number {
    blueprints
        .iter()
        .map(|b| {
            let best = max_open_geodes(b, 24);
            println!("BP #{} => {}", b.number, best);
            best * b.number
        })
        .sum()
}

fn part_2_solution(blueprints: &[Blueprint]) -> Number {
    blueprints
        .iter()
        .take(3)
        .map(|b| {
            let best = max_open_geodes(b, 32);
            println!("BP2 #{} => {}", b.number, best);
            best
        })
        .product()
}

fn calculate_solution(blueprints: &str) -> (Number, Number) {
    let blueprints = parse_blueprints(blueprints);
    (part_1_solution(&blueprints), part_2_solution(&blueprints))
}

fn main() {
    println!("Solution {:?}", calculate_solution(data::INPUT));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn reference_case_part_1() {
        let blueprints = parse_blueprints(data::TEST_INPUT);
        assert_eq!(part_1_solution(&blueprints), 33);
    }

    #[test]
    fn reference_case_part_2() {
        let blueprints = parse_blueprints(data::TEST_INPUT);
        assert_eq!(part_2_solution(&blueprints), 56 * 62);
    }
}
