use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

fn read_lines(file_name: &str) -> Vec<String> {
    let mut file = File::open(file_name).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents.lines().map(|s| s.to_string()).collect()
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Category {
    X,
    M,
    A,
    S,
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Status {
    Accept,
    Reject,
    Next(String),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Part {
    values: [usize; 4],
}
impl Part {
    fn new() -> Part {
        Part {
            values: [0, 0, 0, 0],
        }
    }
    fn get(&self, category: Category) -> usize {
        self.values[category as usize]
    }
    fn set(&mut self, category: Category, value: usize) {
        self.values[category as usize] = value;
    }
}

fn parse_input(
    lines: &Vec<String>,
) -> (
    HashMap<String, Vec<(Option<(Category, Ordering, usize)>, Status)>>,
    Vec<Part>,
) {
    let parts = lines.split(|s| s.is_empty()).collect::<Vec<_>>();
    assert!(parts.len() == 2);
    let workflows = parts[0]
        .iter()
        .map(|s| {
            let parts = s.split('{').collect::<Vec<_>>();
            assert!(parts.len() == 2);
            let name = parts[0].to_owned();
            let parts = parts[1]
                .strip_suffix('}')
                .unwrap()
                .split(',')
                .collect::<Vec<_>>();
            let rules = parts
                .iter()
                .map(|s| match s.contains(':') {
                    true => {
                        let parts = s.split(':').collect::<Vec<_>>();
                        assert!(parts.len() == 2);
                        let target = match parts[1] {
                            "A" => Status::Accept,
                            "R" => Status::Reject,
                            x => Status::Next(x.to_owned()),
                        };
                        for sym in "<>".chars() {
                            if parts[0].contains(sym) {
                                let parts = parts[0].split(sym).collect::<Vec<_>>();
                                assert!(parts.len() == 2);
                                let category = match parts[0] {
                                    "x" => Category::X,
                                    "m" => Category::M,
                                    "a" => Category::A,
                                    "s" => Category::S,
                                    _ => panic!("Invalid category"),
                                };
                                let order = match sym {
                                    '<' => Ordering::Less,
                                    '>' => Ordering::Greater,
                                    _ => panic!("Invalid order"),
                                };
                                let threshold = parts[1].parse::<usize>().unwrap();
                                return (Some((category, order, threshold)), target);
                            }
                        }
                        panic!("Invalid rule");
                    }
                    false => (
                        None,
                        match *s {
                            "A" => Status::Accept,
                            "R" => Status::Reject,
                            x => Status::Next(x.to_owned()),
                        },
                    ),
                })
                .collect::<Vec<_>>();
            (name, rules)
        })
        .collect::<Vec<_>>();
    let mut rules = HashMap::new();
    for (name, r) in workflows {
        rules.insert(name, r);
    }

    let parts = parts[1]
        .iter()
        .map(|s| {
            let parts = s
                .strip_prefix('{')
                .unwrap()
                .strip_suffix('}')
                .unwrap()
                .split(',')
                .collect::<Vec<_>>();
            let mut p = Part::new();
            parts.iter().for_each(|s| {
                let parts = s.split('=').collect::<Vec<_>>();
                assert!(parts.len() == 2);
                let category = match parts[0] {
                    "x" => Category::X,
                    "m" => Category::M,
                    "a" => Category::A,
                    "s" => Category::S,
                    _ => panic!("Invalid category"),
                };
                let value = parts[1].parse::<usize>().unwrap();
                p.set(category, value);
            });
            p
        })
        .collect::<Vec<_>>();
    (rules, parts)
}

fn match_rules(
    part: &Part,
    rules: &HashMap<String, Vec<(Option<(Category, Ordering, usize)>, Status)>>,
) -> bool {
    let mut name = "in".to_string();
    loop {
        let rule = rules.get(&name).unwrap();
        for (r, s) in rule {
            match r {
                Some((category, order, threshold)) => {
                    if part.get(*category).cmp(threshold) == *order {
                        match s {
                            Status::Next(next) => {
                                name = next.to_owned();
                            }
                            Status::Accept => return true,
                            Status::Reject => return false,
                        }
                        break;
                    }
                }
                None => match s {
                    Status::Next(next) => {
                        name = next.to_owned();
                        break;
                    }
                    Status::Accept => return true,
                    Status::Reject => return false,
                },
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct PartRange {
    pub min: [usize; 4],
    pub max: [usize; 4],
}
impl PartRange {
    fn new() -> PartRange {
        PartRange {
            min: [1; 4],
            max: [4000; 4],
        }
    }
    fn split(
        &self,
        category: Category,
        value: usize,
        order: Ordering,
    ) -> (Option<PartRange>, Option<PartRange>) {
        let mut matched = None;
        let mut unmatched = None;
        let (mut min, mut max) = (self.min, self.max);
        match order {
            Ordering::Greater => {
                min[category as usize] = value + 1;
                if min[category as usize] <= self.max[category as usize] {
                    matched = Some(PartRange { min, max: self.max });
                }
                max[category as usize] = value;
                if self.min[category as usize] <= max[category as usize] {
                    unmatched = Some(PartRange { min: self.min, max });
                }
            }
            Ordering::Less => {
                max[category as usize] = value - 1;
                if self.min[category as usize] <= max[category as usize] {
                    matched = Some(PartRange { min: self.min, max });
                }
                min[category as usize] = value;
                if min[category as usize] <= self.max[category as usize] {
                    unmatched = Some(PartRange { min, max: self.max });
                }
            }
            _ => panic!("Invalid order"),
        }
        (matched, unmatched)
    }
}

fn match_range_rules(
    rules: &HashMap<String, Vec<(Option<(Category, Ordering, usize)>, Status)>>,
) -> Vec<PartRange> {
    let mut queue = vec![("in".to_string(), PartRange::new())];
    let mut matches = Vec::new();
    while !queue.is_empty() {
        let (name, part) = queue.pop().unwrap();
        let mut part = part;
        let rule = rules.get(&name).unwrap();
        for (r, s) in rule {
            match r {
                Some((category, order, threshold)) => {
                    let (matched, unmatched) = part.split(*category, *threshold, *order);
                    if let Some(matched) = matched {
                        match s {
                            Status::Next(next) => {
                                queue.push((next.to_owned(), matched));
                            }
                            Status::Accept => matches.push(matched),
                            Status::Reject => {}
                        }
                    }
                    if let Some(unmacthed) = unmatched {
                        part = unmacthed;
                    }
                    if matched.is_none() && unmatched.is_none() {
                        break;
                    }
                }
                None => match s {
                    Status::Next(next) => {
                        queue.push((next.to_owned(), part));
                    }
                    Status::Accept => matches.push(part),
                    Status::Reject => {}
                },
            }
        }
    }
    matches
}

fn main() {
    let lines = read_lines("input.txt");
    let (rules, parts) = parse_input(&lines);
    let result = parts
        .iter()
        .filter(|&p| match_rules(p, &rules))
        .map(|p| p.values.iter().sum::<usize>())
        .sum::<usize>();
    println!("{}", result);
    let ranges = match_range_rules(&rules);
    println!(
        "{}",
        ranges
            .into_iter()
            .map(|r| (0..4).map(|i| r.max[i] - r.min[i] + 1).product::<usize>())
            .sum::<usize>()
    );
}
