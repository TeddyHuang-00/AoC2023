use std::fs::File;
use std::io::Read;

fn read_lines(file_name: &str) -> Vec<String> {
    let mut file = File::open(file_name).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents.lines().map(|s| s.to_string()).collect()
}

fn parse_input(line: &String) -> Vec<String> {
    line.split(",").map(|s| s.to_string()).collect()
}

fn hash(s: &String) -> u8 {
    s.chars()
        .fold(0, |acc, c| ((acc as u16 + c as u16) * 17) as u8)
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Lens {
    pub label: String,
    pub focal: u8,
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Instruction {
    Remove(usize, Lens),
    Set(usize, Lens),
}

fn parse_instruction(s: &String) -> Instruction {
    if s.contains('=') {
        let parts = s.split("=").collect::<Vec<_>>();
        let label = parts[0].to_string();
        let focal = parts[1].parse::<u8>().unwrap();
        Instruction::Set(hash(&label) as usize, Lens { label, focal })
    } else {
        let label = s.replace("-", "");
        Instruction::Remove(hash(&label) as usize, Lens { label, focal: 0 })
    }
}

fn apply_instructions(instructions: Vec<Instruction>) -> Vec<Vec<Lens>> {
    let mut boxes = vec![vec![]; 256] as Vec<Vec<Lens>>;
    instructions
        .iter()
        .for_each(|instruction| match instruction {
            Instruction::Set(hash, lens) => {
                let index = boxes[*hash].iter().position(|l| l.label == lens.label);
                match index {
                    Some(index) => boxes[*hash][index] = lens.clone(),
                    None => boxes[*hash].push(lens.clone()),
                }
            }
            Instruction::Remove(hash, lens) => {
                let index = boxes[*hash].iter().position(|l| l.label == lens.label);
                match index {
                    Some(index) => drop(boxes[*hash].remove(index)),
                    _ => {}
                }
            }
        });
    boxes
}

fn main() {
    let lines = read_lines("input.txt");
    let input = parse_input(&lines[0]);
    let hashes = input.iter().map(|s| hash(s)).collect::<Vec<_>>();
    println!("{}", hashes.iter().map(|&h| h as u32).sum::<u32>());
    let instructions = input
        .iter()
        .map(|s| parse_instruction(&s))
        .collect::<Vec<_>>();
    let boxes = apply_instructions(instructions);
    println!(
        "{}",
        boxes
            .iter()
            .enumerate()
            .map(|(b, ls)| (b + 1)
                * ls.iter()
                    .enumerate()
                    .map(|(i, l)| (i + 1) * l.focal as usize)
                    .sum::<usize>())
            .sum::<usize>()
    );
}
