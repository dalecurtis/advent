use std::fs::File;
use std::io::{BufRead, BufReader};

fn get_digit(v: &str, pos: u64) -> Option<u64> {
    let c = v.chars().nth(pos as usize);
    if c == None || c == Some(' ') {
        return None;
    }
    return Some(c.unwrap().to_digit(10).unwrap() as u64);
}

fn main() {
    let file = File::open("input6.txt").expect("Unable to open file");
    let reader = BufReader::new(file);

    let mut values: Vec<Vec<u64>> = Vec::new();
    let mut ops: Vec<char> = Vec::new();
    let mut max_digits: Vec<u64> = Vec::new();
    let mut raw_lines: Vec<String> = Vec::new();
    for line in reader.lines() {
        let line = line.expect("Unable to read line");

        // Extract the operations line
        if !line.starts_with(|c: char| c.is_digit(10) || c == ' ') {
            ops = line
                .split_whitespace()
                .map(|s| s.chars().next().unwrap())
                .collect();
            break;
        }
        // Extract values as simple integers for part 1.
        let int_range: Vec<u64> = line
            .split_whitespace()
            .map(|s| s.parse::<u64>().unwrap())
            .collect();

        // Figure out max digits per column to make string parsing easy.
        if max_digits.is_empty() {
            max_digits = vec![0; int_range.len()];
        }
        for i in 0..int_range.len() {
            let num_digits: u64 = (int_range[i].ilog10() + 1).into();
            if num_digits > max_digits[i] {
                max_digits[i] = num_digits;
            }
        }
        raw_lines.push(line);
        values.push(int_range);
    }

    println!("values count: {}", values.len());
    println!("ops count: {}", ops.len());

    // Split based on max digits per line to keep spaces since they matter.
    let mut str_values: Vec<Vec<String>> = Vec::new();
    for mut line in raw_lines {
        let mut str_value: Vec<String> = Vec::new();
        loop {
            let s = line.drain(..max_digits[str_value.len()] as usize).collect();
            str_value.push(s);
            if line.is_empty() {
                break;
            }
            line.drain(..1);
        }
        str_values.push(str_value);
    }

    let mut result1: u64 = 0;
    let mut result2: u64 = 0;
    for (i, op) in ops.into_iter().enumerate() {
        let mut result: u64;
        let mut result2_tmp: u64;
        match op {
            '*' => {
                result = 1;
                result2_tmp = 1;
            }
            '+' => {
                result = 0;
                result2_tmp = 0;
            }
            _ => {
                todo!("unsupported operation {}", op);
            }
        }
        for j in 0..values.len() {
            match op {
                '*' => {
                    result *= values[j][i];
                }
                '+' => {
                    result += values[j][i];
                }
                _ => {
                    todo!("unsupported operation {}", op);
                }
            }
        }

        // Assemble each value based on max digits and apply operation.
        for k in (0..max_digits[i]).rev() {
            let mut avenger_v: u64 = 0;
            for j in 0..str_values.len() {
                let v = get_digit(&str_values[j][i], k);
                if v.is_some() {
                    avenger_v = avenger_v * 10 + v.unwrap();
                }
            }
            match op {
                '*' => {
                    result2_tmp *= avenger_v;
                }
                '+' => {
                    result2_tmp += avenger_v;
                }
                _ => {
                    todo!("unsupported operation {}", op);
                }
            }
        }

        result1 += result;
        result2 += result2_tmp;
    }

    println!("result1: {}, result2: {}", result1, result2);
}
