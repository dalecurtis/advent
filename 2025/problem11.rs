extern crate pathfinding;
extern crate rustc_hash;

use pathfinding::prelude;
use rustc_hash::FxHashMap;
use std::collections::HashMap;
use std::fs::File;
use std::hash::Hash;
use std::io::{BufRead, BufReader};

#[derive(Debug, Default, Clone, Hash, Eq, PartialEq)]
struct Edge {
    start: String,
    end: String,
}

fn successors(e: &Edge, mapping: &HashMap<String, Vec<Edge>>) -> Vec<Edge> {
    return match mapping.get(&e.end) {
        Some(v) => v.to_vec(),
        None => Vec::new(),
    };
}

// Modified impl of prelude::cached_count_paths
fn cached_count_paths2<FN, IN, FS>(
    start: Edge,
    saw_dac_fft: (bool, bool),
    successors: &mut FN,
    success: &mut FS,
    cache: &mut FxHashMap<(Edge, (bool, bool)), usize>,
) -> usize
where
    Edge: Eq + Hash,
    FN: FnMut(&Edge) -> IN,
    IN: IntoIterator<Item = Edge>,
    FS: FnMut(&Edge) -> bool,
{
    if let Some(&n) = cache.get(&(start.clone(), saw_dac_fft)) {
        return n;
    }

    let mut new_saw_dac_fft = saw_dac_fft;
    let count = if success(&start) {
        if new_saw_dac_fft.0 && new_saw_dac_fft.1 {
            1
        } else {
            0
        }
    } else {
        new_saw_dac_fft = saw_dac_fft;
        new_saw_dac_fft.0 |= start.start == "dac" || start.end == "dac";
        new_saw_dac_fft.1 |= start.start == "fft" || start.end == "fft";
        successors(&start)
            .into_iter()
            .map(|successor| {
                cached_count_paths2(successor, new_saw_dac_fft, successors, success, cache)
            })
            .sum()
    };

    cache.insert((start, new_saw_dac_fft), count);

    count
}

fn count_paths2<FN, IN, FS>(start: Edge, mut successors: FN, mut success: FS) -> usize
where
    Edge: Eq + Hash,
    FN: FnMut(&Edge) -> IN,
    IN: IntoIterator<Item = Edge>,
    FS: FnMut(&Edge) -> bool,
{
    cached_count_paths2(
        start,
        (false, false),
        &mut successors,
        &mut success,
        &mut FxHashMap::default(),
    )
}

fn load_mapping(fname: &str) -> HashMap<String, Vec<Edge>> {
    let file = File::open(fname).expect("Unable to open file");
    let reader = BufReader::new(file);

    let mut mapping: HashMap<String, Vec<Edge>> = HashMap::new();
    for line in reader.lines() {
        let line = line.expect("Unable to read line");

        let mut split = line.split(':');
        let p0 = split.next().unwrap();

        let edges = split
            .next()
            .unwrap()
            .split_whitespace()
            .map(|p1| Edge {
                start: p0.to_string(),
                end: p1.to_string(),
            })
            .collect();

        mapping.insert(p0.to_string(), edges);
    }

    return mapping;
}

fn main() {
    let mapping = load_mapping("input11.txt");
    println!("mapping.len={}", mapping.len());

    let start = "you";
    let end = "out";

    let mut part1_count = 0;
    for v in mapping.get(start).unwrap() {
        part1_count +=
            prelude::count_paths(v.clone(), |p| successors(p, &mapping), |p| *p.end == *end);
    }

    let mapping2 = load_mapping("input11.txt");
    println!("mapping2.len={}", mapping2.len());

    let start2 = "svr";
    let mut part2_count = 0;
    for v in mapping2.get(start2).unwrap() {
        part2_count += count_paths2(v.clone(), |p| successors(p, &mapping2), |p| *p.end == *end);
    }

    println!("part1 count: {}, part2 count: {}", part1_count, part2_count);
}
