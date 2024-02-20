use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

fn read_lines(file_name: &str) -> Vec<String> {
    let mut file = File::open(file_name).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents.lines().map(|s| s.to_string()).collect()
}

fn locate_symbols(line: &str) -> Vec<(usize, char)> {
    line.to_string()
        .char_indices()
        .filter(|&(_, c)| c != '.' && !c.is_digit(10))
        .collect::<Vec<(usize, char)>>()
}

fn try_part_number(s: &str, idx: usize) -> Option<(usize, usize, u64)> {
    let mut lb = idx;
    let mut rb = idx + 1;
    if s[lb..rb].parse::<u64>().is_err() {
        return None;
    }
    while s[lb - 1..rb].parse::<u64>().is_ok() {
        lb -= 1;
        if lb == 0 {
            break;
        }
    }
    while s[lb..rb + 1].parse::<u64>().is_ok() {
        rb += 1;
        if rb == s.len() {
            break;
        }
    }
    Some((lb, rb, s[lb..rb].parse::<u64>().unwrap()))
}

fn find_part_numbers(lines: &Vec<String>) -> Vec<u64> {
    let mut map = HashMap::new();
    let mut part_numbers = Vec::new();
    for (i, line) in lines.iter().enumerate() {
        let symbol_indices = locate_symbols(&line);
        for (j, _) in symbol_indices {
            for di in -1..2 {
                for dj in -1..2 {
                    if di == 0 && dj == 0 {
                        continue;
                    }
                    let ni = i as i32 + di;
                    let nj = j as i32 + dj;
                    if ni < 0 || nj < 0 {
                        continue;
                    }
                    let ni = ni as usize;
                    let nj = nj as usize;
                    if ni >= lines.len() || nj >= lines[ni].len() {
                        continue;
                    }
                    if let Some((lb, rb, number)) =
                        try_part_number(&lines[ni as usize], nj as usize)
                    {
                        match map.get(&(ni, lb, rb)) {
                            Some(_) => continue,
                            None => {
                                map.insert((ni, lb, rb), number);
                                part_numbers.push(number);
                            }
                        }
                    }
                }
            }
        }
    }
    part_numbers
}

fn find_gears(lines: &Vec<String>) -> Vec<u64> {
    let mut gear_ratios = Vec::new();
    for (i, line) in lines.iter().enumerate() {
        let symbol_indices = locate_symbols(&line);
        for (j, ch) in symbol_indices {
            let mut map = HashMap::new();
            let mut part_numbers = Vec::new();
            if ch != '*' {
                continue;
            }
            for di in -1..2 {
                for dj in -1..2 {
                    if di == 0 && dj == 0 {
                        continue;
                    }
                    let ni = i as i32 + di;
                    let nj = j as i32 + dj;
                    if ni < 0 || nj < 0 {
                        continue;
                    }
                    let ni = ni as usize;
                    let nj = nj as usize;
                    if ni >= lines.len() || nj >= lines[ni].len() {
                        continue;
                    }
                    if let Some((lb, rb, number)) =
                        try_part_number(&lines[ni as usize], nj as usize)
                    {
                        match map.get(&(ni, lb, rb)) {
                            Some(_) => continue,
                            None => {
                                map.insert((ni, lb, rb), number);
                                part_numbers.push(number);
                            }
                        }
                    }
                }
            }
            if part_numbers.len() == 2 {
                gear_ratios.push(part_numbers.iter().product());
            }
        }
    }
    gear_ratios
}

fn main() {
    let lines = read_lines("input.txt");
    let part_numbers = find_part_numbers(&lines);
    let sum: u64 = part_numbers.iter().sum();
    println!("{}", sum);
    let gear_ratios = find_gears(&lines);
    let product: u64 = gear_ratios.iter().sum();
    println!("{}", product);
}
