use std::fs::File;
use std::io::{BufReader, BufRead, Error};

fn main() -> Result<(), Error> {
    let path = "input.txt"; // File should be in root directory

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let mut valid_count = 0;

    for l in buffered.lines() {
        let line = l?;

        let split = line.split_whitespace();
        let vec: Vec<&str> = split.collect();

        let (lower_bound, upper_bound) = get_lower_and_upper_bound(vec[0].to_string());
        let letter = get_letter(vec[1].to_string());

        let is_valid = is_password_valid_part2(lower_bound, upper_bound, letter, vec[2].to_string());

        if !is_valid {
            println!("{}", line);
        } else {
            valid_count = valid_count + 1;
        }   
    }

    println!("{}", valid_count);

    Ok(())
}

fn get_lower_and_upper_bound(range: String) -> (usize, usize) {
    let split = range.split("-");
    let vec: Vec<&str> = split.collect();

    let lower_bound = vec[0].parse::<usize>().unwrap();
    let upper_bound = vec[1].parse::<usize>().unwrap();

    (lower_bound, upper_bound)
}

fn get_letter(str: String) -> char {
    let mut letter = ' ';

    for (i, c) in str.chars().enumerate() {
        if i == 0 {
            letter = c;
            break;
        }
    }

    letter
}

fn is_password_valid_part1(lower_bound: usize, upper_bound: usize, letter: char, password: String) -> bool {
    let mut letter_count: usize = 0;

    for c in password.chars() { 
        if c == letter {
            letter_count = letter_count + 1;
        }
    }

    let mut is_valid = false;

    if letter_count >= lower_bound && letter_count <= upper_bound {
        is_valid = true;
    }

    is_valid

}

fn is_password_valid_part2(first_pos: usize, second_pos: usize, letter: char, password: String) -> bool {
    let mut is_valid = false;

    for (i, c) in password.chars().enumerate() {
        if i == first_pos - 1 && c == letter {
            is_valid = !is_valid
        }

        if i == second_pos - 1 && c == letter {
            is_valid = !is_valid
        }
    }

    is_valid
}
