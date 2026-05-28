use std::str::Lines;

// Accepts a start position and an iterator of rotations, returns the password count
fn move_dial_part_one(start_pos: i32, lines: Lines) -> i32 {
    let mut password_count: i32 = 0;
    let mut dial_position = start_pos;

    for value in lines {
        if value.len() == 0 {
            break;
        }
        // Grab first char + rest
        let start_char = &value[0..1];
        let number = &value[1..];
        
        match number.parse::<i32>() {
            Ok(n) => match start_char {
                "L" => {
                    let temp = (dial_position - n) % 100;
                    if temp < 0 {
                        dial_position = temp + 100;
                    } else {
                        dial_position = temp;
                    }
                }
                "R" => dial_position = (dial_position + n) % 100,
                _ => println!("Unknown rotation {}.", start_char),
            },
            Err(e) => println!("{}", e),
        }
        if dial_position == 0 {
            password_count += 1;
        }
    }

    password_count
}

fn move_dial_part_two(start_pos: i32, lines: Lines) -> i32 {
    let mut password_count: i32 = 0;
    let mut dial_position = start_pos;

    for value in lines {
        if value.len() == 0 {
            break;
        }
        let start_char = &value[0..1];
        let number = &value[1..];
        
        let started_at_zero = dial_position == 0;
        match number.parse::<i32>() {
            Ok(n) => match start_char {
                "L" => {
                    dial_position -= n;
                }
                "R" => {
                    dial_position += n;
                }
                _ => println!("Unknown rotation {}.", start_char),
            },
            Err(e) => println!("{}", e),
        }
        password_count += dial_position.abs() / 100;
        // If we went below 0 (but didn't start at it, increment)
        if dial_position < 0 && !started_at_zero {
            password_count += 1;
        }
        // If we landed on 0, increment
        if dial_position == 0 {
            password_count += 1;
        }
        dial_position %= 100;
        if dial_position < 0 {
            dial_position += 100;
        }
    }

    password_count
}

fn main() {
    // Get input list
    let rotation_list = include_str!("rotations.txt");

    let test_input = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82";
    // Test part one
    let test_count = move_dial_part_one(50, test_input.lines());
    match test_count {
        3 => println!("Passed test (part one)"),
        _ => panic!(),
    }

    // Get count
    let count = move_dial_part_one(50, rotation_list.lines());

    println!("Password Count (Part One): {}.", count);

    // Test part 2
    let test_count_two = move_dial_part_two(50, test_input.lines());
    match test_count_two {
        6 => println!("Passed test (part two)"),
        _ => panic!(),
    }

    let count_two = move_dial_part_two(50, rotation_list.lines());

    println!("Password Count (Part Two): {}.", count_two);
}
