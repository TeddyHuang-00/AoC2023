use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Debug;
use std::fs::File;
use std::io::Read;

fn read_lines(file_name: &str) -> Vec<String> {
    let mut file = File::open(file_name).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents.lines().map(|s| s.to_string()).collect()
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Pulse {
    Low,
    High,
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Module {
    FlipFlop(bool),
    Conjunction(Vec<Pulse>),
    Broadcast,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Node {
    module: Module,
    src: Vec<String>,
    dst: Vec<String>,
}

fn parse_lines(lines: &Vec<String>) -> HashMap<String, Node> {
    let mut nodes = HashMap::new();
    lines.into_iter().for_each(|l| {
        let parts = l.split(" -> ").collect::<Vec<_>>();
        assert!(parts.len() == 2);
        let mut src = parts[0].to_owned();
        let dst = parts[1];
        let mut module = Module::Broadcast;
        match src.chars().next().unwrap() {
            '%' => {
                src = src[1..].to_string();
                module = Module::FlipFlop(false)
            }
            '&' => {
                src = src[1..].to_string();
                module = Module::Conjunction(vec![])
            }
            _ => {}
        }
        nodes.insert(
            src,
            Node {
                module: module,
                src: vec![],
                dst: dst.split(", ").map(|s| s.to_owned()).collect::<Vec<_>>(),
            },
        );
    });
    let nodes_copy = nodes.clone();
    nodes_copy.into_iter().for_each(|(k, v)| {
        v.dst.into_iter().for_each(|d| {
            nodes
                .entry(d)
                .and_modify(|node| {
                    node.src.push(k.clone());
                    match node.module {
                        Module::Conjunction(ref mut pulses) => pulses.push(Pulse::Low),
                        _ => {}
                    }
                })
                .or_insert(Node {
                    module: Module::Broadcast,
                    src: vec![k.clone()],
                    dst: vec![],
                });
        });
    });
    nodes
}

fn push_button(
    states: &HashMap<String, Node>,
) -> (
    HashMap<String, Node>,
    Vec<(String, String, Pulse)>,
    Vec<(String, String, Pulse)>,
) {
    let mut states = states.clone();
    let mut queue = VecDeque::from([("button".to_string(), "broadcaster".to_string(), Pulse::Low)]);
    let mut lows = Vec::new();
    let mut highs = Vec::new();
    while !queue.is_empty() {
        let (src, dst, pulse) = queue.pop_front().unwrap();
        match pulse {
            Pulse::High => highs.push((src.clone(), dst.clone(), pulse)),
            Pulse::Low => lows.push((src.clone(), dst.clone(), pulse)),
        }
        let node = states.get_mut(&dst).unwrap();
        match node.module {
            Module::FlipFlop(ref mut state) => {
                if pulse == Pulse::Low {
                    *state = !*state;
                    let sig = match *state {
                        true => Pulse::High,
                        false => Pulse::Low,
                    };
                    node.dst
                        .iter()
                        .for_each(|d| queue.push_back((dst.clone(), d.clone(), sig)));
                }
            }
            Module::Conjunction(ref mut pulses) => {
                node.src.iter().zip(pulses.iter_mut()).for_each(|(s, p)| {
                    if *s == src {
                        *p = pulse;
                    }
                });
                let sig = match pulses.iter().all(|p| *p == Pulse::High) {
                    true => Pulse::Low,
                    false => Pulse::High,
                };
                node.dst
                    .iter()
                    .for_each(|d| queue.push_back((dst.clone(), d.clone(), sig)));
            }
            Module::Broadcast => node
                .dst
                .iter()
                .for_each(|d| queue.push_back((dst.clone(), d.clone(), pulse))),
        }
    }
    (states, lows, highs)
}

fn get_insight(map: &HashMap<String, Node>) {
    // BFS
    let mut queue = VecDeque::from(["broadcaster".to_string()]);
    let mut visited = HashSet::new();
    let mut edges = Vec::new();
    while !queue.is_empty() {
        let name = queue.pop_front().unwrap();
        if visited.contains(&name) {
            continue;
        }
        visited.insert(name.clone());
        let node = map.get(&name).unwrap();
        node.dst.iter().for_each(|d| {
            queue.push_back(d.clone());
            edges.push((name.clone(), d.clone()));
        });
    }
    println!(
        "graph TD\n{}",
        edges
            .iter()
            .map(|(src, dst)| "    {src}-->{dst}"
                .replace("{src}", &src)
                .replace("{dst}", &dst))
            .collect::<Vec<_>>()
            .join("\n")
    );
}

fn main() {
    let lines = read_lines("input.txt");
    let init = parse_lines(&lines);
    let mut map = init.clone();
    let mut low_count = 0;
    let mut high_count = 0;
    for _ in 0..1_000 {
        let (new_map, lows, highs) = push_button(&map);
        map = new_map;
        low_count += lows.len();
        high_count += highs.len();
    }
    println!("{}", low_count * high_count);
    let mut map = init.clone();
    get_insight(&map); // Get mermaid graph for visualization
    let rx = map.get("rx").unwrap();
    let sources = rx.src.clone();
    assert!(sources.len() == 1); // Single source
    let src = map.get(&sources[0]).unwrap();
    assert!(matches!(src.module, Module::Conjunction { .. })); // Source is a conjunction
    let sources = src.src.clone(); // Sources of the conjunction have high periods
    let mut periods =
        HashMap::from_iter(sources.iter().map(|s| (s.clone(), 0usize))) as HashMap<String, usize>;
    let mut cnt = 0;
    while periods.values().any(|v| *v == 0) {
        let (new_map, _, highs) = push_button(&map);
        map = new_map;
        cnt += 1;
        let fired = sources
            .iter()
            .filter(|s| {
                periods.get(*s).unwrap() == &0
                    && highs.iter().filter(|(ss, _, _)| ss == *s).count() > 0
            })
            .collect::<Vec<_>>();
        fired
            .into_iter()
            .for_each(|s| drop(periods.entry(s.clone()).and_modify(|v| *v = cnt)));
    }
    println!("{}", periods.values().product::<usize>());
}
