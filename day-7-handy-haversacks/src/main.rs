use std::fs::File;
use std::io::{BufReader, BufRead, Error};
use std::collections::HashMap;

#[derive(Clone)]
struct Node {
    count: i32,
    name: String,
}

impl Node {
    fn new(count: i32, name: String) -> Self {
        Self { count, name }
    }
}

fn main() -> Result<(), Error> {
    let path = "input.txt";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let mut rule_map: HashMap<String, Vec<Node>> = HashMap::new();

    for l in buffered.lines() {
        let rule = l?;

       let (bag_name, contents) = get_bag_name_and_contents(rule);

        rule_map.insert(bag_name,contents);
    }

    let result_part1 = get_num_bags(&rule_map);
    println!("{:?}", result_part1);

    // The result includes the shiny gold bag so subtract 1
    let result_part2 = bags_required_inside_a_bag(&rule_map, &"shiny gold".to_string()) - 1;
    println!("{:?}", result_part2);

    Ok(())
}

fn get_bag_name_and_contents(rule: String) -> (String, Vec<Node>) {
    let words: Vec<&str> = rule.split_whitespace().collect();

    let mut bag_items: Vec<Node> = vec![];

    let mut name = "".to_owned();
    let mut index = 0;
    for (i, word) in words.iter().enumerate() {
        if word == &"contain" {
            index = i;
            break;
        }
        if *word == "bags" {
            continue;
        }
        if i == 0 {
            name.push_str(word);
        } else {
            name.push_str(" ");
            name.push_str(word);
        }
    }

    if words[index + 1] == "no" {
        return (name.to_string(),vec![]);
    }

    for (i, word) in words.iter().enumerate() {
        if i <= index {
            continue
        }

        let number: i32;

        // or, to be safe, match the `Err`
        match word.parse::<i32>() {
            Ok(n) => number = n,
            Err(_e) => continue,
        }

        if number != 0 {
            let mut bag_item: String = "".to_owned();

            bag_item.push_str(words[i+1]);
            bag_item.push_str(" ");
            bag_item.push_str(words[i+2]);

            bag_items.push(Node::new(number, bag_item));
        }
    }

    return (name.to_string(),bag_items) 
}

fn get_num_bags(rule_map: &HashMap<String, Vec<Node>>) -> i32 {
    let mut result = 0;
    for (k, _v) in rule_map {
        if can_fit_shiny_gold(rule_map, k) {
            result = result + 1;
        }
    }

    return result;
} 

fn can_fit_shiny_gold(rule_map: &HashMap<String, Vec<Node>>, bag: &String) -> bool {
    let bag_items: Vec<Node>;

    match rule_map.get(bag) {
        Some(items) => bag_items = items.to_vec(),
        None => return false
    }

    for item in bag_items.iter() {
        if item.name == "shiny gold" {
            return true;
        }

        if can_fit_shiny_gold(rule_map, &item.name) {
            return true
        }
    }

    return false
}

fn bags_required_inside_a_bag(rule_map: &HashMap<String, Vec<Node>>, bag: &String) -> i32 {
    let bag_items: Vec<Node>;

    match rule_map.get(bag) {
        Some(items) => bag_items = items.to_vec(),
        None => return 0
    }

    let mut result = 1;

    for item in bag_items.iter() {
        result = result + (item.count * bags_required_inside_a_bag(rule_map, &item.name));
    }

    return result;
}