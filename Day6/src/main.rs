use array2d::Array2D;

use crate::Operation::Multiply;
use crate::Operation::Add;

#[derive(PartialEq)]
enum Operation {
    Add,
    Multiply
}

// Splits on space, ignoring any empty parts
fn clean_split(line: &str) -> Vec<&str> {
    let mut clean_parts = Vec::new();

    let parts: Vec<&str> = line.split(" ").collect();
    for part in parts {
        // Ignore empty split parts
        if part.len() == 0 {
            continue;
        }
        clean_parts.push(part);
    }

    clean_parts
}

fn process_part_one(input: &str) -> u64 {
    // Arrange into array2d
    let all_lines: Vec<&str> = input.lines().collect();

    let line_one = clean_split(all_lines[0]);

    let mut array = Array2D::filled_with("", all_lines.len(), line_one.len());

    let mut row_index = 0;
    for line in all_lines {
        let current_line = clean_split(line);
        for (index, value) in current_line.iter().enumerate() {
            let _ = array.set(row_index, index, value);
        }
        row_index += 1;
    }
    let mut result: u64 = 0;
    for column_iter in array.columns_iter() {
        let mut numbers: Vec<u64> = Vec::new();
        let mut operation: Option<Operation> = None;
        for value in column_iter {
            match *value {
                "*" =>  operation = Some(Multiply),
                "+" => operation = Some(Add),
                _ => {
                    // Try parse as number
                    let number = (*value).parse::<u64>().unwrap();
                    numbers.push(number);
                }
            }
        }
        let column_result: u64 = match operation {
            Some(Add) => numbers.iter().sum::<u64>(),
            Some(Multiply) => numbers.iter().product::<u64>(),
            _ => {
                println!("Unable to find operation in column!");
                0
            }
        };
        result += column_result;
    }

    result
}


fn main() {
    let input = include_str!("input.txt");

    

    println!("Part One: {}", process_part_one(input));
}
