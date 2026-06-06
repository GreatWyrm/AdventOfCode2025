struct IngredientRange {
    lower_bound: u64,
    upper_bound: u64,
}

impl std::fmt::Display for IngredientRange {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {})", self.lower_bound, self.upper_bound)
    }
}

fn count_all_possible_fresh_ingredients(input: &str) -> u64 {
    let (ingredient_ranges, _ingredients) = split_into_parts(input);

    let mut total_range_count: u64 = 0;
    let mut counted_ranges: Vec<IngredientRange> = Vec::new();

    for range in ingredient_ranges {
        let mut current_lower_bound = range.lower_bound;
        let mut current_upper_bound = range.upper_bound;
        for counted in &counted_ranges {
            // Case 1, current bounds are entirely within this bound
            if current_lower_bound >= counted.lower_bound
                && current_upper_bound <= counted.upper_bound
            {
                current_lower_bound = 0;
                current_upper_bound = 0;
                break;
            // Case 2, upper bound reaches into a lower bound. Clip to lower bound
            } else if current_upper_bound > counted.lower_bound && current_lower_bound < counted.lower_bound {
                current_upper_bound = counted.lower_bound - 1;
            // Case 3, lower bound reaches into an upper bound, Clip to upper bound
            } else if current_lower_bound < counted.upper_bound && current_upper_bound > counted.upper_bound {
                current_lower_bound = counted.upper_bound + 1;
            }
            // Case 4, no overlap. Do nothing
        }
        if current_lower_bound == 0 && current_upper_bound == 0 {
            // Range was nullified, don't count
            continue;
        }
        total_range_count += current_upper_bound - current_lower_bound + 1;
        // Add to range
        counted_ranges.push(range);
    }

    total_range_count
}

fn is_ingredient_fresh(ranges: &Vec<IngredientRange>, ingredient: u64) -> bool {
    for range in ranges {
        if ingredient >= range.lower_bound && ingredient <= range.upper_bound {
            return true;
        }
    }
    false
}

fn count_fresh_ingredients(input: &str) -> u32 {
    let (ingredient_ranges, ingredients) = split_into_parts(input);

    let mut fresh: u32 = 0;

    for ingredient in ingredients {
        if is_ingredient_fresh(&ingredient_ranges, ingredient) {
            fresh += 1;
        }
    }

    fresh
}

fn split_into_parts(input: &str) -> (Vec<IngredientRange>, Vec<u64>) {
    let lines: Vec<&str> = input.lines().collect();

    let mut ranges: Vec<IngredientRange> = Vec::new();
    let mut ingredients: Vec<u64> = Vec::new();

    for line in lines {
        match line.find("-") {
            Some(n) => {
                let num_first = &line[0..n].parse::<u64>();
                let num_second = &line[n + 1..].parse::<u64>();
                match (num_first, num_second) {
                    (Ok(n1), Ok(n2)) => {
                        ranges.push(IngredientRange {
                            lower_bound: *n1,
                            upper_bound: *n2,
                        });
                    }
                    _ => println!("Failed to parse range {}", line),
                }
            }
            None => {
                // Could be the empty line check
                if line.len() > 0 {
                    let num_result = &line.parse::<u64>();
                    match num_result {
                        Ok(num) => ingredients.push(*num),
                        _ => println!("Failed to parse ingredient."),
                    }
                }
            }
        }
    }

    (ranges, ingredients)
}

fn main() {
    let input = include_str!("input.txt");
    println!("Part One: {}", count_fresh_ingredients(input));
    println!("Part Two: {}", count_all_possible_fresh_ingredients(input));
    // Answer 354106051784269 is too low
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let test_input = "3-5\n10-14\n16-20\n12-18\n\n1\n5\n8\n11\n17\n32";
        assert_eq!(3, count_fresh_ingredients(test_input));
    }

    #[test]
    fn test_part_two() {
        let test_input = "3-5\n10-14\n16-20\n12-18\n\n1\n5\n8\n11\n17\n32";

        assert_eq!(14, count_all_possible_fresh_ingredients(test_input));
    }
}
