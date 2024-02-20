use std::fs::File;
use std::io::Read;
use std::str::FromStr;
use strum::EnumString;

#[derive(Debug, PartialEq, EnumString)]
#[strum(serialize_all = "snake_case", ascii_case_insensitive)]
enum Color {
    Red,
    Green,
    Blue,
}

#[derive(Debug, Eq, Ord)]
struct Bag {
    red: u64,
    green: u64,
    blue: u64,
}

impl Bag {
    fn new() -> Bag {
        Bag {
            red: 0,
            green: 0,
            blue: 0,
        }
    }

    fn update(&mut self, color: Color, value: u64) {
        match color {
            Color::Red => self.red = value.max(self.red),
            Color::Green => self.green = value.max(self.green),
            Color::Blue => self.blue = value.max(self.blue),
        }
    }

    fn power(&self) -> u64 {
        self.red * self.green * self.blue
    }
}

impl PartialEq for Bag {
    fn eq(&self, other: &Self) -> bool {
        self.red == other.red && self.green == other.green && self.blue == other.blue
    }
}

impl PartialOrd for Bag {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.red > other.red || self.green > other.green || self.blue > other.blue {
            Some(std::cmp::Ordering::Greater)
        } else {
            Some(std::cmp::Ordering::Less)
        }
    }
}

fn read_lines(file_name: &str) -> Vec<String> {
    let mut file = File::open(file_name).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents.lines().map(|s| s.to_string()).collect()
}

fn parse_line(line: &str) -> (u64, Bag) {
    let mut bag = Bag::new();
    let parts: Vec<&str> = line.split(": ").collect();
    assert!(parts.len() == 2);
    let id = parts[0].split(" ").last().unwrap().parse::<u64>().unwrap();
    let sets = parts[1]
        .split("; ")
        .map(|s| {
            s.split(", ")
                .map(|s| {
                    let parts = s.split(" ").collect::<Vec<&str>>();
                    assert!(parts.len() == 2);
                    let color = Color::from_str(parts[1]).unwrap();
                    let value = parts[0].parse::<u64>().unwrap();
                    (color, value)
                })
                .collect::<Vec<(Color, u64)>>()
        })
        .collect::<Vec<Vec<(Color, u64)>>>();
    for set in sets {
        for (color, value) in set {
            bag.update(color, value);
        }
    }
    (id, bag)
}

fn main() {
    const TARGET: Bag = Bag {
        red: 12,
        green: 13,
        blue: 14,
    };
    let lines = read_lines("input.txt");
    let bags = lines
        .iter()
        .map(|l| parse_line(l))
        .collect::<Vec<(u64, Bag)>>();
    let possible = bags
        .iter()
        .filter(|(_, bag)| bag <= &TARGET)
        .map(|(id, _)| *id)
        .collect::<Vec<u64>>()
        .into_iter()
        .sum::<u64>();
    println!("{}", possible);
    let power = bags.iter().map(|(_, bag)| bag.power()).sum::<u64>();
    println!("{}", power);
}
