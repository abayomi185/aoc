#[allow(dead_code)]
fn part_1(input: &str) -> i32 {
    let mut count = 0;

    let mut rotation_index = 50;

    for line in input.lines() {
        let coordinates = line.split_at(1);
        let direction = coordinates.0;
        let distance: i32 = coordinates.1.parse().unwrap();

        match direction {
            "L" => {
                // Move left (negative)
                rotation_index -= distance;
            }
            "R" => {
                // Move right (positive)
                rotation_index += distance;
            }
            _ => {}
        }

        if rotation_index == 0 || (rotation_index % 100) == 0 {
            count += 1;
        }
    }

    count
}

#[allow(dead_code)]
fn part_2(input: &str) -> i128 {
    let mut count = 0;

    let mut rotation_index_prev = 50;
    let mut rotation_index_next = 50;

    for line in input.lines() {
        let coordinates = line.split_at(1);
        let direction = coordinates.0;
        let distance: i128 = coordinates.1.parse().unwrap();

        match direction {
            "L" => {
                // Move left (negative)
                rotation_index_next = rotation_index_prev - distance;

                if rotation_index_next < 0 {
                    if distance.abs() > 100 {
                        let multiples = distance / 100;
                        rotation_index_next -= 100 * multiples;
                        count += multiples;
                    } else {
                        count += 1;
                    }
                }

                // Normalize value
                rotation_index_next = rotation_index_prev + rotation_index_next.abs();
            }
            "R" => {
                // Move right (positive)
                rotation_index_next = rotation_index_prev + distance;

                if rotation_index_next > 99 {
                    let multiples = rotation_index_next / 100;
                    if multiples > 0 {
                        rotation_index_next -= 100 * multiples;
                        count += multiples;
                    } else {
                        rotation_index_next -= 100;
                        count += 1;
                    }
                }
            }
            _ => {}
        }

        rotation_index_prev = rotation_index_next;
    }

    count
}

#[cfg(test)]
mod tests {
    use common::util::read_input;

    use super::*;

    const INPUT: &str = r#"
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
"#;

    #[test]
    fn test_part_1() {
        let result = part_1(INPUT.trim_start_matches('\n'));
        assert_eq!(result, 3);
    }

    #[test]
    fn run_part_1() {
        let result = part_1(&read_input(file!()));
        assert_eq!(result, 1135)
    }

    #[test]
    fn test_part_2() {
        let result = part_2(INPUT.trim_start_matches('\n'));
        assert_eq!(result, 6);
    }

    #[test]
    fn run_part_2() {
        let result = part_2(&read_input(file!()));
        assert_eq!(result, 2520)
    }
}
