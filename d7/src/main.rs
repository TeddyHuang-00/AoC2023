use std::collections::BTreeMap;
use std::fs::File;
use std::io::Read;

fn read_lines(file_name: &str) -> Vec<String> {
    let mut file = File::open(file_name).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents.lines().map(|s| s.to_string()).collect()
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Hand {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn get_strength(ch: char, joker: bool) -> u64 {
    match ch {
        _ if ch.is_digit(10) => ch.to_digit(10).unwrap() as u64,
        'T' | 't' => 10,
        'J' | 'j' => {
            if joker {
                1
            } else {
                11
            }
        }
        'Q' | 'q' => 12,
        'K' | 'k' => 13,
        'A' | 'a' => 14,
        _ => panic!("Invalid card: {}", ch),
    }
}

fn parse_line(line: &str, joker: bool) -> (Hand, Vec<u64>, u64) {
    // Hand type, card strengths, bid value
    let parts = line.split_whitespace().collect::<Vec<_>>();
    assert!(parts.len() == 2);
    let cards = parts[0]
        .chars()
        .map(|ch| get_strength(ch, joker))
        .collect::<Vec<_>>();
    let bid = parts[1].parse::<u64>().unwrap();
    let mut card_count = BTreeMap::new();
    cards
        .iter()
        .for_each(|&card| *card_count.entry(card).or_insert(0 as u64) += 1);
    if card_count.contains_key(&1) && card_count.len() > 1 {
        // The best strategy to use the joker is to use it as the card with the most count
        // Unless the joker is the only card, then we don't need to do anything
        let num_joker = *card_count.get(&1).unwrap();
        card_count.remove(&1);
        let max_count = *card_count.values().max().unwrap();
        let max_card = *card_count
            .iter()
            .find(|(_, &count)| count == max_count)
            .unwrap()
            .0;
        *card_count.entry(max_card).or_insert(0) += num_joker;
    }
    let mut counts = vec![0 as u64; 6];
    card_count
        .iter()
        .for_each(|(_, &count)| counts[count as usize] += 1);
    let hand = match counts.as_slice() {
        [.., 1] => Hand::FiveOfAKind,
        [.., 1, 0] => Hand::FourOfAKind,
        [.., 1, 1, 0, 0] => Hand::FullHouse,
        [.., 0, 1, 0, 0] => Hand::ThreeOfAKind,
        [.., 2, 0, 0, 0] => Hand::TwoPair,
        [.., 1, 0, 0, 0] => Hand::OnePair,
        _ => Hand::HighCard,
    };

    (hand, cards, bid)
}

fn main() {
    let lines = read_lines("input.txt");
    let mut games = lines
        .iter()
        .map(|line| parse_line(&line, false))
        .collect::<Vec<_>>();
    games.sort();
    println!(
        "{:?}",
        games
            .into_iter()
            .enumerate()
            .map(|(i, (_, _, bid))| { (i + 1) as u64 * bid })
            .sum::<u64>()
    );
    let mut games = lines
        .into_iter()
        .map(|line| parse_line(&line, true))
        .collect::<Vec<_>>();
    games.sort();
    println!(
        "{:?}",
        games
            .into_iter()
            .enumerate()
            .map(|(i, (_, _, bid))| { (i + 1) as u64 * bid })
            .sum::<u64>()
    );
}
