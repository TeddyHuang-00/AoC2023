use std::fs::File;
use std::io::Read;

fn read_lines(file_name: &str) -> Vec<String> {
    let mut file = File::open(file_name).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents.lines().map(|s| s.to_string()).collect()
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Spring {
    Unknown,
    Operating,
    Damaged,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum State {
    Start,
    Match,
    End,
    Terminal,
}

fn transition(state: State, spring: Spring) -> Vec<usize> {
    // 0: Stay in the same state
    // 1: Move to the next state
    // None: Invalid transition
    // Note: The sequence of the states is always
    // N * (Start -> (k-1) * Match -> End) -> Start -> (k-1) * Match -> Terminal
    match state {
        State::Start => match spring {
            Spring::Operating => vec![0],
            Spring::Damaged => vec![1],
            Spring::Unknown => vec![0, 1],
        },
        State::Match => match spring {
            Spring::Operating => vec![],
            Spring::Damaged | Spring::Unknown => vec![1],
        },
        State::End => match spring {
            Spring::Operating | Spring::Unknown => vec![1],
            Spring::Damaged => vec![],
        },
        State::Terminal => match spring {
            Spring::Operating | Spring::Unknown => vec![0],
            Spring::Damaged => vec![],
        },
    }
}

fn parse_line(line: &String) -> (Vec<Spring>, Vec<usize>) {
    let parts = line.split(" ").collect::<Vec<_>>();
    assert!(parts.len() == 2);
    let status = parts[0]
        .chars()
        .map(|c| match c {
            '.' => Spring::Operating,
            '#' => Spring::Damaged,
            '?' => Spring::Unknown,
            _ => panic!("Invalid status"),
        })
        .collect::<Vec<_>>();
    let numbers = parts[1]
        .split(",")
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    (status, numbers)
}

fn calculate_combinitions(springs: &Vec<Spring>, numbers: &Vec<usize>) -> usize {
    let mut states = vec![];
    for n in numbers.iter() {
        states.push(State::Start);
        states.extend(vec![State::Match; *n - 1]);
        states.push(State::End);
    }
    states = vec![
        states.split_last().unwrap().1.to_owned(),
        vec![State::Terminal],
    ]
    .concat();
    let mut counter = vec![0 as usize; states.len()];
    counter[0] = 1;
    for &s in springs {
        let mut _counter = vec![0; states.len()];
        (0..states.len()).filter(|&i| counter[i] > 0).for_each(|i| {
            transition(states[i], s)
                .into_iter()
                .filter(|&di| i + di < states.len())
                .for_each(|di| _counter[i + di] += counter[i])
        });
        counter = _counter;
    }
    counter.last().unwrap().to_owned()
}

fn main() {
    let lines = read_lines("input.txt");
    let data = lines.iter().map(parse_line).collect::<Vec<_>>();
    let result = data
        .iter()
        .map(|(status, numbers)| calculate_combinitions(status, numbers))
        // .collect::<Vec<_>>();
        .sum::<usize>();
    println!("{}", result);
    let data = lines
        .iter()
        .map(parse_line)
        .map(|(status, numbers)| {
            (
                status
                    .repeat(5)
                    .chunks(status.len())
                    .fold(vec![], |acc, x| {
                        vec![acc, x.to_owned(), vec![Spring::Unknown]].concat()
                    })
                    .split_last()
                    .unwrap()
                    .1
                    .to_owned(),
                numbers.repeat(5),
            )
        })
        .collect::<Vec<_>>();
    let result = data
        .iter()
        .map(|(status, numbers)| calculate_combinitions(status, numbers))
        // .collect::<Vec<_>>();
        .sum::<usize>();
    println!("{}", result);
}
