use std::fs;
use std::io::{Error};

fn main() -> Result<(), Error> {
    let path = "input_test.txt"; // File should be in root directory

    let file_contents = fs::read_to_string(path).expect("Error reading the file");

    let contents_split = file_contents.split("\n\n");
    let groups: Vec<&str> = contents_split.collect();

    let mut result = 0;

    for group in groups {
        result =  result + yes_answered(&group.to_string());
    }

    println!("{}", result);

    Ok(())
}

fn yes_answered(group: &String) -> i32 {
    let mut answers: Vec<i32> = vec![0;26];

    let people: Vec<&str> = group.split_whitespace().collect();

    for person in &people {
        for c in person.chars() {
            if c.is_lowercase() && c.is_alphabetic() {
                let index = c as usize - 97;
                answers[index] = answers[index] + 1;
            }
        }
    }
    

    let mut result = 0;

    for answer in answers.iter() {
        if *answer as usize == people.len() {
            result = result + 1;
        }
    }

    return result;
}
