
#[derive(PartialEq, Debug)]
struct Point {
    x: u64,
    y: u64
}

fn parse_into_points(input: &str) -> Vec<Point> {
    let lines: Vec<&str> = input.lines().collect();
    let mut points: Vec<Point> = Vec::new();
    points.reserve(lines.len());

    for line in lines {
        let values: Vec<&str> = line.split(",").collect();
        if values.len() != 2 {
            continue;
        }
        let x_parsed = values[0].parse::<u64>();
        let y_parsed = values[1].parse::<u64>();
        match (x_parsed, y_parsed) {
            (Ok(x), Ok(y)) => points.push(Point {
                x,
                y
            }),
            _ => println!("Failed to parse value {}", line),
        }
    }

    points
}

fn find_biggest_rectangle_area(point_list: Vec<Point>) -> u64 {
    let mut biggest_area: u64 = 0;

    for point in &point_list {
        for other_point in &point_list {
            if point == other_point {
                continue;
            }
            let size = get_rectangle_size(point, other_point);
            if biggest_area < size {
                biggest_area = size;
            }
        }
    }

    
    biggest_area
}


fn get_rectangle_size(one: &Point, two: &Point) -> u64 {
    let max_x = std::cmp::max(one.x, two.x);
    let min_x = std::cmp::min(one.x, two.x);
    let max_y = std::cmp::max(one.y, two.y);
    let min_y = std::cmp::min(one.y, two.y);
    
    (max_x - min_x + 1) * (max_y - min_y + 1)
}

fn main() {
    let input = include_str!("input.txt");

    println!("Part One: {}", find_biggest_rectangle_area(parse_into_points(input)));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let test_input = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";
        assert_eq!(50, find_biggest_rectangle_area(parse_into_points(test_input)));
    }

    #[test]
    fn test_part_two() {
       let test_input = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";
        //assert_eq!(24, some_function(test_input));
    }
}
