extern crate pathfinding;

use pathfinding::prelude;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Default, Clone, Hash, Eq, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

const BEAM_DIR: (i32, i32) = (0, 1);
const BEAM_LEFT_DIR: (i32, i32) = (-1, 1);
const BEAM_RIGHT_DIR: (i32, i32) = (1, 1);

fn move_point(p: &Point, graph: &Vec<Vec<char>>, dir: (i32, i32)) -> Option<Point> {
    let npx = p.x as i32 + dir.0;
    let npy = p.y as i32 + dir.1;
    if npx < 0 || npy < 0 {
        return None;
    }
    if npx as usize >= graph[0].len() || npy as usize >= graph.len() {
        return None;
    }
    return Some(Point {
        x: npx as usize,
        y: npy as usize,
    });
}

impl Point {
    fn successors(&self, graph: &Vec<Vec<char>>, splitters: &HashSet<Self>) -> Vec<Self> {
        let next_p = move_point(self, graph, BEAM_DIR);
        if next_p.is_none() {
            return Vec::new();
        }
        let mut successors = Vec::new();
        if next_p.is_some() {
            let dp = next_p.unwrap();
            if splitters.contains(&dp) {
                let l = move_point(&dp, graph, BEAM_LEFT_DIR);
                let r = move_point(&dp, graph, BEAM_RIGHT_DIR);
                if l.is_some() {
                    successors.push(l.unwrap());
                }
                if r.is_some() {
                    successors.push(r.unwrap());
                }
            } else {
                successors.push(dp);
            }
        }
        return successors;
    }
}

fn print_manifold(manifold: &Vec<Vec<char>>) {
    for y in 0..manifold.len() {
        for x in 0..manifold[y].len() {
            print!("{}", manifold[y][x])
        }
        println!();
    }
    println!();
}

fn main() {
    let file = File::open("input7.txt").expect("Unable to open file");
    let reader = BufReader::new(file);

    let mut start = Point { x: 0, y: 0 };
    let mut splitters = HashSet::new();
    let mut manifold: Vec<Vec<char>> = Vec::new();
    let mut y_pos = 0;
    for line in reader.lines() {
        let line = line.expect("Unable to read line");
        for (x, c) in line.chars().enumerate() {
            match c {
                '.' => {}
                'S' => {
                    start = Point { x: x, y: y_pos };
                }
                '^' => {
                    splitters.insert(Point { x: x, y: y_pos });
                }
                _ => todo!("bad input"),
            }
        }

        manifold.push(line.chars().collect());
        y_pos += 1;
    }

    print_manifold(&manifold);
    println!("start={:?}", start);
    println!("splitters.len={}", splitters.len());

    let mut part1_count = 0;
    let mut part2_count = 0;
    for splitter in &splitters {
        let just_before_splitter = move_point(&splitter, &manifold, (0, -1)).unwrap();
        let path_count = prelude::count_paths(
            start.clone(),
            |p| p.successors(&manifold, &splitters),
            |p| *p == just_before_splitter,
        );
        part1_count += if path_count > 0 { 1 } else { 0 };
        part2_count += path_count;
    }

    // No idea why the part2 count ends up being off by one here...
    println!(
        "part1 count: {}, part2 count: {}",
        part1_count,
        part2_count + 1
    );
}
