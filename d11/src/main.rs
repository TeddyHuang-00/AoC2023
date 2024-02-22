use std::fs::File;
use std::io::Read;

fn read_lines(file_name: &str) -> Vec<String> {
    let mut file = File::open(file_name).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents.lines().map(|s| s.to_string()).collect()
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Space {
    Empty,
    Galaxy,
}

fn parse_input(input: Vec<String>, span_size: usize) -> Vec<(usize, usize)> {
    let mut galaxies = Vec::new();
    let map = input
        .iter()
        .enumerate()
        .map(|(r, s)| {
            s.chars()
                .enumerate()
                .map(|(c, ch)| match ch {
                    '.' => Space::Empty,
                    '#' => {
                        galaxies.push((r, c));
                        Space::Galaxy
                    }
                    _ => panic!("Invalid input"),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    (0..map[0].len())
        .rev()
        .filter(|&i| map.iter().all(|r| r[i] == Space::Empty))
        .for_each(|i| {
            galaxies.iter_mut().for_each(|(_, c)| {
                if *c > i {
                    *c += span_size - 1;
                }
            })
        });
    (0..map.len())
        .rev()
        .filter(|&i| map[i].iter().all(|&r| r == Space::Empty))
        .for_each(|i| {
            galaxies.iter_mut().for_each(|(r, _)| {
                if *r > i {
                    *r += span_size - 1;
                }
            })
        });
    galaxies
}

fn calculate_distance(coords: Vec<(usize, usize)>) -> Vec<usize> {
    (0..coords.len())
        .map(|i| {
            (i + 1..coords.len())
                .map(|j| {
                    (coords[i].0 as isize - coords[j].0 as isize).abs() as usize
                        + (coords[i].1 as isize - coords[j].1 as isize).abs() as usize
                })
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect::<Vec<_>>()
}

fn main() {
    let lines = read_lines("input.txt");
    let map = parse_input(lines.to_owned(), 2);
    let dist = calculate_distance(map);
    println!("{}", dist.iter().sum::<usize>());
    let map = parse_input(lines, 1_000_000);
    let dist = calculate_distance(map);
    println!("{}", dist.iter().sum::<usize>());
}
