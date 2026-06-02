fn find_joltage(ranges: Vec<&str>) -> u32 {
    let mut result: u32 = 0;

    for line in ranges {
        let mut max_num: u8 = 0;
        for i in 0..line.len() {
            let string_digit = &line[i..i + 1];
            for j in i + 1..line.len() {
                let string_digit_2 = &line[j..j + 1];
                let result = format!("{string_digit}{string_digit_2}").parse::<u8>();
                match result {
                    Ok(number) => {
                        if max_num < number {
                            max_num = number;
                        }
                    }
                    Err(e) => println!("{}", e),
                }
            }
        }
        result += max_num as u32;
    }

    result
}

fn find_joltage_part_two(ranges: Vec<&str>) -> u64 {
    let mut result: u64 = 0;
    for line in ranges {
        
    }
    result
}

fn main() {
    let input = include_str!("input.txt");

    println!("Part One: {}", find_joltage(input.lines().collect()));

    println!("Part Two: {}", find_joltage_part_two(input.lines().collect()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let test_input = "987654321111111\n811111111111119\n234234234234278\n818181911112111";

        assert_eq!(357, find_joltage(test_input.lines().collect()))
    }

    #[test]
    fn test_part_two() {
        let test_input = "987654321111111\n811111111111119\n234234234234278\n818181911112111";

        assert_eq!(3121910778619, find_joltage_part_two(test_input.lines().collect()))
    }
}
