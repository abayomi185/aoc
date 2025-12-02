use std::collections::HashMap;

use common::util::Point;

type Matrix = Vec<Vec<char>>;

#[allow(dead_code)]
fn part_1(input: &str) -> i32 {
    let next_word_map: HashMap<char, char> = HashMap::from([('X', 'M'), ('M', 'A'), ('A', 'S')]);

    let mut coordinates: Vec<Vec<(char, Point)>> = Vec::new();

    fn find_next_word_around(
        matrix: &Matrix,
        curr_char: char,
        point: Point,
        next_word_map: &HashMap<char, char>,
        coordinates: &mut Vec<Vec<(char, Point)>>,
    ) {
        for range_y in [-1, 1].iter() {
            for range_x in [-1, 1].iter() {
                if point.y + range_y < 0
                    || point.x + range_x < 0
                    || point.y + range_y > matrix.len() as i64
                    || point.x + range_x > matrix[0].len() as i64
                {
                    continue;
                }

                if matrix.get((point.y + *range_y) as usize).is_some()
                    && matrix[(point.y + *range_y) as usize]
                        .get((point.x + *range_x) as usize)
                        .is_some()
                    && next_word_map.contains_key(&curr_char)
                {
                    // coordinates.push((curr_char, Point::new(point.y + range_y, point.x + range_x)));
                    find_next_word_around(
                        matrix,
                        next_word_map[&curr_char],
                        Point::new(point.x + range_x, point.y + range_y),
                        next_word_map,
                        coordinates,
                    )
                }
            }
        }
    }

    let matrix: Matrix = input.lines().map(|line| line.chars().collect()).collect();

    for (idx_row, row) in matrix.iter().enumerate() {
        for (idx_char, char) in row.iter().enumerate() {
            find_next_word_around(
                &matrix,
                *char,
                Point::new(idx_row as i64, idx_char as i64),
                &next_word_map,
                &mut coordinates,
            )
        }
    }

    println!("{:?}", coordinates);

    1
}

#[allow(dead_code)]
fn part_2(input: &str) -> i32 {
    2
}

#[cfg(test)]
mod tests {
    use common::util::read_input;

    use super::*;

    const INPUT: &str = r#"
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
"#;

    #[test]
    fn test_part_1() {
        let result = part_1(INPUT.trim_start_matches('\n'));
        assert_eq!(result, 1);
    }

    #[test]
    fn run_part_1() {
        let result = part_1(&read_input(file!()));
        assert_eq!(result, 1)
    }

    #[test]
    fn test_part_2() {
        let result = part_2(INPUT.trim_start_matches('\n'));
        assert_eq!(result, 1);
    }

    #[test]
    fn run_part_2() {
        let result = part_2(&read_input(file!()));
        assert_eq!(result, 1)
    }
}
