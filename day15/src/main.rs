mod data;
mod interval;

use interval::Interval;
use regex::RegexBuilder;
use itertools::Itertools;

#[derive(Debug)]
struct Sensor {
    x: i64,
    y: i64,
    manhattan_radius: i64,
}

struct Report {
    sensors: Vec<Sensor>,
}

fn parse_report(report: &str) -> Report {
    let regex = RegexBuilder::new(
        "Sensor at x=(-*\\d+), y=(-*\\d+): closest beacon is at x=(-*\\d+), y=(-*\\d+)",
    )
    .multi_line(true)
    .build()
    .unwrap();
    Report {
        sensors: regex
            .captures_iter(report)
            .map(|capture| {
                let x = capture[1].parse::<i64>().unwrap();
                let y = capture[2].parse::<i64>().unwrap();
                let beacon_x = capture[3].parse::<i64>().unwrap();
                let beacon_y = capture[4].parse::<i64>().unwrap();
                let manhattan_radius = manhattan_distance(beacon_x, beacon_y, x, y);
                Sensor {
                    x,
                    y,
                    manhattan_radius,
                }
            })
            .collect(),
    }
}

fn calculate_solution(report: &str, inspected_row_number: usize) -> (usize, i64) {
    let report = parse_report(report);
    let no_beacon_positions = find_no_beacon_intervals_in_row(&report, inspected_row_number)
        .iter()
        .map(Interval::len)
        .sum();
    let (beacon_x, beacon_y) = deduce_beacon_coordinates(&report).unwrap();
    let tuning_frequency = beacon_x.checked_mul(4000000).unwrap() + beacon_y;
    (no_beacon_positions, tuning_frequency)
}

fn manhattan_distance(x0: i64, y0: i64, x1: i64, y1: i64) -> i64 {
    (x1 - x0).abs() + (y1 - y0).abs()
}

fn find_no_beacon_intervals_in_row(report: &Report, inspected_row_number: usize) -> Vec<Interval> {
    let mut no_beacon_intervals = report
        .sensors
        .iter()
        .filter_map(|sensor| {
            let distance_to_row = (sensor.y - inspected_row_number as i64).abs();
            if distance_to_row <= sensor.manhattan_radius {
                Some((sensor, distance_to_row))
            } else {
                None
            }
        })
        .map(|(sensor, distance_to_row)| {
            let half_chord = sensor.manhattan_radius - distance_to_row;
            let from = sensor.x - half_chord;
            let to = sensor.x + half_chord;
            Interval::new(from, to)
        })
        .collect::<Vec<_>>();

    if no_beacon_intervals.len() > 1 {
        let mut keep_merging: bool = true;
        while keep_merging {
            keep_merging = false;
            no_beacon_intervals.retain(|interval| interval.len() > 0);
            no_beacon_intervals.sort_by_key(|interval| interval.from());
            for i in 1..no_beacon_intervals.len() {
                if let Some(merged) = no_beacon_intervals[i - 1].merge(&no_beacon_intervals[i]) {
                    keep_merging = true;
                    no_beacon_intervals[i - 1] = Interval::new(0, 0);
                    no_beacon_intervals[i] = merged;
                }
            }
        }
    }
    no_beacon_intervals
}

fn deduce_beacon_coordinates(report: &Report) -> Option<(i64, i64)> {
    for row in 0..4000000 {
        for (a, b) in find_no_beacon_intervals_in_row(report, row).iter().tuple_windows() {
            if b.from() > a.to() {
                assert!(b.from() - a.to() != 1);
                return Some((a.to() + 1, row as i64));
            }
        }
    }
    None
}

fn main() {
    println!(
        "Solution {:?}",
        calculate_solution(data::REPORT, data::INSPECTED_ROW_NUMBER)
    );
}
