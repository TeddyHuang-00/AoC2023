use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

fn read_lines(file_name: &str) -> Vec<String> {
    let mut file = File::open(file_name).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents.lines().map(|s| s.to_string()).collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Instruction {
    Left,
    Right,
}

fn parse_map(lines: Vec<String>) -> (Vec<Instruction>, HashMap<String, (String, String)>) {
    let sequence = lines[0]
        .chars()
        .map(|ch| match ch {
            'L' => Instruction::Left,
            'R' => Instruction::Right,
            _ => panic!("Invalid instruction"),
        })
        .collect::<Vec<_>>();
    let mut map = HashMap::new();
    for line in lines.into_iter().skip(2) {
        let parts = line.split(" = ").collect::<Vec<_>>();
        assert!(parts.len() == 2);
        let src = parts[0].to_string();
        let dst = parts[1]
            .strip_prefix("(")
            .unwrap()
            .strip_suffix(")")
            .unwrap()
            .split(", ")
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        assert!(dst.len() == 2);
        map.insert(src, (dst[0].to_owned(), dst[1].to_owned()));
    }
    (sequence, map)
}

fn follow_map(instruction: Vec<Instruction>, map: HashMap<String, (String, String)>) -> u64 {
    let mut steps = 0;
    let mut position = String::from("AAA");
    while position != "ZZZ" {
        let (left, right) = map.get(&position).unwrap();
        position = match instruction[steps % instruction.len()] {
            Instruction::Left => left.to_owned(),
            Instruction::Right => right.to_owned(),
        };
        steps += 1;
    }
    steps as u64
}

fn follow_map_simultaneous(
    instruction: Vec<Instruction>,
    map: HashMap<String, (String, String)>,
) -> u64 {
    let positions = map
        .keys()
        .filter(|k| k.ends_with("A"))
        .map(|s| s.to_owned())
        .collect::<Vec<_>>();
    // We just assume it will be in cycle after some testing.
    // Fingers crossed and hope for the best!
    let periods = positions
        .into_iter()
        .map(|pos| {
            let mut position = pos.to_owned();
            let mut s = 0;
            while !position.ends_with("Z") {
                let (left, right) = map.get(&position).unwrap();
                position = match instruction[s % instruction.len()] {
                    Instruction::Left => left.to_owned(),
                    Instruction::Right => right.to_owned(),
                };
                s += 1;
            }
            s as u64
        })
        .collect::<Vec<_>>();
    let gcd = |a: u64, b: u64| {
        let mut a = a;
        let mut b = b;
        while b != 0 {
            (a, b) = (b, a % b);
        }
        a
    };
    let lcm = periods.iter().fold(1, |acc, &x| acc * x / gcd(acc, x));
    lcm
}

fn main() {
    let lines = read_lines("input.txt");
    let (sequence, map) = parse_map(lines);
    let steps = follow_map(sequence.to_owned(), map.to_owned());
    println!("{}", steps);
    let steps = follow_map_simultaneous(sequence, map);
    println!("{}", steps);
}
