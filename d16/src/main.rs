use std::fs::File;
use std::io::Read;

fn read_lines(file_name: &str) -> Vec<String> {
    let mut file = File::open(file_name).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents.lines().map(|s| s.to_string()).collect()
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Grid {
    Empty,
    Mirror(bool),
    Splitter(bool),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
}

fn parse_input(lines: &Vec<String>) -> Vec<Vec<Grid>> {
    lines
        .iter()
        .map(|l| {
            l.chars()
                .map(|ch| match ch {
                    '.' => Grid::Empty,
                    '/' => Grid::Mirror(false),
                    '\\' => Grid::Mirror(true),
                    '|' => Grid::Splitter(false),
                    '-' => Grid::Splitter(true),
                    _ => panic!("Unknown symbol: {ch}"),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

struct Beam {
    pub position: (usize, usize),
    pub direction: Direction,
}

fn convert_direction(dir: Direction) -> (isize, isize) {
    match dir {
        Direction::Right => (0, 1),
        Direction::Down => (1, 0),
        Direction::Left => (0, -1),
        Direction::Up => (-1, 0),
    }
}

fn traverse(map: &Vec<Vec<Grid>>, pos: (usize, usize), dir: Direction) -> Vec<Vec<bool>> {
    let mut visited = vec![vec![vec![false; 4]; map[0].len()]; map.len()];
    let mut beams = vec![Beam {
        position: pos,
        direction: dir,
    }];
    visited[pos.0][pos.1][dir as usize] = true;
    while !beams.is_empty() {
        let beam = beams.pop().unwrap();
        let (r, c) = beam.position;
        let new_directions = match map[r][c] {
            Grid::Empty => vec![beam.direction],
            Grid::Mirror(mirror_type) => match (beam.direction, mirror_type) {
                (Direction::Right, false) | (Direction::Left, true) => vec![Direction::Up],
                (Direction::Down, false) | (Direction::Up, true) => vec![Direction::Left],
                (Direction::Right, true) | (Direction::Left, false) => vec![Direction::Down],
                (Direction::Down, true) | (Direction::Up, false) => vec![Direction::Right],
            },
            Grid::Splitter(splitter_type) => match (beam.direction, splitter_type) {
                (Direction::Right, false) | (Direction::Left, false) => {
                    vec![Direction::Up, Direction::Down]
                }
                (Direction::Down, true) | (Direction::Up, true) => {
                    vec![Direction::Right, Direction::Left]
                }
                _ => vec![beam.direction],
            },
        };
        for new_direction in new_directions {
            let (dr, dc) = convert_direction(new_direction);
            let (nr, nc) = (r as isize + dr, c as isize + dc);
            if nr >= 0 && nr < map.len() as isize && nc >= 0 && nc < map[0].len() as isize {
                let (nr, nc) = (nr as usize, nc as usize);
                if !visited[nr][nc][new_direction as usize] {
                    visited[nr][nc][new_direction as usize] = true;
                    beams.push(Beam {
                        position: (nr, nc),
                        direction: new_direction,
                    });
                }
            }
        }
    }
    visited
        .iter()
        .map(|r| r.iter().map(|c| c.iter().any(|v| *v)).collect())
        .collect()
}

fn trials(map: &Vec<Vec<Grid>>) -> usize {
    let mut max_energized = 0;
    let (nr, nc) = (map.len(), map[0].len());
    for (rows, cols, d) in [
        (vec![0; nc], Vec::from_iter(0..nc), Direction::Down),
        (vec![nr - 1; nc], Vec::from_iter(0..nc), Direction::Up),
        (Vec::from_iter(0..nr), vec![0; nr], Direction::Right),
        (Vec::from_iter(0..nr), vec![nc - 1; nr], Direction::Left),
    ] {
        for (r, c) in rows.into_iter().zip(cols.into_iter()) {
            let energized_map = traverse(&map, (r, c), d);
            max_energized = max_energized.max(
                energized_map
                    .iter()
                    .map(|r| r.iter().filter(|c| **c).count())
                    .sum::<usize>(),
            );
        }
    }
    max_energized
}

fn main() {
    let lines = read_lines("input.txt");
    let map = parse_input(&lines);
    let energized_map = traverse(&map, (0, 0), Direction::Right);
    println!(
        "{}",
        energized_map
            .iter()
            .map(|r| r.iter().filter(|c| **c).count())
            .sum::<usize>()
    );
    println!("{}", trials(&map));
}
