use std::collections::{HashSet, HashMap};
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Card {
    card_id: u32,
    winning_numbers: HashSet<u32>,
    numbers: HashSet<u32>,
}

fn read_cards_from_file(filename: &str) -> io::Result<Vec<Card>> {
    let path = Path::new(filename);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut cards = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let card_parts: Vec<&str> = line.split(": ").collect();
        let card_id_parts: Vec<&str> = card_parts[0].split_whitespace().collect();
        let card_id = card_id_parts[1].parse::<u32>().unwrap();

        let parts: Vec<&str> = line.split("|").collect();

        if parts.len() != 2 {
            continue;
        }

        let right_parts: HashSet<u32> = parts[1]
            .split_whitespace()
            .filter_map(|s| s.parse::<u32>().ok())
            .collect();

        let winning_numbers: HashSet<u32> = parts[0][2..]
            .split_whitespace()
            .filter_map(|s| s.parse::<u32>().ok())
            .collect();

        cards.push(Card {
            card_id,
            winning_numbers,
            numbers: right_parts,
        });
    }

    Ok(cards)
}


fn main() {
    // get file name from command line arguments
    let args: Vec<String> = env::args().collect();
    let file_name = args.get(1).expect("Please provide a file name");

    let cards = read_cards_from_file(file_name).unwrap_or_else(|err| {
        println!("Error reading file: {}", err);
        vec![]
    });

    let mut cumulative_score = 0;
    for card in cards.iter() {
        // println!("Card {}: {:?} | {:?}", card.card_id, card.winning_numbers, card.numbers);

        let intersection_size = card.winning_numbers.intersection(&card.numbers).count();

        let mut score = 0;
        if intersection_size > 0 {
            score = 2u32.pow(intersection_size as u32 - 1);
        }

        // println!("{}: {} - {}\n", card.card_id, intersection_size, score);
        cumulative_score += score;
    }
    println!("P1 final score: {}", cumulative_score);

    // part II
    let mut card_stack = HashMap::new();

    for card in cards.iter() {
        card_stack.insert(card.card_id, 1);
    }

    for card in cards.iter() {
        let intersection_size = card.winning_numbers.intersection(&card.numbers).count();
        // account for copies
        let current_card_score = *card_stack.get(&card.card_id).unwrap_or(&0);

        for _j in 0..current_card_score {
            for i in 1..(intersection_size + 1)  {
                let key = card.card_id + i as u32;
                let card_score = *card_stack.get(&key).unwrap_or(&0);
                card_stack.insert(key, card_score + 1);
            }
        }
    }

    let sum: u32 = card_stack.values().sum();
    println!("P2 Sum of card_stack values: {}", sum);
}
