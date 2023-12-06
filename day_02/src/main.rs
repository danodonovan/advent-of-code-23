use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};


struct Grab { 
    colours: HashMap<String, u32>,
}


struct Game {
    game: u32,
    grabs: Vec<Grab>,
    line: String,
}


fn main() {
    // get file name from command line arguments
    let args: Vec<String> = env::args().collect();
    let file_name = args.get(1).expect("Please provide a file name");

    println!("Hello, world!");
    println!("File name: {}", file_name);

    read_file(file_name);
}

// read a file line by line
fn read_file(file_name: &str) {
    let mut game_id_sum = 0;

    // open the file in read-only mode
    if let Ok(file) = File::open(file_name) {
        let reader = io::BufReader::new(file);

        // read the file line by line using the lines() iterator
        for (index, line) in reader.lines().enumerate() {
            // ignore any lines that can't be read properly
            if let Ok(line) = line {
                //println!("{}. {}", index + 1, line);
                let game = parse_game(&line);
                // println!("{} {}", game.game, game.grabs.len());
                if possible_game(&game) {
                    //println!("Game {} is possible", game.game);
                    //println!("{}", game.line);
                    game_id_sum += game.game;   
                } else {
                    // println!("Game {} is not possible", game.game);
                    // println!("{}\n", game.line);
                }
            }
        }
    } else {
        println!("Couldn't open file");
    }
    println!("Sum of game IDs of possible games: {}", game_id_sum);
}

fn parse_game(game_str: &str) -> Game {
    let parts: Vec<&str> = game_str.split(": ").collect();
    let game_number = parts[0][5..].parse::<u32>().unwrap();
    let grabs_str = parts[1];

    let mut grabs = Vec::new();

    for grab_str in grabs_str.split("; ") {
        let mut colours = HashMap::new();

        for colour_count_str in grab_str.split(", ") {
            let colour_count: Vec<&str> = colour_count_str.split(" ").collect();
            let count = colour_count[0].parse::<u32>().unwrap();
            let colour = colour_count[1].to_string();
            colours.insert(colour, count);
        }

        grabs.push(Grab { colours: colours });
    }

    Game { game: game_number, grabs: grabs, line: game_str.to_string() }
}


fn possible_game(game: &Game) -> bool {
    let mut possible = true;
    // let mut red_sum = 0;
    // let mut blue_sum = 0;
    // let mut green_sum = 0;

    for grab in &game.grabs {
        for (colour, count) in &grab.colours {
            if *colour == "red"{ 
                if count > &12 {
                    return false;
                }
            }
            else if *colour == "green" {
                if count > &13 {
                    return false;
                }
            }
            else if *colour == "blue" {
                if count > &14 {
                    return false;
                }
            }
            else {
                println!("Unknown colour: {}", colour);
            }
        }
    }
    return true; 
    // for grab in &game.grabs {
    //     for (colour, count) in &grab.colours {
    //         if *colour == "red" {
    //             red_sum += count;
    //         }
    //         else if *colour == "green" {
    //             green_sum += count;
    //         }
    //         else if *colour == "blue" {
    //             blue_sum += count;
    //         }
    //         else {
    //             println!("Unknown colour: {}", colour);
    //         }
    //     }
    // }

    // println!("Game {}: Red: {}, Green: {}, Blue: {}", game.game, red_sum, green_sum, blue_sum);
    // possible && red_sum <= 12 && green_sum <= 13 && blue_sum <= 14
}