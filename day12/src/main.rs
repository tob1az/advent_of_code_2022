mod data;

use petgraph::algo::astar;
use petgraph::prelude::*;

type Height = u8;
const TOP_ELEVATION: Height = b'z';
const BOTTOM_ELEVATION: Height = b'a';
#[derive(Clone)]
enum Point {
    AtHeight(Height),
    Top,
    Start,
}

impl Point {
    fn height(&self) -> Height {
        match self {
            Self::AtHeight(h) => *h,
            Self::Top => TOP_ELEVATION - BOTTOM_ELEVATION,
            Self::Start => 0,
        }
    }
}

type X = usize;
type Y = usize;
type Cost = u32;
type HeightGraph = Graph<(Point, X, Y), Cost>;

fn update_graph_edges(
    graph: &mut HeightGraph,
    point_index: NodeIndex,
    point: &Point,
    another_index: NodeIndex,
    another_point: &Point,
) {
    let height = point.height();
    let another_height = another_point.height();
    if another_height == height {
        graph.add_edge(another_index, point_index, 10);
        graph.add_edge(point_index, another_index, 10);
    } else if height > another_height {
        graph.add_edge(point_index, another_index, 100);
        if another_height + 1 == height {
            graph.add_edge(another_index, point_index, 1);
        }
    } else if another_height > height {
        graph.add_edge(another_index, point_index, 100);
        if height + 1 == another_height {
            graph.add_edge(point_index, another_index, 1);
        }
    }
}

fn calculate_solution(height_map: &str) -> usize {
    let heights = height_map
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| {
                    if c == 'E' {
                        Point::Top
                    } else if c == 'S' {
                        Point::Start
                    } else {
                        assert!(c as u32 <= TOP_ELEVATION as u32);
                        Point::AtHeight(c as u8 - BOTTOM_ELEVATION)
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut graph = Graph::new();
    let mut upper_row = Vec::new();
    let mut start_index = None;
    for (j, row) in heights.iter().enumerate() {
        let mut previous: Option<(NodeIndex, Point)> = None;
        let mut current_row = Vec::new();
        for (i, point) in row.iter().enumerate() {
            let point_index = graph.add_node((point.clone(), i, j));
            if matches!(point, Point::Start) {
                start_index = Some(point_index);
            }
            if let Some((previous_index, previous_point)) = previous {
                update_graph_edges(&mut graph, point_index, point, previous_index, &previous_point);
            }
            if upper_row.len() > 0 {
                let (upper_index, upper_point) = &upper_row[i];
                update_graph_edges(&mut graph, point_index, point, *upper_index, upper_point);
            }
            current_row.push((point_index.clone(), point.clone()));
            previous = Some((point_index, point.clone()));
        }
        upper_row = current_row;
    }
    let result = astar(
        &graph,
        start_index.unwrap(),
        |n| matches!(graph[n].0, Point::Top),
        |e| *e.weight(),
        |_| 0,
    );
    let shortest_path = result.unwrap().1;
    // number of edges
    shortest_path.len() - 1
}

fn main() {
    println!("Solution {:?}", calculate_solution(data::HEIGHT_MAP));
}
