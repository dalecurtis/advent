use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Default, Clone, Hash, Eq, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

const VALID_DIR: [(i32, i32); 8] = [
    (1, 0),
    (0, 1),
    (-1, 0),
    (0, -1),
    (-1, -1),
    (1, -1),
    (1, 1),
    (-1, 1),
];

fn move_point(p: &Point, graph: &Vec<Vec<char>>, dir: (i32, i32)) -> Option<Point> {
    let npx = p.x as i32 + dir.0;
    let npy = p.y as i32 + dir.1;
    if npx < 0 || npy < 0 {
        return None;
    }
    if npx as usize >= graph[0].len() || npy as usize >= graph.len() {
        return None;
    }
    return Some(Point {
        x: npx as usize,
        y: npy as usize,
    });
}

fn successors(pos: &Point, c: char, graph: &Vec<Vec<char>>) -> Vec<Point> {
    let mut successors = Vec::new();
    for dir in VALID_DIR {
        let next_p = move_point(&pos, graph, dir);
        if next_p != None {
            let unwrapped_p = next_p.unwrap();
            if graph[unwrapped_p.y][unwrapped_p.x] == c {
                successors.push(unwrapped_p);
            }
        }
    }
    return successors;
}

fn print_maze(maze: &Vec<Vec<char>>) {
    for y in 0..maze.len() {
        for x in 0..maze[y].len() {
            print!("{}", maze[y][x])
        }
        println!();
    }
    println!();
}

fn main() {
    let file = File::open("input4.txt").expect("Unable to open file");
    let reader = BufReader::new(file);

    let mut tp_locations = Vec::new();
    let mut maze: Vec<Vec<char>> = Vec::new();
    let mut y_pos = 0;
    for line in reader.lines() {
        let line = line.expect("Unable to read line");
        for (x, c) in line.chars().enumerate() {
            match c {
                '.' => {}
                '@' => {
                    tp_locations.push(Point { x: x, y: y_pos });
                }
                _ => todo!("bad input"),
            }
        }

        maze.push(line.chars().collect());
        y_pos += 1;
    }

    print_maze(&maze);

    let mut part1_count = 0;
    for tp in tp_locations {
        let s = successors(&tp, '@', &maze);
        if s.len() < 4 {
            part1_count += 1;
        }
    }

    println!("part1 count: {}", part1_count);
}
