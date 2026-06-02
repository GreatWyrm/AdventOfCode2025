use array2d::Array2D;

fn count_accessible_paper_rolls(input: Array2D<bool>) -> u32 {
    let mut count: u32 = 0;
    for i in 0..input.row_len() {
        for j in 0..input.column_len() {
            let result = input.get(i, j);
            match result {
                Some(b) => {
                    if !*b {
                        // Not a roll, skip
                        continue;
                    }
                }
                _ => {}
            }
            let mut surrounding_count: u8 = 0;
            // Read surrounding parts, if possible
            for k in -1isize..2 {
                for l in -1isize..2 {
                    // Skip self
                    if k == 0 && l == 0 {
                        continue;
                    }
                    let current_row_index = i as isize + k;
                    let current_column_index = j as isize + l;
                    let result =
                        input.get(current_row_index as usize, current_column_index as usize);
                    match result {
                        Some(b) => {
                            if *b {
                                surrounding_count += 1
                            }
                        }
                        _ => {}
                    }
                }
            }
            if surrounding_count < 4 {
                count += 1;
            }
        }
    }
    count
}

fn convert_to_2d_array(input: &str) -> Array2D<bool> {
    let lines: Vec<&str> = input.lines().collect();
    // Should be a fixed 2d array
    let rows_count = lines.len();
    let columns_count = lines[0].len();

    let mut array = Array2D::filled_with(false, rows_count, columns_count);

    let mut row_index = 0;
    for line in lines {
        for j in 0..line.len() {
            if &line[j..j + 1] == "@" {
                let _ = array.set(row_index, j, true);
            }
        }
        row_index += 1;
    }

    array
}

fn main() {
    let input = include_str!("input.txt");

    let amount = count_accessible_paper_rolls(convert_to_2d_array(input));
    println!("Part One: {}", amount);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let test_input = "..@@.@@@@.\n@@@.@.@.@@\n@@@@@.@.@@\n@.@@@@..@.\n@@.@@@@.@@\n.@@@@@@@.@\n.@.@.@.@@@\n@.@@@.@@@@\n.@@@@@@@@.\n@.@.@@@.@.";

        assert_eq!(
            13,
            count_accessible_paper_rolls(convert_to_2d_array(test_input))
        );
    }

    #[test]
    fn test_part_two() {
        let test_input = "..@@.@@@@.\n@@@.@.@.@@\n@@@@@.@.@@\n@.@@@@..@.\n@@.@@@@.@@\n.@@@@@@@.@\n.@.@.@.@@@\n@.@@@.@@@@\n.@@@@@@@@.\n@.@.@@@.@.";

        assert_eq!(0, 0);
    }
}
