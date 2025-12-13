extern crate itertools;

use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Default, Clone, Hash, Eq, PartialEq)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Debug, Default, Clone, Hash, Eq, PartialEq)]
struct Edge {
    cost: i64,
    a: Point,
    b: Point,
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

// PartialOrd is required for Ord
impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.cmp(self))
    }
}

fn area(a: &Point, b: &Point) -> i64 {
    if a == b {
        todo!("unexpected combo");
    }
    return (a.y - b.y + 1).abs() * (a.x - b.x + 1).abs();
}

fn main() {
    let file = File::open("input9.txt").expect("Unable to open file");
    let reader = BufReader::new(file);

    let mut red_tiles = Vec::new();
    for line in reader.lines() {
        let line = line.expect("Unable to read line");
        let mut int_range = line.split(',').map(|s| s.parse::<i64>().unwrap());

        red_tiles.push(Point {
            x: int_range.next().unwrap(),
            y: int_range.next().unwrap(),
        });
    }

    println!("red_tiles.len={}", red_tiles.len());

    let mut max_heap = BinaryHeap::new();
    for combo in red_tiles.iter().combinations(2) {
        let (p1, p2) = (combo[0].clone(), combo[1].clone());
        let d = area(&p1, &p2);
        max_heap.push(Edge {
            cost: d,
            a: p1,
            b: p2,
        });
    }

    println!("max_heap.len()={}", max_heap.len());

    let edge = max_heap.pop().unwrap();
    println!("{:?}", edge);
}
