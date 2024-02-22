use std::fs::File;
use std::io::Read;

fn read_lines(file_name: &str) -> Vec<String> {
    let mut file = File::open(file_name).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents.lines().map(|s| s.to_string()).collect()
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Terrain {
    Ash,
    Rock,
}

fn parse_lines(lines: Vec<String>) -> Vec<Vec<Vec<Terrain>>> {
    lines
        .split(|s| s.is_empty())
        .map(|s| {
            s.iter()
                .map(|l| {
                    l.chars()
                        .map(|c| match c {
                            '.' => Terrain::Ash,
                            '#' => Terrain::Rock,
                            _ => panic!("Unknown terrain type: {}", c),
                        })
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

fn locate_mirror(terrain: Vec<Vec<Terrain>>, smudge: usize) -> (Option<usize>, Option<usize>) {
    let nrows = terrain.len();
    let ncols = terrain[0].len();
    let mut mirror = (None, None);
    // Check if mirror is horizontal
    for i in 1..nrows {
        let n = terrain
            .iter()
            .skip(i)
            .zip(terrain.iter().rev().skip(nrows - i))
            .map(|(a, b)| {
                a.iter()
                    .zip(b.iter())
                    .map(|(x, y)| if x == y { 0 } else { 1 })
                    .sum::<usize>()
            })
            .sum::<usize>();
        if n == smudge {
            mirror.0 = Some(i);
            return mirror;
        }
    }
    let transposed = (0..ncols)
        .map(|i| terrain.iter().map(|r| r[i]).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    // Check if mirror is vertical
    for j in 1..ncols {
        let n = transposed
            .iter()
            .skip(j)
            .zip(transposed.iter().rev().skip(ncols - j))
            .map(|(a, b)| {
                a.iter()
                    .zip(b.iter())
                    .map(|(x, y)| if x == y { 0 } else { 1 })
                    .sum::<usize>()
            })
            .sum::<usize>();
        if n == smudge {
            mirror.1 = Some(j);
            return mirror;
        }
    }
    panic!("No mirror found");
}

fn main() {
    let lines = read_lines("input.txt");
    let terrains = parse_lines(lines);
    for smudge in [0, 1] {
        let mirrors = terrains
            .iter()
            .map(|t| locate_mirror(t.to_owned(), smudge))
            .collect::<Vec<_>>();
        println!(
            "{}",
            mirrors
                .into_iter()
                .map(|(row, col)| {
                    match (row, col) {
                        (Some(r), None) => r * 100,
                        (None, Some(c)) => c,
                        _ => panic!("No mirror found"),
                    }
                })
                .sum::<usize>()
        );
    }
}
