use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Default, Clone, Hash, Eq, PartialEq)]
struct Point {
    // x, y position of a point.
    x: usize,
    y: usize,
}

#[derive(Debug, Default, Clone, Hash, Eq, PartialEq)]
struct Shape {
    // Unique character representing this shape for debugging.
    id: char,

    // All points making up the shape.
    coords: Vec<Point>,

    // Minimum number of black/white tiles needed to draw this piece.
    min_bw: usize,
}

// #[derive(Debug, Default, Clone, Hash, Eq, PartialEq)]
// struct ShapeVariants {
//     // Prebuilt directory of all rotations and flips for each shape.
//     // At most there should be 8 entries (4 rotations, 2 flips each).
//     variants: Vec<Shape>,
// }

#[derive(Debug, Default, Clone, Hash, Eq, PartialEq)]
struct Region {
    width: usize,
    height: usize,
    presents: Vec<usize>,

    // Available number of black/white cells.
    avail_black: usize,
    avail_white: usize,
}

// True is white, black is zero.
fn is_white(x: usize, y: usize) -> bool {
    return (x + y) % 2 == 1;
}

fn count_white(width: usize, height: usize) -> usize {
    return (width * height) - count_black(width, height);
}

fn count_black(width: usize, height: usize) -> usize {
    return (width * height + 1) / 2;
}

fn cheap_fit_test(region: &Region, shapes: &Vec<Shape>) -> Option<bool> {
    let area = region.width * region.height;

    let max_area: usize = region.presents.iter().sum::<usize>() * 9;
    if max_area <= area {
        return Some(true);
    }

    let mut min_area: usize = 0;
    let mut min_bw: usize = 0;
    for (i, p) in region.presents.iter().enumerate() {
        min_area += p * shapes[i].coords.len();
        min_bw += p * shapes[i].min_bw;
    }
    if min_area > area {
        println!("failed area...");
        return Some(false);
    }
    if min_bw > std::cmp::min(region.avail_black, region.avail_white) {
        println!("failed bw...");
        return Some(false);
    }

    return None;
}

fn main() {
    let file = File::open("input12.txt").expect("Unable to open file");
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    // Parse contents of the form:
    // n:
    // #.#
    // ###
    // ..#
    const NUM_PRESENTS: usize = 6;
    let mut presents: Vec<Shape> = Vec::new();
    for _ in 0..NUM_PRESENTS {
        let id = lines
            .next()
            .expect("presents")
            .unwrap()
            .chars()
            .nth(0)
            .unwrap();

        const NUM_ROWS: usize = 3;
        let mut coords = Vec::new();
        let mut black: usize = 0;
        let mut white: usize = 0;
        for y in 0..NUM_ROWS {
            let row = lines.next().expect("p coords").unwrap();
            for (x, c) in row.chars().enumerate() {
                match c {
                    '.' => {}
                    '#' => {
                        coords.push(Point { x: x, y: y });
                        if is_white(x, y) {
                            white += 1;
                        } else {
                            black += 1;
                        }
                    }
                    _ => todo!("bad input"),
                }
            }
        }

        lines.next();

        presents.push(Shape {
            id: id,
            coords: coords,
            min_bw: std::cmp::min(black, white),
        })
    }

    // Parse contents of the form:
    // WxH: 0 1 2 3 4 5
    let mut regions: Vec<Region> = Vec::new();
    for line in lines {
        let l = line.expect("regions");
        let (dims, gifts) = l.split_once(':').expect("Invalid format");
        let (w, h) = dims.split_once('x').expect("Invalid dimensions");

        let width: usize = w.parse().expect("Invalid width");
        let height: usize = h.parse().expect("Invalid height");

        let pres = gifts
            .split_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .collect();

        regions.push(Region {
            width: width,
            height: height,
            presents: pres,
            avail_black: count_black(width, height),
            avail_white: count_white(width, height),
        });
    }

    // LOOOOL and ARRRRGH, test input has 2 don't know, but real input all
    // fail the most basic area test...

    let mut part1_count = 0;
    for r in regions {
        if let Some(v) = cheap_fit_test(&r, &presents) {
            part1_count += if v { 1 } else { 0 };
        } else {
            println!("Don't know...");
        }
    }
    println!("part1 count: {}", part1_count);
}
