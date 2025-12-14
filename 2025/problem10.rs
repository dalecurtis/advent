extern crate good_lp;
extern crate pathfinding;
extern crate regex;

use good_lp::{
    constraint, solvers::microlp::microlp, variable, variables, Expression, Solution, SolverModel,
    Variable,
};
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

use pathfinding::prelude;

#[derive(Debug, Default, Clone, Hash, Eq, PartialEq)]
struct Machine {
    lights: Vec<char>,
    buttons: Vec<Vec<usize>>,
    joltage: Vec<i32>,
}

#[derive(Debug, Default, Clone, Hash, Eq, PartialEq)]
struct MachineState {
    lights: Vec<char>,
    button: usize,
    depth: usize,
}

fn cost(state: &MachineState) -> u32 {
    return state.depth as u32;
}

fn apply_buttons(lights: &Vec<char>, button: &Vec<usize>) -> Vec<char> {
    let mut light_clone = lights.clone();
    for v in button {
        if light_clone[*v] == '#' {
            light_clone[*v] = '.';
        } else {
            light_clone[*v] = '#';
        }
    }
    return light_clone;
}

fn successors(state: &MachineState, machine: &Machine) -> Vec<(MachineState, u32)> {
    let mut successors = Vec::new();
    for (i, b) in machine.buttons.clone().into_iter().enumerate() {
        let m = MachineState {
            lights: apply_buttons(&state.lights, &b),
            button: i,
            depth: state.depth + 1,
        };
        let cost_m = cost(&m);
        successors.push((m, cost_m));
    }
    return successors;
}

fn solve_lp_problem(m: &Machine) -> usize {
    // Build an buttons x joltage sized matrix.
    // E.g. for the first sample:
    //
    //   [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
    //
    // We want to solve:
    //   | 0 0 0 1 |     |p0|
    //   | 0 1 0 1 |     |p1|      | 3 |
    //   | 0 0 1 0 |     |p2|      | 5 |
    //   | 0 0 1 1 |  *  |p3]   =  | 4 |
    //   | 1 0 1 0 |     |p4|      | 7 |
    //   | 1 1 0 0 |     |p5|
    //
    // Which gives:
    //
    //   |  0 +  0 +  0 +  0 + p4 + p5 |     | 3 |
    //   |  0 + p1 +  0 +  0 +  0 + p5 |  =  | 5 |
    //   |  0 +  0 + p2 + p3 + p4 +  0 |     | 4 |
    //   | p0 + p1 +  0 + p3 +  0 +  0 |     | 7 |

    let mut matrix = vec![vec![0; m.buttons.len()]; m.joltage.len()];
    for (i, b) in m.buttons.clone().into_iter().enumerate() {
        for v in b {
            matrix[v][i] = 1;
        }
    }

    // Create variables for each button.
    let mut vars = variables!();
    let p: Vec<Variable> = (0..m.buttons.len())
        .map(|_| vars.add(variable().min(0).integer()))
        .collect();

    // We want to minimize the sum of the button presses.
    let objective: Expression = p.iter().sum();

    // Use the matrix to produce
    let mut problem = vars.minimise(objective).using(microlp);
    for (r, j) in m.joltage.clone().into_iter().enumerate() {
        let p_sum: Expression = matrix[r]
            .iter()
            .zip(p.iter())
            .filter(|(&coeff, _)| coeff == 1)
            .map(|(&coeff, &var)| var * coeff)
            .sum();
        problem = problem.with(p_sum.eq(j));
    }

    let sol = problem.solve().unwrap();

    let mut total = 0;
    for sp in p {
        // Argh, .integer() above is not perfect.
        let x = sol.value(sp).round();
        if x < 0.0 {
            todo!("Unexpected negative {}", sol.value(sp));
        }
        total += x as usize;
    }
    return total;
}

// Thanks Gemini!
fn parse_line(line: &str) -> Machine {
    // Regex breakdown:
    // ^\[(?P<lights>[^\]]+)\]  -> Start with [ ... ] capturing content as 'lights'
    // \s+                      -> Whitespace
    // (?P<buttons>.*)          -> Capture everything in the middle as 'buttons'
    // \s+                      -> Whitespace
    // \{(?P<joltage>[\d,]+)\}$ -> End with { ... } capturing content as 'joltage'
    let main_re =
        Regex::new(r"^\[(?P<lights>[^\]]+)\]\s+(?P<buttons>.*)\s+\{(?P<joltage>[\d,]+)\}$")
            .expect("Valid regex");

    // Regex to find individual (x,y,z) groups inside the 'buttons' string
    let button_group_re = Regex::new(r"\((?P<nums>[\d,]+)\)").expect("Regex compile");

    let caps = main_re.captures(line).expect("Line did not match format");

    // 1. Parse Lights: simple char collection
    let lights: Vec<char> = caps["lights"].chars().collect();

    // 2. Parse Buttons: find all (parentheses), then split by comma
    let buttons_str = &caps["buttons"];
    let mut buttons = Vec::new();

    for cap in button_group_re.captures_iter(buttons_str) {
        let nums: Vec<usize> = cap["nums"]
            .split(',')
            .map(|s: &str| s.trim().parse::<usize>().expect("ints"))
            .collect();
        buttons.push(nums);
    }

    // 3. Parse Joltage: split the curly brace content by comma
    let joltage: Vec<i32> = caps["joltage"]
        .split(',')
        .map(|s: &str| s.trim().parse::<i32>().expect("more ints"))
        .collect();

    return Machine {
        lights,
        buttons,
        joltage,
    };
}

fn main() {
    let file = File::open("input10.txt").expect("Unable to open file");
    let reader = BufReader::new(file);

    let mut machines: Vec<Machine> = Vec::new();
    for line in reader.lines() {
        let line = line.expect("Unable to read line");
        machines.push(parse_line(&line));
    }

    println!("machines.len={}", machines.len());

    let mut part1_count: usize = 0;
    let mut part2_count: usize = 0;

    for m in machines {
        let start_state = MachineState {
            lights: vec!['.'; m.lights.len()],
            button: 0,
            depth: 0,
        };

        {
            let result = prelude::astar(
                &start_state,
                |s: &MachineState| successors(s, &m),
                |s: &MachineState| s.depth as u32,
                |s: &MachineState| s.lights == m.lights,
            );

            if result.is_none() {
                todo!("unexpected no path!");
            }
            let path = result.unwrap();
            part1_count += path.0.len() - 1; // -1 for start_state.
        }

        // Too slow for part 2.
        //
        // {
        //     start_state.joltage = m.joltage.clone();
        //     let result = prelude::astar(
        //         &start_state,
        //         |s: &MachineState| successors2(s, &m),
        //         |s: &MachineState| s.joltage.iter().sum::<i32>() as u32,
        //         |s: &MachineState| s.joltage.iter().all(|&x| x == 0),
        //     );
        //     if result.is_none() {
        //         todo!("unexpected no path!");
        //     }
        //     let path = result.unwrap();
        //     part2_count += path.0.len() - 1; // -1 for start_state.
        //     println!("slow={}", path.0.len() - 1);
        // }

        part2_count += solve_lp_problem(&m);
    }

    println!("part1 count: {}", part1_count);
    println!("part2 count: {}", part2_count);
}
