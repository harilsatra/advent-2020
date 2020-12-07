use std::fs::File;
use std::io::{BufReader, BufRead, Error};
use std::collections::HashMap;

fn main() -> Result<(), Error> {
    let path = "input.txt"; // File should be in root directory

    let input = File::open(path)?;
    let buffered = BufReader::new(input); // BufReader is optimal

    let mut coins_map: HashMap<i64, bool> = HashMap::new();

    for line in buffered.lines() {
        let number_string = line?;
        let number_int = number_string.parse::<i64>().unwrap();

        coins_map.insert(number_int, true);
    }

    let mut result = 0;

    for x in 0..2020 {

        if coins_map.contains_key(&x) {
            result = two_sum(coins_map.clone(), x);

            if result != 0 {
                break;
            }
        }
    }

    println!("{}", result);

    Ok(())
}

fn two_sum(coins_map: HashMap<i64, bool>, first_num: i64) -> i64 {
    let mut result = 0;

    let look_up_number = 2020 - first_num;

    for (key, _value) in &coins_map {

        if look_up_number-key >= 0 && coins_map.contains_key(&(look_up_number-key)) {
            println!("Found it: {}, {}, {}", &(look_up_number-key), &(first_num), &(key));

            result = (look_up_number-key) * key * first_num;
            break;
        }

    }

    result
}