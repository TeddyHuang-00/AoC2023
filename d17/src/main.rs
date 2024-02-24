use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::fs::File;
use std::io::Read;

fn read_lines(file_name: &str) -> Vec<String> {
    let mut file = File::open(file_name).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents.lines().map(|s| s.to_string()).collect()
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Direction {
    Left,
    Up,
    Right,
    Down,
}

fn convert_direction(dir: Direction) -> (isize, isize) {
    match dir {
        Direction::Left => (0, -1),
        Direction::Up => (-1, 0),
        Direction::Right => (0, 1),
        Direction::Down => (1, 0),
    }
}

fn parse_input(lines: &Vec<String>) -> Vec<Vec<usize>> {
    lines
        .into_iter()
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse::<usize>().unwrap())
                .collect()
        })
        .collect()
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct State {
    pos: (usize, usize),
    direction: Direction,
    count: usize,
    heat_loss: usize,
}
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.heat_loss.cmp(&self.heat_loss)
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl State {
    fn get_possible_next_state(
        &self,
        map: &Vec<Vec<usize>>,
        min_steps: usize,
        max_steps: usize,
    ) -> Vec<State> {
        let mut next_states = vec![];
        for dir in vec![
            Direction::Left,
            Direction::Up,
            Direction::Right,
            Direction::Down,
        ] {
            // Straight line of 3, can't go straight anymore.
            // Haven't reached min_steps yet.
            // Can't go back.
            if self.direction == dir && self.count == max_steps
                || self.direction != dir
                    && (self.count < min_steps || self.direction as usize % 2 == dir as usize % 2)
            {
                continue;
            }
            let (dx, dy) = convert_direction(dir);
            let (x, y) = self.pos;
            let (nx, ny) = (x as isize + dx, y as isize + dy);
            if nx < 0 || ny < 0 || nx >= map.len() as isize || ny >= map[0].len() as isize {
                // Can't go out of bounds.
                continue;
            }
            let (nx, ny) = (nx as usize, ny as usize);
            let heat_loss = self.heat_loss + map[nx][ny];
            next_states.push(State {
                pos: (nx, ny),
                direction: dir,
                count: if self.direction == dir {
                    self.count + 1
                } else {
                    1
                },
                heat_loss,
            });
        }
        next_states
    }
}

fn minimize_heat_loss(map: &Vec<Vec<usize>>, min_steps: usize, max_steps: usize) -> Option<usize> {
    let mut visited = HashSet::new();
    let mut heap = BinaryHeap::new();
    heap.push(State {
        pos: (0, 0),
        direction: Direction::Right,
        count: 0,
        heat_loss: 0,
    });
    heap.push(State {
        pos: (0, 0),
        direction: Direction::Down,
        count: 0,
        heat_loss: 0,
    });
    while let Some(state) = heap.pop() {
        let ((r, c), heat_loss, count) = (state.pos, state.heat_loss, state.count);
        if (r, c) == (map.len() - 1, map[0].len() - 1) && count >= min_steps {
            return Some(heat_loss);
        }
        for next_state in state.get_possible_next_state(map, min_steps, max_steps) {
            // As each direction and step count at every position creates a unique state,
            // the visited set will need to be a set of tuples of position, direction, and step count.
            // Whatever comes later will have a higher heat loss.
            if !visited.contains(&(next_state.pos, next_state.direction, next_state.count)) {
                visited.insert((next_state.pos, next_state.direction, next_state.count));
                heap.push(next_state);
            }
        }
    }

    None
}

fn main() {
    let lines = read_lines("input.txt");
    let map = parse_input(&lines);
    let result = minimize_heat_loss(&map, 1, 3);
    println!("{:?}", result);
    let result = minimize_heat_loss(&map, 4, 10);
    println!("{:?}", result);
}
