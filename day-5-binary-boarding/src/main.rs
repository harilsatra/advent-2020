use std::fs::File;
use std::io::{BufReader, BufRead, Error};
use std::collections::HashMap;

fn main() -> Result<(), Error> {
    let path = "input.txt"; // File should be in root directory

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    // let mut result = 0;

    let mut seat_id_map: HashMap<i32, bool> = HashMap::new();

    for l in buffered.lines() {
        let boarding_pass = l?;

        let seat_id = get_seat_id(&boarding_pass);
        // if seat_id > result {
        //     result = seat_id;
        // }

        seat_id_map.insert(seat_id, true);
    }

    for i in 5..1024 {
        if seat_id_map.contains_key(&(i-1)) && seat_id_map.contains_key(&(i+1)) && !seat_id_map.contains_key(&(i)) {
            println!("{}", i);
        }
    }

    //println!("{}", result);

    Ok(())
}

fn get_seat_id(boarding_pass: &String) -> i32 {
    let (first, last) = boarding_pass.split_at(7);

    let row = get_row_number(&first.to_string());
    let col = get_col_number(&last.to_string());

    return (row * 8) + col;
}

fn get_row_number(row_info: &String) -> i32 {
    let mut low = 0;
    let mut high = 127;

    for c in row_info.chars() {
        let mid: i32 = (high + low) / 2;

        if c == 'F' {
            high = mid;
        } else {
            low = mid;
        }
    }

    return high;
}

fn get_col_number(col_info: &String) -> i32 {
    let mut low = 0;
    let mut high = 7;

    for c in col_info.chars() {
        let mid: i32 = (high + low) / 2;

        if c == 'L' {
            high = mid;
        } else {
            low = mid;
        }
    }

    return high;
}