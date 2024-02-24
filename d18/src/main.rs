use std::fs::File;
use std::io::Read;

fn read_lines(file_name: &str) -> Vec<String> {
    let mut file = File::open(file_name).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents.lines().map(|s| s.to_string()).collect()
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Left,
    Up,
    Right,
    Down,
}

fn parse_line(line: &String, color: bool) -> (Direction, usize) {
    let parts = line.split_whitespace().collect::<Vec<_>>();
    assert!(parts.len() == 3);
    if !color {
        let dir = match parts[0] {
            "L" => Direction::Left,
            "U" => Direction::Up,
            "R" => Direction::Right,
            "D" => Direction::Down,
            _ => panic!("Invalid direction"),
        };
        let steps = parts[1].parse::<usize>().unwrap();
        (dir, steps)
    } else {
        let hex = parts[2]
            .strip_prefix("(#")
            .unwrap()
            .strip_suffix(")")
            .unwrap();
        let steps = usize::from_str_radix(&hex[..5], 16).unwrap();
        let dir = match hex.chars().last().unwrap() {
            '0' => Direction::Right,
            '1' => Direction::Down,
            '2' => Direction::Left,
            '3' => Direction::Up,
            _ => panic!("Invalid direction"),
        };
        (dir, steps)
    }
}

fn calculate_area(instructions: &Vec<(Direction, usize)>) -> usize {
    let mut positions = vec![(0, 0)];
    let mut length = 0;
    instructions.into_iter().for_each(|(dir, len)| {
        length += len;
        let len = *len as i64;
        let (r, c) = positions.last().unwrap().to_owned();
        positions.push(match dir {
            Direction::Left => (r, c - len),
            Direction::Up => (r - len, c),
            Direction::Right => (r, c + len),
            Direction::Down => (r + len, c),
        })
    });
    let area = positions
        .windows(2)
        .map(|pair| {
            let (r1, c1) = pair[0];
            let (r2, c2) = pair[1];
            (r1 * c2) - (c1 * r2)
        })
        .sum::<i64>()
        .abs()
        / 2;
    area as usize + length / 2 + 1
}

fn main() {
    let lines = read_lines("input.txt");
    for color in [false, true] {
        let instructions = lines
            .iter()
            .map(|l| parse_line(l, color))
            .collect::<Vec<_>>();
        let area = calculate_area(&instructions);
        println!("{}", area);
    }
}
