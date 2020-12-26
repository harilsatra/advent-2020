use std::fs::File;
use std::io::{BufReader, BufRead, Error};

fn main() -> Result<(), Error> {
    let path = "input.txt"; // File should be in root directory

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let mut two_d_array: Vec<Vec<char>> = vec![];

    for l in buffered.lines() {
        let line = l?;

        let v: Vec<char> = line.chars().collect();
        two_d_array.push(v);
    }

    let result: i64 = no_trees_encountered(&two_d_array, 1, 3) * no_trees_encountered(&two_d_array, 1, 1) * no_trees_encountered(&two_d_array, 1, 5) * no_trees_encountered(&two_d_array, 1, 7) * no_trees_encountered(&two_d_array, 2, 1);

    println!("{:?}", result);

    Ok(())
}

fn no_trees_encountered(two_d_array: &Vec<Vec<char>>, row_slope: usize, col_slope: usize) -> i64 {
    let mut result: i64 = 0;

    let col_size = two_d_array[0].len();

    let mut row = row_slope;
    let mut col = col_slope;

    while row < two_d_array.len() {
        if col >= col_size {
            col = col % col_size;
        }

        if two_d_array[row][col] == '#' {
            result =  result + 1;
        }

        row =  row + row_slope;
        col = col + col_slope;
    } 

    return result;
}
