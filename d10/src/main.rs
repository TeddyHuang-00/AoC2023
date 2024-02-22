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
    Empty,
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn opposite(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            _ => panic!("Invalid direction: {:?}", self),
        }
    }
    fn to_pos(&self) -> (i32, i32) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
            _ => panic!("Invalid direction: {:?}", self),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Pipe {
    direction: (Direction, Direction),
}

impl Pipe {
    fn extend_from(&self, direction: Direction) -> Option<Direction> {
        match self.direction {
            (x, y) | (y, x) if y == direction.opposite() => Some(x),
            _ => None,
        }
    }
}

fn parse_lines(lines: &Vec<String>) -> ((usize, usize), Vec<Vec<Pipe>>) {
    let mut row = 0;
    let mut col = 0;
    let map = lines
        .iter()
        .enumerate()
        .map(|(r, l)| {
            l.chars()
                .enumerate()
                .map(|(c, ch)| match ch {
                    'S' => {
                        row = r;
                        col = c;
                        Pipe {
                            direction: (Direction::Empty, Direction::Empty),
                        }
                    }
                    '.' => Pipe {
                        direction: (Direction::Empty, Direction::Empty),
                    },
                    '|' => Pipe {
                        direction: (Direction::Up, Direction::Down),
                    },
                    '-' => Pipe {
                        direction: (Direction::Left, Direction::Right),
                    },
                    'L' => Pipe {
                        direction: (Direction::Right, Direction::Up),
                    },
                    'J' => Pipe {
                        direction: (Direction::Left, Direction::Up),
                    },
                    'F' => Pipe {
                        direction: (Direction::Right, Direction::Down),
                    },
                    '7' => Pipe {
                        direction: (Direction::Left, Direction::Down),
                    },
                    _ => panic!("Unknown character: {}", c),
                })
                .collect()
        })
        .collect();
    ((row, col), map)
}

fn get_loop_length_and_area(map: &Vec<Vec<Pipe>>, start: (usize, usize)) -> (usize, u64) {
    let (row, col) = start;
    let (mut r, mut c) = (row, col);
    let mut length = 0;
    let mut direction = Direction::Empty;
    // Determine initial direction
    for d in vec![
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ] {
        let (dr, dc) = d.to_pos();
        if let Some(_) = map[(r as i32 + dr) as usize][(c as i32 + dc) as usize].extend_from(d) {
            direction = d;
            break;
        }
    }
    let mut coords = vec![(r, c)];
    let mut counter = vec![0; 4];

    // Get loop length and determine clockwise or counter-clockwise
    let mut visited = vec![vec![false; map[0].len()]; map.len()];
    let mut angle = 0;
    visited[r][c] = true;
    loop {
        let (dr, dc) = direction.to_pos();
        r = (r as i32 + dr) as usize;
        c = (c as i32 + dc) as usize;
        coords.push((r, c));
        length += 1;
        if visited[r][c] {
            break;
        }
        visited[r][c] = true;
        direction = map[r][c].extend_from(direction).unwrap();
        let (ddr, ddc) = direction.to_pos();
        match dr * ddc - dc * ddr {
            1 => {
                counter[1] += 1;
                angle += 90
            }
            -1 => {
                counter[3] += 1;
                angle -= 90
            }
            _ => {
                counter[2] += 1;
            }
        };
    }
    // Swap corner counters if angle is -360
    match angle {
        360 => {}
        -360 => (counter[1], counter[3]) = (counter[3], counter[1]),
        _ => panic!("Unexpected angle: {}", angle),
    }
    // Calculate area using Shoelace formula
    let showlace = coords.windows(2).fold(0, |acc, xy| {
        let (x, y) = (xy[0], xy[1]);
        let (rx, cx) = x;
        let (ry, cy) = y;
        acc + (rx * cy) as i32 - (cx * ry) as i32
    });
    // Calculate areas from corners and edges. This is the only source of error
    let extra = ((1..4).map(|i| i * counter[i]).sum::<usize>() + 3) / 4;
    // The desired area is the total area minus the area where the loop is
    let area = showlace as u64 / 2 - extra as u64;

    (length, area)
}

fn main() {
    let lines = read_lines("input.txt");
    let (start, map) = parse_lines(&lines);
    let (length, area) = get_loop_length_and_area(&map, start);
    println!("{}", length / 2);
    println!("{}", area);
}
