use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // Open the file
    let file = File::open("input1.txt").expect("Unable to open file");
    let reader = BufReader::new(file);

    let mut code: i32 = 50;
    let mut count1: i32 = 0;
    let mut count2: i32 = 0;

    const WRAP: i32 = 100;

    // Iterate over each line in the file
    for line in reader.lines() {
        let line = line.expect("Unable to read line");

        let direction_char = line.chars().next().unwrap();
        let number = line.get(1..).unwrap().parse::<i32>().unwrap();

        if number == 0 {
            todo!("didn't consider");
        }

        let mut temp = code;
        match direction_char {
            'R' => {
                temp += number;
                // temp < wrap = 0 rotation
                // temp == wrap ==> 1 rotation handled below.
                // temp > wrap => n rotations.
                if temp > WRAP {
                    count2 += temp / WRAP;
                } else if temp == WRAP {
                    count2 += 1;
                }
                code = temp % WRAP;
            }

            'L' => {
                temp -= number;
                // temp > 0 = 0 rotations
                // temp <= 0 = 1 or more rotations.
                if temp <= 0 {
                    if code == 0 {
                        if number >= WRAP {
                            count2 += number / WRAP;
                        }
                    } else {
                        count2 += 1 + -temp / WRAP;
                    }

                    code = (WRAP - -temp % WRAP) % WRAP;
                } else {
                    code = temp;
                }
            }

            _ => {
                todo!("bad input");
            }
        }

        if code == 0 {
            count1 += 1;
        }

        if code < 0 {
            todo!("broken");
        }
    }

    println!("Code: {}, Count1: {}, Count2: {}", code, count1, count2);
}
