use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::fs::File;

fn get_nums(line: String) -> Vec<char> {
    let valid_nums: HashMap<&str, char> = HashMap::from([("one", '1'), ("two", '2'), ("three", '3'), ("four", '4'), ("five", '5'), ("six", '6'), ("seven", '7'), ("eight", '8'), ("nine", '9')]);
    let mut nums = Vec::new();
    let chars = line.chars();
    let mut i = 0;
    for char in chars {
        if char.is_numeric() {
            nums.push(char);
        } else {
            for (word, num) in valid_nums.iter() {
                let part = match line.get(i..i + word.len()) {
                    Some(part) => part,
                    None => continue
                };
                if &part == word {
                    nums.push(*num);
                }
            }
        }
        i += 1;
    }
    nums
}

fn get_value(line: String) -> u32 {
    let chars = get_nums(line);
    if chars.len() == 1 {
        return format!("{}{}", chars[0], chars[0]).parse::<u32>().unwrap();
    }
    let first = chars[0];
    let last = chars[chars.len() - 1];
    format!("{}{}", first, last).parse::<u32>().unwrap()
}

pub fn main() -> u32{
    let file = match File::open("src/trebuchet/trebuchet.txt") {
        Ok(file) => file,
        Err(error) => {
            eprintln!("Could not open file: {}", error);
            return 1;
        }
    };
    
    let mut values = Vec::new();

    let reader = BufReader::new(file);
    for line in reader.lines() {
        match line {
            Ok(line) => {
                values.push(get_value(line));
            },
            Err(error) => {
                eprintln!("Could not read line: {}", error);
                return 1;
            }
        };
    }
    values.iter().sum::<u32>()
}