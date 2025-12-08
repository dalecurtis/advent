extern crate binary_search;

use std::fs::File;
use std::io::{BufRead, BufReader};

use binary_search::{binary_search, Direction};

fn has_overlap(a: &(u64, u64), b: &(u64, u64)) -> bool {
    return a.0 <= b.0 && a.1 >= b.0;
}

fn main() {
    let file = File::open("input5.txt").expect("Unable to open file");
    let reader = BufReader::new(file);

    let mut items: Vec<u64> = Vec::new();
    let mut ranges: Vec<(u64, u64)> = Vec::new();
    let mut parsing_ranges = true;
    for line in reader.lines() {
        let line = line.expect("Unable to read line");
        if line.is_empty() {
            parsing_ranges = false;
            continue;
        }

        if parsing_ranges {
            let int_range: Vec<u64> = line.split('-').map(|s| s.parse::<u64>().unwrap()).collect();
            if int_range[0] > int_range[1] {
                todo!("unexpected range");
            }
            ranges.push((int_range[0], int_range[1]));
        } else {
            let item = line.parse::<u64>().unwrap();
            items.push(item);
        }
    }

    println!("range count: {}", ranges.len());
    println!("item count: {}", items.len());

    ranges.sort();

    let mut i = 0;
    loop {
        if i + 1 >= ranges.len() {
            break;
        }
        if has_overlap(&ranges[i], &ranges[i + 1]) {
            if ranges[i + 1].1 > ranges[i].1 {
                ranges[i].1 = ranges[i + 1].1;
            }
            ranges.remove(i + 1);
        } else {
            i += 1;
        }
    }

    // println!("{:?}", ranges);
    // println!("{:?}", items);
    println!("merged range count: {}", ranges.len());

    let mut fresh_count = 0;
    for item in items {
        let (insert_index, _) = binary_search((0, ()), (ranges.len(), ()), |range: usize| {
            let r = ranges[range];
            if r.0 < item {
                return Direction::Low(());
            } else {
                return Direction::High(());
            }
        });

        let r = ranges[insert_index.0];
        if r.0 <= item && r.1 >= item {
            fresh_count += 1;
        }
    }

    let mut total_fresh_count = 0;
    for r in &ranges {
        total_fresh_count += r.1 - r.0 + 1;
    }

    println!("Fresh count: {}", fresh_count);
    println!("Total fresh count: {}", total_fresh_count);
}
