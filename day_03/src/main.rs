use std::collections::HashSet;
use std::io::{self, BufRead};
use std::fs::File;
use std::env;


struct Word {
    text: String,
    coordinates: Vec<(usize, usize)>,
    keep: bool,
}


struct Gear {
    coordinate: (usize, usize),
    words: HashSet<String>,
}


fn main() {
    // get file name from command line arguments
    let args: Vec<String> = env::args().collect();
    let file_name = args.get(1).expect("Please provide a file name");

    let grid = read_file(file_name);
    print_grid(&grid);

    let word = Word{ text: "".to_string(), coordinates: Vec::new(), keep: false };
    let mut words = vec![word];
    let mut coordinates = Vec::new();
    for (row_index, row) in grid.iter().enumerate() {
        for (column_index, character) in row.iter().enumerate() {
            if character.is_ascii_digit() {
                if let Some(word) = words.last_mut() {
                    word.text.push(*character);
                    word.coordinates.push((row_index, column_index));
                }
            } else {
                coordinates.push((row_index, column_index));
                let word = Word{ text: "".to_string(), coordinates: Vec::new(), keep: false };
                words.push(word);
            }
        }
    }
   
    // filter words so that any words with text length 0 are dropped    
    words.retain(|word| !word.text.is_empty());

    for word_ix in 0..words.len() {
        let word = &mut words[word_ix];
        println!("{}", word.text);
        let adjacent_characters = get_adjacent_characters(&grid, &word.coordinates);
        
        for (index, adjacent) in adjacent_characters.iter().enumerate() {
            print!("{} ({}, {}): ", word.text.chars().nth(index).unwrap(), coordinates[index].0, coordinates[index].1);
            for character in adjacent.iter() {
                print!("{}", character);
                if !character.is_ascii_digit() && *character != '.' {
                    word.keep = true;
                }
            }
            println!();
        }
        println!();
    }

    // for word in words find the cumulative sum of the text as a number if keep is true
    let mut cumulative_sum = 0;
    for word in words.iter() {
        if word.keep {
            let number = word.text.parse::<u32>().unwrap();
            cumulative_sum += number;
        }
    } 

    println!("Cumulative sum of all numbers: {}", cumulative_sum);

    // part b

    let mut gears = Vec::<Gear>::new();
    for (row_index, row) in grid.iter().enumerate() {
        for (column_index, character) in row.iter().enumerate() {
            // check if character is "*" character
            if *character == '*' {
                // create Gear struct and add it to gears vector
                let gear = Gear{ coordinate: (row_index, column_index), words: HashSet::new() };
                gears.push(gear);
            }
        }
    }

    // for word in words check if any of the coordinates are in the same row or column as any of the gears
    for word in words.iter() {
        for gear in gears.iter_mut() {
            for coordinate in word.coordinates.iter() {
                for adjacent in get_adjacent_coordinates(&grid, coordinate).iter() {
                    if adjacent.0 == gear.coordinate.0 && adjacent.1 == gear.coordinate.1 {
                        gear.words.insert(word.text.clone());
                    }
                }
            }
        }
    }

    for gear in gears.iter() {
        println!("({}, {}): {:?}", gear.coordinate.0, gear.coordinate.1, gear.words);
    }

    // for gear in gears, if len gear.words is 2 then mutiply the two numbers together and add to cumulative sum
    let mut cumulative_product = 0;
    for gear in gears.iter() {
        if gear.words.len() == 2 {
            let mut product = 1;
            for word in gear.words.iter() {
                let number = word.parse::<u32>().unwrap();
                product *= number;
            }
            cumulative_product += product;
        }
        if gear.words.len() > 2 {
            println!("More than 2 words: {:?}", gear.words);
        }
    }
    println!("Cumulative product of all numbers: {}", cumulative_product);
    // 82454502 is too low
}


// read file as 2d vector of characters
fn read_file(file_name: &str) -> Vec<Vec<char>> {
    let mut grid = Vec::new();

    // open the file in read-only mode
    if let Ok(file) = File::open(file_name) {
        let reader = io::BufReader::new(file);

        // read the file line by line using the lines() iterator
        for line in reader.lines() {
            // ignore any lines that can't be read properly
            if let Ok(line) = line {
                let mut row = Vec::new();
                for character in line.chars() {
                    row.push(character);
                }
                grid.push(row);
            }
        }
    } else {
        println!("Couldn't open file");
    }
    grid
}

// print each row of the grid
fn print_grid(grid: &Vec<Vec<char>>) {
    println!("analysing...\n");
    for row in grid.iter() {
        for character in row.iter() {
            print!("{}", character);
        }
        println!();
    }
    println!();
}

// function which gets the adjacent coordinates for a given set of coordinates in the grid
fn get_adjacent_coordinates(grid: &Vec<Vec<char>>, coordinates: &(usize, usize)) -> Vec<(usize, usize)> {
    let (row_index, column_index) = coordinates;
    let mut adjacent_coordinates = Vec::new();

    // above
    if *row_index > 0 {
        adjacent_coordinates.push((row_index - 1, *column_index));
    }
    // below
    if *row_index < grid.len() - 1 {
        adjacent_coordinates.push((row_index + 1, *column_index));
    }
    // left
    if *column_index > 0 {
        adjacent_coordinates.push((*row_index, column_index - 1));
    }
    // right
    if *column_index < grid[*row_index].len() - 1 {
        adjacent_coordinates.push((*row_index, column_index + 1));
    }
    // diagonal
    if *row_index > 0 && *column_index > 0 {
        adjacent_coordinates.push((*row_index - 1, column_index - 1));
    }
    if *row_index > 0 && *column_index < grid[*row_index].len() - 1 {
        adjacent_coordinates.push((*row_index - 1, column_index + 1));
    }
    if *row_index < grid.len() - 1 && *column_index > 0 {
        adjacent_coordinates.push((*row_index + 1, column_index - 1));
    }
    if *row_index < grid.len() - 1 && *column_index < grid[*row_index].len() - 1 {
        adjacent_coordinates.push((*row_index + 1, column_index + 1));
    }

    adjacent_coordinates
}



fn get_adjacent_characters(grid: &Vec<Vec<char>>, coordinates: &Vec<(usize, usize)>) -> Vec<Vec<char>> {
    let mut adjacent_characters = Vec::new();
    for coordinate in coordinates.iter() {
        let mut adjacent = Vec::new();
        let (row_index, column_index) = coordinate;
        // above
        if *row_index > 0 {
            adjacent.push(grid[row_index - 1][*column_index]);
        }
        // below
        if *row_index < grid.len() - 1 {
            adjacent.push(grid[row_index + 1][*column_index]);
        }
        // left
        if *column_index > 0 {
            adjacent.push(grid[*row_index][column_index - 1]);
        }
        // right
        if *column_index < grid[*row_index].len() - 1 {
            adjacent.push(grid[*row_index][column_index + 1]);
        }
        // diagonal
        if *row_index > 0 && *column_index > 0 {
            adjacent.push(grid[row_index - 1][column_index - 1]);
        }
        if *row_index > 0 && *column_index < grid[*row_index].len() - 1 {
            adjacent.push(grid[row_index - 1][column_index + 1]);
        }
        if *row_index < grid.len() - 1 && *column_index > 0 {
            adjacent.push(grid[row_index + 1][column_index - 1]);
        }
        if *row_index < grid.len() - 1 && *column_index < grid[*row_index].len() - 1 {
            adjacent.push(grid[row_index + 1][column_index + 1]);
        }
        adjacent_characters.push(adjacent);
    }
    adjacent_characters
}