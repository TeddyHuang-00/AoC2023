use std::fs::File;
use std::io::Read;

fn read_lines(file_name: &str) -> Vec<String> {
    let mut file = File::open(file_name).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents.lines().map(|s| s.to_string()).collect()
}

fn parse_line(line: &str) -> Vec<i64> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    parts.iter().map(|s| s.parse::<i64>().unwrap()).collect()
}

fn predict_values(sequence: &Vec<i64>) -> (i64, i64) {
    let mut diffs = Vec::new();
    diffs.push(sequence.to_owned());
    while diffs.last().unwrap().iter().any(|&x| x != 0) {
        let last = diffs.last().unwrap();
        diffs.push(
            last.iter()
                .skip(1)
                .zip(last.into_iter())
                .map(|(a, b)| a - b)
                .collect(),
        );
    }
    let mut next = 0;
    let mut prev = 0;
    while diffs.len() > 0 {
        let last = diffs.pop().unwrap();
        next += last.last().unwrap();
        prev = last.first().unwrap() - prev;
    }
    (prev, next)
}

fn main() {
    let lines = read_lines("input.txt");
    let sequences: Vec<Vec<i64>> = lines.iter().map(|l| parse_line(l)).collect();
    println!(
        "{:?}",
        sequences
            .iter()
            .map(predict_values)
            .map(|(_, l)| l)
            .sum::<i64>()
    );
    println!(
        "{:?}",
        sequences
            .iter()
            .map(predict_values)
            .map(|(f, _)| f)
            .sum::<i64>()
    );
}
