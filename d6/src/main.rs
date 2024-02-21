use std::fs::File;
use std::io::Read;

fn read_lines(file_name: &str) -> Vec<String> {
    let mut file = File::open(file_name).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents.lines().map(|s| s.to_string()).collect()
}

fn parse_input(lines: Vec<String>) -> Vec<(u64, u64)> {
    assert!(lines.len() == 2);
    let parse_line = |s: &str| {
        s.split(":")
            .last()
            .unwrap()
            .split(" ")
            .map(|s| s.parse::<u64>())
            .filter(|r| r.is_ok())
            .map(|r| r.unwrap())
            .collect::<Vec<_>>()
    };
    let time = parse_line(lines[0].as_str());
    let distance = parse_line(lines[1].as_str());
    return time.into_iter().zip(distance.into_iter()).collect();
}

fn winning_strategies(records: Vec<(u64, u64)>) -> Vec<u64> {
    records
        .into_iter()
        .map(|(t, d)| {
            let i = (0..t / 2).find(|i| i * (t - i) > d).unwrap();
            t - 2 * i + 1
        })
        .collect()
}

fn winning_strategies_bad_kerning(records: Vec<(u64, u64)>) -> u64 {
    let mut time = Vec::new();
    let mut distance = Vec::new();
    records.into_iter().for_each(|(t, d)| {
        time.push(t.to_string());
        distance.push(d.to_string());
    });
    let time = time.concat().parse::<u64>().unwrap();
    let distance = distance.concat().parse::<u64>().unwrap();
    let mut lb = 0;
    let mut rb = time / 2;
    while rb - lb > 1 {
        let pivot = (lb + rb) / 2;
        if pivot * (time - pivot) > distance {
            rb = pivot;
        } else {
            lb = pivot + 1;
        }
    }
    assert!(lb * (time - lb) > distance);
    assert!((lb - 1) * (time - lb + 1) <= distance);
    time - 2 * lb + 1
}

fn main() {
    let lines = read_lines("input.txt");
    let records = parse_input(lines);
    let result = winning_strategies(records.clone()).into_iter().product::<u64>();
    println!("{}", result);
    let result = winning_strategies_bad_kerning(records);
    println!("{}", result);
}
