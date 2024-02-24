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
    Rock,
    Fixed,
    Empty,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    North,
    West,
    South,
    East,
}

fn parse_lines(lines: Vec<String>) -> Vec<Vec<Grid>> {
    lines
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    'O' => Grid::Rock,
                    '#' => Grid::Fixed,
                    '.' => Grid::Empty,
                    _ => panic!("Invalid character"),
                })
                .collect()
        })
        .collect()
}

fn get_deltas(direction: Direction) -> (i32, i32) {
    match direction {
        Direction::North => (-1, 0),
        Direction::East => (0, 1),
        Direction::South => (1, 0),
        Direction::West => (0, -1),
    }
}

fn tilt(platform: Vec<Vec<Grid>>, direction: Direction) -> Vec<Vec<Grid>> {
    let mut platform = platform;
    let (dr, dc) = get_deltas(direction);
    let (row, col) = (platform.len(), platform[0].len());
    for i in match dr {
        1 => Box::new((0..row).rev()) as Box<dyn Iterator<Item = usize>>,
        _ => Box::new(0..row) as Box<dyn Iterator<Item = usize>>,
    } {
        for j in match dc {
            1 => Box::new((0..col).rev()) as Box<dyn Iterator<Item = usize>>,
            _ => Box::new(0..col) as Box<dyn Iterator<Item = usize>>,
        } {
            match platform[i][j] {
                Grid::Rock => {
                    let (mut r, mut c) = (i, j);
                    loop {
                        let nr = r as i32 + dr;
                        let nc = c as i32 + dc;
                        if nr >= 0
                            && nr < row as i32
                            && nc >= 0
                            && nc < col as i32
                            && platform[nr as usize][nc as usize] == Grid::Empty
                        {
                            platform[nr as usize][nc as usize] = Grid::Rock;
                            platform[r][c] = Grid::Empty;
                            r = nr as usize;
                            c = nc as usize;
                        } else {
                            break;
                        }
                    }
                }
                _ => {}
            }
        }
    }
    platform
}

fn calc_load(platform: &Vec<Vec<Grid>>) -> usize {
    platform
        .iter()
        .enumerate()
        .map(|(r, row)| row.iter().filter(|&&x| x == Grid::Rock).count() * (platform.len() - r))
        .sum()
}

fn cycle(platform: &Vec<Vec<Grid>>) -> Vec<Vec<Grid>> {
    let mut platform = platform.to_owned();
    platform = tilt(platform, Direction::North);
    platform = tilt(platform, Direction::West);
    platform = tilt(platform, Direction::South);
    platform = tilt(platform, Direction::East);
    platform
}

fn main() {
    let lines = read_lines("input.txt");
    let mut platform = parse_lines(lines);
    let tilt_north_once = tilt(platform.clone(), Direction::North);
    let load = calc_load(&tilt_north_once);
    println!("{}", load);
    // Burn in stage before determining the period
    // This number is arbitrary, but it should be large enough
    // to ensure that the platform has reached a stable state
    // after the burn-in stage
    let burn_in = 1_00;
    for _ in 0..burn_in {
        platform = cycle(&platform);
    }
    // Determine the period
    let start = platform.clone();
    let mut period = 0;
    loop {
        platform = cycle(&platform);
        period += 1;
        if platform == start {
            break;
        }
    }
    // Skip to the 10^9-th stage
    platform = start;
    for _ in 0..((1_000_000_000 - burn_in) % period) {
        platform = cycle(&platform);
    }
    let load = calc_load(&platform);
    println!("{}", load);
}
