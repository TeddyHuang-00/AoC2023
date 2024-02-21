use std::fs::File;
use std::io::Read;

fn read_lines(file_name: &str) -> Vec<String> {
    let mut file = File::open(file_name).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents.lines().map(|s| s.to_string()).collect()
}

fn divide_maps(lines: Vec<String>) -> Vec<Vec<String>> {
    let mut chunks = Vec::new();
    chunks.push(Vec::new());
    for line in lines {
        match line.as_str() {
            "" => {
                chunks.push(Vec::new());
            }
            _ => {
                let last = chunks.len() - 1;
                chunks[last].push(line);
            }
        }
    }
    chunks
}

fn apply_maps(maps: &Vec<Vec<String>>) -> Vec<u64> {
    let seeds = maps[0][0]
        .split(" ")
        .skip(1)
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    let mut result = seeds.clone();
    for m in maps[1..].iter() {
        let mut flags = vec![false; seeds.len()];
        for line in m[1..].iter() {
            let parts = line.split(" ").collect::<Vec<_>>();
            assert!(parts.len() == 3);
            let parts = parts
                .into_iter()
                .map(|s| s.parse::<u64>().unwrap())
                .collect::<Vec<_>>();
            let (dst, src, len) = (parts[0], parts[1], parts[2]);
            result
                .iter_mut()
                .enumerate()
                .for_each(|(i, x)| match flags[i] {
                    true => {}
                    false => {
                        if *x >= src && *x < src + len {
                            *x -= src;
                            *x += dst;
                            flags[i] = true;
                        }
                    }
                });
        }
    }
    result
}

fn determine_intersection(
    s: u64,
    l: u64,
    src: u64,
    len: u64,
) -> (Vec<(u64, u64)>, Vec<(u64, u64)>) {
    let e = s + l;
    let end = src + len;
    if s >= end || e <= src {
        // Outside the range src..src+len
        (vec![], vec![(s, l)])
    } else if s <= src && e >= end {
        // The range src..src+len is inside the original range
        (
            vec![(src, len)],
            vec![
                if s < src { vec![(s, src - s)] } else { vec![] },
                if e > end {
                    vec![(end, e - end)]
                } else {
                    vec![]
                },
            ]
            .concat(),
        )
    } else if s <= src {
        // The left part of src..src+len intersects with the right part of original range
        (
            vec![(src, e - src)],
            if s < src { vec![(s, src - s)] } else { vec![] },
        )
    } else if e >= end {
        // The right part of src..src+len intersects with the left part of original range
        (
            vec![(s, end - s)],
            if e > end {
                vec![(end, e - end)]
            } else {
                vec![]
            },
        )
    } else if s > src && e < end {
        // The original range is inside the range src..src+len
        (vec![(s, l)], vec![])
    } else {
        panic!(
            "Unexpected case: s={}, l={}, src={}, len={}",
            s, l, src, len
        );
    }
}

fn apply_range_maps(maps: &Vec<Vec<String>>) -> Vec<(u64, u64)> {
    let seeds = maps[0][0]
        .split(" ")
        .skip(1)
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    let seeds = seeds
        .iter()
        .step_by(2)
        .zip(seeds.iter().skip(1).step_by(2))
        .map(|(x, y)| (*x, *y))
        .collect::<Vec<_>>();
    let mut unmapped = seeds.clone();
    let mut mapped = Vec::new();
    for m in maps[1..].iter() {
        for line in m[1..].iter() {
            let parts = line.split(" ").collect::<Vec<_>>();
            assert!(parts.len() == 3);
            let parts = parts
                .into_iter()
                .map(|s| s.parse::<u64>().unwrap())
                .collect::<Vec<_>>();
            let (dst, src, len) = (parts[0], parts[1], parts[2]);
            let mut unmatched = Vec::new();
            unmapped.iter().for_each(|&(s, l)| {
                let (ma, um) = determine_intersection(s, l, src, len);
                mapped.extend(ma.into_iter().map(|(s, l)| (s + dst - src, l)));
                unmatched.extend(um);
            });
            unmapped = unmatched;
        }
        mapped.extend(unmapped);
        unmapped = Vec::new();
        // Concat continuous segments
        mapped.sort();
        mapped.into_iter().for_each(|(rs, rl)| {
            if unmapped.len() > 0 {
                let (ls, ll) = unmapped.last_mut().unwrap();
                if *ls + *ll == rs {
                    *ll += rl;
                } else {
                    unmapped.push((rs, rl));
                }
            } else {
                unmapped.push((rs, rl));
            }
        });
        mapped = Vec::new();
    }
    unmapped
}

fn main() {
    let lines = read_lines("input.txt");
    let maps = divide_maps(lines);
    let result = apply_maps(&maps);
    println!("{}", result.iter().reduce(|x, y| x.min(y)).unwrap());
    let result = apply_range_maps(&maps);
    println!("{}", result.first().unwrap().0);
}
