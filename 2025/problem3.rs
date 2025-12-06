use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

const MAX_DEPTH: u32 = 12;

fn build_num(digits: &Vec<u32>) -> u64 {
    return digits.iter().fold(0, |acc, &digit| acc * 10 + digit as u64);
}

fn find_largest(
    pos: usize,
    depth: u32,
    digits: &Vec<u32>,
    mut cache: &mut HashMap<(usize, u32), Vec<u32>>,
) -> Vec<u32> {
    if pos == digits.len() - 1 || depth == MAX_DEPTH - 1 {
        return vec![digits[pos]];
    }

    {
        let cached_largest = cache.get(&(pos, depth));
        if !cached_largest.is_none() {
            return (*cached_largest.unwrap().clone()).to_vec();
        }
    }

    let mut largest: u64 = 0;
    let mut largest_vec: Vec<u32> = Vec::new();
    for i in pos + 1..digits.len() {
        let vd = find_largest(i, depth + 1, &digits, &mut cache);
        let v = digits[pos] as u64 * 10_u64.pow(vd.len() as u32) + build_num(&vd);
        if v > largest {
            largest = v;
            largest_vec = vd;
            largest_vec.insert(0, digits[pos]);
        }
    }

    cache.insert((pos, depth), largest_vec.clone());
    return largest_vec;
}

fn main() {
    let file = File::open("input3.txt").expect("Unable to open file");
    let reader = BufReader::new(file);

    let mut power_sum1: u64 = 0;
    let mut power_sum2: u64 = 0;
    for line in reader.lines() {
        let line = line.expect("Unable to read line");

        let mut leading: u32 = 0;
        let mut trailing: Option<u32> = None;
        let mut digits: Vec<u32> = Vec::with_capacity(line.len());
        for (i, c) in line.chars().enumerate() {
            let d = c.to_digit(10).expect("bad input");
            if d > leading && i + 1 < line.len() {
                leading = d;
                trailing = None;
            } else if Some(d) > trailing {
                trailing = Some(d);
            }
            digits.push(d);
        }

        let v = 10 * leading + trailing.expect("bad logic");
        power_sum1 += v as u64;

        let mut largest: u64 = 0;
        let mut cache = HashMap::new();
        for i in 0..digits.len() {
            let vd = find_largest(i, 0, &digits, &mut cache);
            let v = build_num(&vd);
            if v > largest {
                largest = v;
            }
        }

        power_sum2 += largest;
    }

    println!("Power Sum: {}, {}", power_sum1, power_sum2);
}
