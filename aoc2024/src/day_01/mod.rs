use std::{collections::HashMap, iter::zip};

/**
* Advent of code 2024, Day 1: Historian Hysteria
* https://adventofcode.com/2024/day/1
*/
#[allow(dead_code)]
fn part_1(input: &str) -> i32 {
    let mut left_nums: Vec<i32> = Vec::new();
    let mut right_nums: Vec<i32> = Vec::new();

    for line in input.lines() {
        let mut iter = line.split_whitespace();
        left_nums.push(iter.next().unwrap().parse().unwrap());
        right_nums.push(iter.next().unwrap().parse().unwrap());
    }

    left_nums.sort();
    right_nums.sort();

    let sum: i32 = zip(left_nums.iter(), right_nums.iter())
        .map(|(a, b)| (a - b).abs())
        .sum();

    sum
}

#[allow(dead_code)]
fn part_2(input: &str) -> i32 {
    let mut map: HashMap<i32, i32> = HashMap::new();

    let mut left_nums: Vec<i32> = Vec::new();

    for line in input.lines() {
        let mut iter = line.split_whitespace();
        let left = iter.next().unwrap().parse().unwrap();
        left_nums.push(left);
        map.insert(left, 0);
    }

    for line in input.lines() {
        let mut iter = line.split_whitespace();
        let _left = iter.next();
        let right: i32 = iter.next().unwrap().parse().unwrap();
        if map.contains_key(&right) {
            map.insert(right, map.get(&right).unwrap() + 1);
        }
    }

    let sum: i32 = map
        .iter()
        .map(|(key, value)| {
            (key * value) * left_nums.iter().filter(|&val| val == key).count() as i32
        })
        .sum();

    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::util::{print_green, read_input};

    #[test]
    fn test_part_1() {
        let input = r#"
3   4
4   3
2   5
1   3
3   9
3   3
"#
        .trim_start_matches('\n');

        let result = part_1(input);
        assert_eq!(result, 11);
    }

    #[test]
    fn run_part_1() {
        let result = part_1(&read_input(file!()));
        print_green(&format!("Day 1, part 1 result: {result}"));
        assert_eq!(result, 2815556)
    }

    #[test]
    fn test_part_2() {
        let input = r#"
3   4
4   3
2   5
1   3
3   9
3   3
"#
        .trim_start_matches('\n');

        let result = part_2(input);
        assert_eq!(result, 31);
    }

    #[test]
    fn run_part_2() {
        let result = part_2(&read_input(file!()));
        print_green(&format!("Day 1, part 2 result: {result}"));
        assert_eq!(result, 23927637)
    }
}
