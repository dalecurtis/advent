extern crate geo;
extern crate geo_types;
extern crate itertools;

use geo::Within;
use geo_types::{coord, Coord, LineString, Polygon, Rect};
use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Default, Clone, Hash, Eq, PartialEq)]
struct Edge {
    cost: i64,
    a: Coord<i64>,
    b: Coord<i64>,
}

fn area(a: &Coord<i64>, b: &Coord<i64>) -> i64 {
    if a == b {
        todo!("unexpected combo");
    }
    // ARGGGGGGH: I originally wrote this as |a.y - b.y + 1| * |a.x - b.x + 1|
    // and couldn't figure out why my solution didn't work.
    return ((a.y - b.y).abs() + 1) * ((a.x - b.x).abs() + 1);
}

fn main() {
    let file = File::open("input9.txt").expect("Unable to open file");
    let reader = BufReader::new(file);

    let mut red_tiles = Vec::new();
    for line in reader.lines() {
        let line = line.expect("Unable to read line");
        let mut int_range = line.split(',').map(|s| s.parse::<i64>().unwrap());

        red_tiles.push(coord! {
            x: int_range.next().unwrap(),
            y: int_range.next().unwrap(),
        });
    }

    println!("red_tiles.len={}", red_tiles.len());

    let mut outline = Vec::new();
    for p in &red_tiles {
        outline.push(coord! { x: p.x as f64, y: p.y as f64});
    }
    outline.push(coord! { x: red_tiles[0].x as f64, y: red_tiles[0].y as f64});

    let n = LineString::new(outline.clone());
    if !n.is_closed() {
        todo!("this shouldn't happen");
    }
    let xmas_blob = Polygon::new(n, vec![]);

    let mut max_total = Edge::default();
    let mut max_contained = Edge::default();
    for combo in red_tiles.iter().combinations(2) {
        let (p1, p2) = (combo[0].clone(), combo[1].clone());
        let d = area(&p1, &p2);
        let r = Rect::new(
            coord! {x: p1.x as f64, y: p1.y as f64},
            coord! {x: p2.x as f64, y: p2.y as f64},
        );
        if d > max_total.cost {
            max_total = Edge {
                cost: d,
                a: p1,
                b: p2,
            };
        }
        if d > max_contained.cost && r.is_within(&xmas_blob) {
            max_contained = Edge {
                cost: d,
                a: p1,
                b: p2,
            };
        }
    }

    // Since I messed up the area() calculation I eventually gave up on the
    // programmatic solution and plotted shape to canvas and just found the
    // answer by manual inspection...
    println!(
        "human part2={}",
        area(&coord! {x: 94699, y: 50401}, &coord! {x: 5106, y: 67466})
    );

    println!("part1={:?}", max_total);
    println!("part2={:?}", max_contained);
}
