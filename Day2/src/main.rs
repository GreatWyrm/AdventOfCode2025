
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
                    let part2 = &value[length/2..];
                    if part1 == part2 {
                        // We've matched, add
                        invalid_id_sum += num;
                    }
                }
            }
            (Ok(_num1), Err(e)) => println!("Failed to parse second number with string {}: {}.", string, e),
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
}
