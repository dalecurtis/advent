use std::fs;

fn split_number(v: u64, num_digits: u32) -> (u64, u64) {
    let divisor = 10_u64.pow(num_digits / 2) as u64;
    let first_part: u64 = v / divisor;
    let second_part: u64 = v % divisor;
    return (first_part, second_part);
}

fn main() {
    let line = fs::read_to_string("input2-test.txt").unwrap();

    let mut invalid_sum1: u64 = 0;
    for range in line.split(',') {
        let int_range: Vec<u64> = range
            .split('-')
            .map(|s| s.parse::<u64>().unwrap())
            .collect();

        for v in int_range[0]..=int_range[1] {
            let num_digits = v.ilog10() + 1;
            if num_digits & 1 == 1 {
                continue;
            }

            let (upper, lower) = split_number(v, num_digits);
            if upper == lower {
                invalid_sum1 += v;
            }
        }

        println!("{}, {}", int_range[0], int_range[1]);
    }

    println!("Invalid Sum: {}", invalid_sum1);
}
