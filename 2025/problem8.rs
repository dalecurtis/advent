extern crate itertools;
extern crate ordered_float;

use itertools::Itertools;
use ordered_float::OrderedFloat;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Default, Clone, Hash, Eq, PartialEq)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

#[derive(Debug, Default, Clone, Hash, Eq, PartialEq)]
struct Edge {
    cost: OrderedFloat<f64>,
    a: Point,
    b: Point,
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice the flip here: other.cost.cmp(&self.cost)
        // This makes the BinaryHeap behave as a min-heap based on cost.
        other.cost.cmp(&self.cost)
    }
}

// PartialOrd is required for Ord
impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn distance(a: &Point, b: &Point) -> f64 {
    if a == b {
        todo!("unexpected combo");
    }
    return (((a.x - b.x).pow(2) + (a.y - b.y).pow(2) + (a.z - b.z).pow(2)) as f64).sqrt();
}

fn process_edge(circuits: &mut Vec<HashSet<Point>>, edge: &Edge) {
    //   If only a in circuits, add b
    //   If only b in circuits, add a
    //   If a and b in circuits, merge circuit.
    //   If none, create new circuit
    let mut c_a: Option<usize> = None;
    let mut c_b: Option<usize> = None;
    for (i, c) in circuits.into_iter().enumerate() {
        if c.contains(&edge.a) {
            if c_a.is_some() {
                todo!("unexpected a");
            }
            c_a = Some(i);
        }
        if c.contains(&edge.b) {
            if c_b.is_some() {
                todo!("unexpected b");
            }
            c_b = Some(i);
        }
    }

    if c_a.is_none() && c_b.is_none() {
        // Circuit not found. Create a new on
        let mut new_circuit = HashSet::new();
        new_circuit.insert(edge.a.clone());
        new_circuit.insert(edge.b.clone());
        circuits.push(new_circuit);
        return;
    }

    // Both circuits exist, so merge.
    if c_a.is_some() && c_b.is_some() {
        let mut a = c_a.unwrap();
        let mut b = c_b.unwrap();
        if a == b {
            return;
        }
        if a > b {
            (a, b) = (b, a);
        }
        {
            let (left, right) = circuits.split_at_mut(b);
            left[a].extend(right[0].drain());
        }
        circuits.swap_remove(b);
        return;
    }

    if c_a.is_some() {
        let a = c_a.unwrap();
        circuits[a].insert(edge.b.clone());
        return;
    }

    let b = c_b.unwrap();
    circuits[b].insert(edge.a.clone());
}

fn main() {
    let file = File::open("input8.txt").expect("Unable to open file");
    let reader = BufReader::new(file);

    let mut junctions = Vec::new();
    for line in reader.lines() {
        let line = line.expect("Unable to read line");
        let mut int_range = line.split(',').map(|s| s.parse::<i64>().unwrap());

        junctions.push(Point {
            x: int_range.next().unwrap(),
            y: int_range.next().unwrap(),
            z: int_range.next().unwrap(),
        });
    }

    println!("junctions.len={}", junctions.len());

    let mut min_heap = BinaryHeap::new();
    for combo in junctions.iter().combinations(2) {
        let (p1, p2) = (combo[0].clone(), combo[1].clone());
        let d = distance(&p1, &p2);
        min_heap.push(Edge {
            cost: OrderedFloat(d),
            a: p1,
            b: p2,
        });
    }

    println!("min_heap.len()={}", min_heap.len());

    let mut circuits: Vec<HashSet<Point>> = Vec::new();
    for _ in 0..1000 {
        let edge = min_heap.pop().unwrap();
        process_edge(&mut circuits, &edge);
    }

    circuits.sort_by(|a, b| b.len().cmp(&a.len()));

    let joined = circuits.iter().fold(0, |acc, c| acc + c.len());
    println!(
        "circuit len={}, junction_count={}, total={}, part1_calc={}",
        circuits.len(),
        joined,
        junctions.len() - joined + circuits.len(),
        circuits[0].len() * circuits[1].len() * circuits[2].len()
    );
}
