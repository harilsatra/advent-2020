use std::fs;
use std::io::{Error};

fn main() -> Result<(), Error> {
    let path = "input.txt"; // File should be in root directory

    let contents = fs::read_to_string(path).expect("Error reading the file");

    let contents_split = contents.split("\n\n");
    let passports: Vec<&str> = contents_split.collect();

    let mut num_valid_passports = 0;

    for passport in passports.iter() {
        if is_passport_valid_part_two(passport) {
            num_valid_passports = num_valid_passports + 1;
        }
    }

    println!("{}", num_valid_passports);

    Ok(())
}

fn is_passport_valid_part_one(passport: &str) -> bool {
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

fn is_passport_valid_part_two(passport: &str) -> bool {
    let passport_split = passport.split_whitespace();
    let passport_fields: Vec<&str> = passport_split.collect();

    if passport_fields.len() < 7 {
        return false;
    }

    if passport_fields.len() > 8 {
        return false;
    }

    if passport_fields.len() == 7 {
        for field in passport_fields.iter() {
            let field_split = field.split(":");
            let v: Vec<&str> = field_split.collect();
    
            if v[0] == "cid" {
                return false;
            }
        }
    }
    
    for field in passport_fields.iter() {
        let field_split = field.split(":");
        let v: Vec<&str> = field_split.collect();

        if v[0] == "byr" {
            if !validate_year(v[1], 1920, 2002) {
                return false
            }
        }

        if v[0] == "iyr" {
            if !validate_year(v[1], 2010, 2020) {
                return false
            }
        }

        if v[0] == "eyr" {
            if !validate_year(v[1], 2020, 2030) {
                return false
            }
        }

        if v[0] == "hgt" {
            if !validate_height(v[1]) {
                return false
            }
        }

        if v[0] == "hcl" {
            if !validate_hair_color(v[1]) {
                return false
            }
        }

        if v[0] == "ecl" {
            if !validate_eye_color(v[1]) {
                return false
            }
        }

        if v[0] == "pid" {
            if !validate_passport_number(v[1]) {
                return false
            }
        }
    }

    return true;
}

fn validate_year(year_str: &str, lower_bound: i32, upper_bound: i32) -> bool {
    let year = year_str.parse::<i32>().unwrap();

    if year < lower_bound || year > upper_bound {
        return false
    }

    return true
}

fn validate_height(height_str: &str) -> bool {
    let (number_str, metric) = height_str.split_at(height_str.len() - 2);

    if metric != "cm" && metric != "in" {
        return false
    }

    let height = number_str.parse::<i32>().unwrap();

    if metric == "cm" && (height < 150 || height > 193) {
        return false
    }

    if metric == "in" && (height < 59 || height > 76) {
        return false
    }

    return true
}

fn validate_hair_color(hair_color: &str) -> bool {
    if hair_color.len() != 7 {
        return false
    }

    for (i, c) in hair_color.chars().enumerate() {
        if i == 0 && c != '#' {
            return false
        }

        if i != 0 {
            if !c.is_alphanumeric() {
                return false
            }

            if c.is_alphabetic() && c.is_uppercase() {
                return false
            }
        }
    }

    return true
}

fn validate_eye_color(eye_color: &str) -> bool {
    if eye_color == "amb" || eye_color == "blu" || eye_color == "brn" || eye_color == "gry" || eye_color == "grn" || eye_color == "hzl" || eye_color == "oth"  {
        return true
    }

    return false
}

fn validate_passport_number(passport_number_str: &str) -> bool {
    if passport_number_str.len() != 9 {
        return false
    }

    for c in passport_number_str.chars() { 
       if !c.is_numeric() {
           return false
       }
    }

    return true
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    //TODO: Make the tests table driven

    #[test]
    fn test_validate_year() {
        assert_eq!(validate_year("2002", 1920, 2002), true);
        assert_eq!(validate_year("1920", 1920, 2002), true);
        assert_eq!(validate_year("2003", 1920, 2002), false);
        assert_eq!(validate_year("1919", 1920, 2002), false);        
    }

    #[test]
    fn test_validate_height() {
        assert_eq!(validate_height("59in"), true);
        assert_eq!(validate_height("60in"), true);
        assert_eq!(validate_height("76in"), true);
        assert_eq!(validate_height("58in"), false);
        assert_eq!(validate_height("77in"), false);

        assert_eq!(validate_height("150cm"), true);
        assert_eq!(validate_height("193cm"), true);
        assert_eq!(validate_height("151cm"), true);
        assert_eq!(validate_height("49cm"), false);
        assert_eq!(validate_height("194cm"), false);

        assert_eq!(validate_height("59"), false);
    }

    #[test]
    fn test_validate_hair_color() {
        assert_eq!(validate_hair_color("#123abc"), true);
        assert_eq!(validate_hair_color("#123abz"), true);
        assert_eq!(validate_hair_color("123abz"), false);
        assert_eq!(validate_hair_color("1234abz"), false);
        assert_eq!(validate_hair_color("#123a,z"), false);
    }

    #[test]
    fn test_validate_eye_color() {
        assert_eq!(validate_eye_color("amb"), true);
        assert_eq!(validate_eye_color("blu"), true);
        assert_eq!(validate_eye_color("brn"), true);
        assert_eq!(validate_eye_color("gry"), true);
        assert_eq!(validate_eye_color("grn"), true);
        assert_eq!(validate_eye_color("hzl"), true);
        assert_eq!(validate_eye_color("oth"), true);
        assert_eq!(validate_eye_color("wat"), false);
    }

    #[test]
    fn test_validate_passport_number() {
        assert_eq!(validate_passport_number("000000001"), true);
        assert_eq!(validate_passport_number("0123456789"), false);
        assert_eq!(validate_passport_number("a12345678"), false);
    }
}