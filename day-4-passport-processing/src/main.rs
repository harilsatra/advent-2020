use std::fs;
use std::io::{Error};

fn main() -> Result<(), Error> {
    let path = "input.txt"; // File should be in root directory

    let contents = fs::read_to_string(path).expect("Error reading the file");

    let contents_split = contents.split("\n\n");
    let passports: Vec<&str> = contents_split.collect();
    println!("{:?}", passports);

    let mut num_valid_passports = 0;

    for passport in passports.iter() {
        if is_passport_valid(passport) {
            num_valid_passports = num_valid_passports + 1;
        }
    }

    println!("{}", num_valid_passports);

    Ok(())
}

fn is_passport_valid(passport: &str) -> bool {
    let passport_split = passport.split_whitespace();
    let passport_fields: Vec<&str> = passport_split.collect();

    println!("{:?}", passport_fields);

    if passport_fields.len() < 7 {
        return false;
    }

    if passport_fields.len() == 8 {
        return true;
    }


    for field in passport_fields.iter() {
        let field_split = field.split(":");
        let v: Vec<&str> = field_split.collect();

        if v[0] == "cid" {
            return false;
        }
    }

    return true;
}
