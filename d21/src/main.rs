use std::collections::VecDeque;
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
    Plot,
    Rock,
}

fn parse_input(lines: &Vec<String>) -> (Vec<Vec<Grid>>, (usize, usize)) {
    let (mut row, mut col) = (0, 0);
    let map = lines
        .iter()
        .enumerate()
        .map(|(r, line)| {
            line.chars()
                .enumerate()
                .map(|(c, ch)| match ch {
                    '.' => Grid::Plot,
                    '#' => Grid::Rock,
                    'S' => {
                        (row, col) = (r, c);
                        Grid::Plot
                    }
                    _ => panic!("Invalid character: {}", ch),
                })
                .collect()
        })
        .collect();
    (map, (row, col))
}

fn move_steps(map: &Vec<Vec<Grid>>, (row, col): (usize, usize), max_steps: usize) -> usize {
    let mut queue = VecDeque::from([(row, col)]);
    let mut visited = vec![vec![None; map[0].len()]; map.len()];
    visited[row][col] = Some(0usize);
    while !queue.is_empty() {
        let (row, col) = queue.pop_front().unwrap();
        let step = visited[row][col].unwrap();
        if step < max_steps {
            let mut next = Vec::new();
            if row > 0 {
                next.push((row - 1, col));
            }
            if row < map.len() - 1 {
                next.push((row + 1, col));
            }
            if col > 0 {
                next.push((row, col - 1));
            }
            if col < map[0].len() - 1 {
                next.push((row, col + 1));
            }
            next.into_iter().for_each(|(r, c)| {
                if map[r][c] == Grid::Plot && visited[r][c].is_none() {
                    visited[r][c] = Some(step + 1);
                    queue.push_back((r, c));
                }
            });
        }
    }
    visited
        .iter()
        .flatten()
        .flatten()
        .filter(|&x| (max_steps - *x) % 2 == 0)
        .count()
}

fn move_steps_infinite(
    map: &Vec<Vec<Grid>>,
    (row, col): (usize, usize),
    max_steps: usize,
) -> usize {
    assert!(map.len() == map[0].len()); // Square map
    let n = map.len();
    assert!(n % 2 == 1); // The chunks will be in alternate even and odd steps
    assert!(row == n / 2 && col == n / 2); // Starting point is in the middle
    assert!((max_steps - (n / 2)) % n == 0); // max_steps is multiple of n after first chunk
    assert!(((max_steps - (n / 2)) / n) % 2 == 0); // max_steps is multiple of n after first chunk
    let chunks = (max_steps - (n / 2)) / n;
    // Make the map 5 times bigger in each direction for our interpolation
    let extended = map
        .iter()
        .map(|row| row.repeat(5))
        .cycle()
        .take(5 * n)
        .collect::<Vec<_>>();
    let (row, col) = (2 * n + row, 2 * n + col);
    let k1 = move_steps(&extended, (row, col), n / 2);
    let k2 = move_steps(&extended, (row, col), n / 2 + n);
    let k3 = move_steps(&extended, (row, col), n / 2 + n * 2);
    let c = k1;
    let b = (4 * k2 - 3 * k1 - k3) / 2;
    let a = k2 - k1 - b;
    println!("{a}x^2 + {b}x + {c}");
    a * chunks.pow(2) + c + b * chunks
}

fn main() {
    let lines = read_lines("input.txt");
    let (map, (row, col)) = parse_input(&lines);
    let result = move_steps(&map, (row, col), 64);
    println!("{}", result);
    // Infinite map is easy as from starting point to the neighboring chunks
    // the shortest path is in straight line. Therefore the number of chunks
    // it can reach is a quadratic function of the number of steps. And the
    // number of possible positions are also a quadratic function of the number
    let result = move_steps_infinite(&map, (row, col), 26501365);
    println!("{}", result);
}
