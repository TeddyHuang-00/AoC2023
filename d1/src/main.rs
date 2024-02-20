use std::fs::File;
use std::io::Read;
use std::str::FromStr;
use strum::EnumString;

fn read_file(file_name: &str) -> Vec<String> {
    let mut file = File::open(file_name).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents.lines().map(|s| s.to_string()).collect()
}

fn parse_line(line: &str) -> u64 {
    let digits = line.chars().filter(|c| c.is_digit(10)).collect::<String>();
    let first = digits.chars().next().unwrap();
    let last = digits.chars().last().unwrap();
    let value = (first.to_string() + &last.to_string())
        .parse::<u64>()
        .unwrap();
    value
}

#[derive(Debug, PartialEq, EnumString)]
#[strum(serialize_all = "snake_case", ascii_case_insensitive)]
enum StringNumber {
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
}

fn parse_line_2(line: &str) -> u64 {
    let try_parse = |s: String| match StringNumber::from_str(s.as_str()) {
        Ok(v) => Some(v as u64),
        Err(_) => None,
    };
    let digits = line
        .char_indices()
        .map(|(i, ch)| match ch.is_digit(10) {
            true => Some(ch.to_string()),
            false => {
                let r3 = if line.len() >= i + 3 {
                    try_parse(line[i..i + 3].to_string())
                } else {
                    None as Option<u64>
                };
                let r4 = if line.len() >= i + 4 {
                    try_parse(line[i..i + 4].to_string())
                } else {
                    None as Option<u64>
                };
                let r5 = if line.len() >= i + 5 {
                    try_parse(line[i..i + 5].to_string())
                } else {
                    None as Option<u64>
                };

                match r5 {
                    Some(v) => Some(v.to_string()),
                    None => match r4 {
                        Some(v) => Some(v.to_string()),
                        None => match r3 {
                            Some(v) => Some(v.to_string()),
                            None => None,
                        },
                    },
                }
            }
        })
        .filter(|s| s.is_some())
        .map(|s| s.unwrap())
        .collect::<String>();
    let first = digits.chars().next().unwrap();
    let last = digits.chars().last().unwrap();
    let value = (first.to_string() + &last.to_string())
        .parse::<u64>()
        .unwrap();
    value
}

fn main() {
    let input = read_file("input.txt");
    println!(
        "{:?}",
        input.iter().map(|l| parse_line(l.as_str())).sum::<u64>()
    );
    println!(
        "{:?}",
        input.iter().map(|l| parse_line_2(l.as_str())).sum::<u64>()
    );
}
