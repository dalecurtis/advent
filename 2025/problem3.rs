use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input3.txt").expect("Unable to open file");
    let reader = BufReader::new(file);

    let mut power_sum1: u64 = 0;
    let mut power_sum2: u64 = 0;
    for line in reader.lines() {
        let line = line.expect("Unable to read line");

        let mut leading: u32 = 0;
        let mut trailing: Option<u32> = None;
        for (i, c) in line.chars().enumerate() {
            let d = c.to_digit(10).expect("bad input");
            if d > leading && i + 1 < line.len() {
                leading = d;
                trailing = None;
            } else if Some(d) > trailing {
                trailing = Some(d);
            }
        }

        let v = 10 * leading + trailing.expect("bad logic");
        power_sum1 += v as u64;
    }

    println!("Power Sum: {}, {}", power_sum1, power_sum2);
}
