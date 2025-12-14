extern crate pathfinding;
extern crate regex;

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

    for m in machines {
        let start_state = MachineState {
            lights: vec!['.'; m.lights.len()],
            button: 0,
            depth: 0,
        };

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

    println!("part1 count: {}", part1_count);
}
