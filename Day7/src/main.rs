use array2d::Array2D;
use std::collections::HashSet;

#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Debug)]
enum ArrayState {
    Empty,
    Start,
    Splitter
}

fn transform_into_array(input: &str) -> Array2D<ArrayState> {
    let lines: Vec<&str> = input.lines().collect();

    let mut curr_array = Array2D::filled_with(ArrayState::Empty, lines.len(), lines[0].len());

    for i in 0..lines.len() {
        for j in 0..lines[i].len() {
            let current_char = &lines[i][j..j+1];
            let _ = match current_char {
                "S" => curr_array.set(i, j, ArrayState::Start),
                "^" => curr_array.set(i, j, ArrayState::Splitter),
                _ => Ok({}), // Do nothing
            };
        }
    }

    curr_array
}

fn count_beam_splits(input: &str) -> i32 {
    let array = transform_into_array(input);

    let mut beam_split_count = 0;

    // Start from the top, find the start, start the splitting
    let mut beam_column_indicies: HashSet<usize> = HashSet::new();
    for i in 0..array.num_rows() {
        if i == 0 {
            for j in 0..array.row_len() {
                let current = array.get(i, j);
                if *current.unwrap() == ArrayState::Start {
                    beam_column_indicies.insert(j);
                }
            }
        } else {
            let mut next_beam_column_indicies: Vec<(usize, usize, usize)> = Vec::new();
            for j in beam_column_indicies.iter() {
                let result = array.get(i, *j);
                if *result.unwrap() == ArrayState::Splitter {
                    // Split to the left and the right
                    beam_split_count += 1;
                    // It's not present in the input, so we don't have to check it, but we should check bounds (i.e. < 0 or == row_len())
                    next_beam_column_indicies.push((*j, *j - 1, *j + 1));
                }
            }
            for value in next_beam_column_indicies {
                beam_column_indicies.remove(&value.0);
                beam_column_indicies.insert(value.1);
                beam_column_indicies.insert(value.2);
            }
            
        }
        
    }

    beam_split_count
}

fn main() {

    let input = include_str!("input.txt");

    println!("Part One: {}", count_beam_splits(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let test_input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";
        assert_eq!(21, count_beam_splits(test_input));
    }

    #[test]
    fn test_part_two() {
       let test_input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";
        //assert_eq!(40, count_timelines(test_input));
    }
}
