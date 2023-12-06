use std::fs::File;
use std::io::{self, BufRead};
//use std::path::Path;
use std::collections::HashMap;
use std::env;

fn main() {
    // get file name from command line arguments
    let args: Vec<String> = env::args().collect();
    let file_name = args.get(1).expect("Please provide a file name");

    let binding = file_name.trim();

    let mut cumulative_sum = 0;

    // open the file in read-only mode (ignoring errors)
    if let Ok(file) = File::open(&binding) {
        let reader = io::BufReader::new(file);

        // read the file line by line using the lines() iterator
        for (index, line) in reader.lines().enumerate() {
            // ignore any lines that can't be read properly
            if let Ok(line) = line {
                // println!("\n{}", line);
                if let Some((first_digit, last_digit)) = extract_first_last_digit(&line) {
                    // println!("First digit: {}, Last digit: {}", first_digit, last_digit);
                    if let Some(combined_digit) = combine_digits(first_digit, last_digit) {
                        // println!("Combined digit: {}", combined_digit);
                        cumulative_sum += combined_digit;
                    }
                } else {
                    println!("No digits found in line {}", index + 1);
                }
            }
        }
    } else {
        println!("Couldn't open file");
    }
    println!("Cumulative sum of all combined digits: {}", cumulative_sum);
}


fn extract_first_last_digit(line: &str) -> Option<(char, char)> {
    let number_word_map = get_number_word_map();
    let mut digits = Vec::new();

    // check each character in line to see if it is a digit, then check the remaining charactrs in line to see if they appear in number_word_map
    for (index, character) in line.chars().enumerate() {
        if character.is_digit(10) {
            digits.push(character);
        } else {
            let remaining_chars = &line[index..];
            for (word, digit) in number_word_map.iter() {
                if remaining_chars.starts_with(word) {
                    digits.push(*digit);
                }
            }
        }
    }

    match digits.len() {
        0 => None,
        1 => Some((digits[0], digits[0])),
        _ => Some((digits[0], *digits.last().unwrap())),
    }
}


fn combine_digits(first_digit: char, last_digit: char) -> Option<u32> {
    let combined_str = format!("{}{}", first_digit, last_digit);
    combined_str.parse::<u32>().ok()
}


fn get_number_word_map() -> HashMap<&'static str, char> {
    let mut map = HashMap::new();
    map.insert("zero", '0');
    map.insert("one", '1');
    map.insert("two", '2');
    map.insert("three", '3');
    map.insert("four", '4');
    map.insert("five", '5');
    map.insert("six", '6');
    map.insert("seven", '7');
    map.insert("eight", '8');
    map.insert("nine", '9');
    map
}

