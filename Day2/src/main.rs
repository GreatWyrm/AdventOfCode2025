fn find_invalid_ids_part_two(ranges: Vec<&str>) -> i64 {
    let mut invalid_id_sum: i64 = 0;
    for string in ranges {
        if string.len() == 0 {
            continue;
        }
        // Split on '-' and convert both to ints
        let numbers: Vec<&str> = string.split("-").collect();
        if numbers.len() != 2 {
            println!("Invalid input: {}", string);
            continue;
        }
        // Try parse both
        let num1_result = numbers[0].parse::<i64>();
        let num2_result = numbers[1].parse::<i64>();
        match (num1_result, num2_result) {
            (Ok(num1), Ok(num2)) => {
                for num in num1..num2 + 1 {
                    let number_string = num.to_string();
                    let length = number_string.len();
                    // Iterate through parts up to length/2
                    for i in 1..length/2 + 1 {
                        let fragment = &number_string[0..i];
                        let fragment_length = fragment.len();
                        //println!("Trying fragment {}.", fragment);
                        // Needs to be evenly divisible
                        if length % fragment_length != 0 {
                            continue;
                        }
                        let mut is_valid: bool = false;
                        for j in 0..(length / fragment_length) {
                            let previous_index = j * fragment_length;
                            let current_index = (j + 1) * fragment_length;
                            let current_frag = &number_string[previous_index..current_index];
                            if current_frag != fragment {
                                is_valid = true;
                                break;
                            }
                        }
                        if !is_valid {
                            invalid_id_sum += num;
                            break; // Avoid counting stuff like 2222 twice
                        }
                    }
                }
            }
            (Ok(_num1), Err(e)) => println!(
                "Failed to parse second number with string {}: {}.",
                string, e
            ),
            _ => println!("Error parsing both numbers"),
        }
    }
    invalid_id_sum
}

fn find_invalid_ids(ranges: Vec<&str>) -> i64 {
    let mut invalid_id_sum: i64 = 0;
    for string in ranges {
        if string.len() == 0 {
            continue;
        }
        // Split on '-' and convert both to ints
        let numbers: Vec<&str> = string.split("-").collect();
        if numbers.len() != 2 {
            println!("Invalid input: {}", string);
            continue;
        }
        // Try parse both
        let num1_result = numbers[0].parse::<i64>();
        let num2_result = numbers[1].parse::<i64>();
        match (num1_result, num2_result) {
            (Ok(num1), Ok(num2)) => {
                for num in num1..num2 + 1 {
                    let value = num.to_string();
                    let length = value.len();
                    if length % 2 != 0 {
                        continue;
                    }
                    // Split into halves
                    let part1 = &value[0..length / 2];
                    let part2 = &value[length / 2..];
                    if part1 == part2 {
                        // We've matched, add
                        invalid_id_sum += num;
                    }
                }
            }
            (Ok(_num1), Err(e)) => println!(
                "Failed to parse second number with string {}: {}.",
                string, e
            ),
            _ => println!("Error parsing both numbers"),
        }
    }
    invalid_id_sum
}

fn main() {
    // Input
    let test_input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
    let input = include_str!("input.txt");
    let test_result = find_invalid_ids(test_input.split(",").collect());
    match test_result {
        1227775554 => println!("Passed test (part one)"),
        _ => panic!(),
    }

    println!("Part One: {}", find_invalid_ids(input.split(",").collect()));

    let test_result_part_two = find_invalid_ids_part_two(test_input.split(",").collect());
    assert_eq!(4174379265, test_result_part_two);

    println!("Passed test (Part two)");

    println!(
        "Part Two: {}",
        find_invalid_ids_part_two(input.split(",").collect())
    );
}
