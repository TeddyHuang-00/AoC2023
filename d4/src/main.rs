use std::collections::HashSet;
use std::fs::File;
use std::io::Read;

fn read_lines(file_name: &str) -> Vec<String> {
    let mut file = File::open(file_name).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents.lines().map(|s| s.to_string()).collect()
}

fn parse_line(line: &str) -> u64 {
    let parts: Vec<&str> = line.split(": ").collect();
    assert!(parts.len() == 2);
    let parts: Vec<&str> = parts[1].split(" | ").collect();
    assert!(parts.len() == 2);
    let targets = parts[0]
        .split(" ")
        .map(|s| s.parse::<u64>())
        .filter(|r| r.is_ok())
        .map(|r| r.unwrap())
        .collect::<HashSet<u64>>();
    let numbers = parts[1]
        .split(" ")
        .map(|s| s.parse::<u64>())
        .filter(|r| r.is_ok())
        .map(|r| r.unwrap())
        .collect::<Vec<u64>>();
    let mut result: u64 = 0;
    for &x in &numbers {
        match targets.contains(&x) {
            true => result += 1,
            false => {}
        }
    }
    result
}

fn number_of_cards(values: Vec<u64>) -> Vec<u64> {
    let mut counts = vec![1; values.len()];
    for i in 0..values.len() {
        let v = values[i];
        let c = counts[i];
        for j in 0..v as usize {
            if (i + j + 1) >= values.len() {
                break;
            }
            counts[i + j + 1] += c;
        }
    }
    counts
}

fn main() {
    let lines = read_lines("input.txt");
    let values = lines.iter().map(|s| parse_line(s)).collect::<Vec<u64>>();
    println!(
        "{}",
        values
            .iter()
            .map(|x| if *x > 0 { 1 << (x - 1) } else { 0 })
            .sum::<u64>()
    );
    let numbers = number_of_cards(values);
    println!("{}", numbers.iter().sum::<u64>());
}
