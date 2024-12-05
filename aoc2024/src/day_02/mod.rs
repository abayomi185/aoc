#[allow(dead_code)]
fn part_1(input: &str) -> i32 {
    let safe_unsafe = input
        .lines()
        .map(|line| {
            let numbers: Vec<i32> = line
                .split_whitespace()
                .map(|number| number.parse::<i32>().unwrap())
                .collect();

            let diff: Vec<i32> = numbers
                .iter()
                .take(numbers.len() - 1)
                .enumerate()
                .map(|(idx, num)| numbers[idx + 1] - num)
                .collect();

            if diff.iter().all(|val| (1..=3).contains(&val.abs()))
                && (diff.iter().all(|val| val.is_positive())
                    || diff.iter().all(|val| val.is_negative()))
            {
                return 1; // Safe
            }

            0 // Unsafe
        })
        .sum();

    safe_unsafe
}

#[allow(dead_code)]
fn part_2(input: &str) -> i32 {
    let safe_unsafe = input
        .lines()
        .map(|line| {
            let numbers: Vec<i32> = line
                .split_whitespace()
                .map(|number| number.parse::<i32>().unwrap())
                .collect();

            for idx in 0..numbers.len() {
                let mut numbers_copied = numbers.clone();
                numbers_copied.remove(idx);

                let diff: Vec<i32> = numbers_copied
                    .iter()
                    .take(numbers_copied.len() - 1)
                    .enumerate()
                    .map(|(idx, num)| numbers_copied[idx + 1] - num)
                    .collect();

                if diff.iter().all(|val| (1..=3).contains(&val.abs()))
                    && (diff.iter().all(|val| val.is_positive())
                        || diff.iter().all(|val| val.is_negative()))
                {
                    println!("  Safe {:?}", numbers_copied);
                    return 1; // Safe
                }
            }

            println!("Unsafe {:?}", numbers);
            0 // Unsafe
        })
        .sum();

    safe_unsafe
}

#[cfg(test)]
mod tests {
    use common::util::read_input;

    use super::*;

    const INPUT: &str = r#"
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
"#;

    #[test]
    fn test_part_1() {
        let result = part_1(INPUT.trim_start_matches('\n'));
        assert_eq!(result, 2);
    }

    #[test]
    fn run_part_1() {
        let result = part_1(&read_input(file!()));
        assert_eq!(result, 564)
    }

    #[test]
    fn test_part_2() {
        let result = part_2(INPUT.trim_start_matches('\n'));
        assert_eq!(result, 4);
    }

    #[test]
    fn run_part_2() {
        let result = part_2(&read_input(file!()));
        assert_eq!(result, 604)
    }
}
