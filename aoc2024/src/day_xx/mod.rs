#[allow(dead_code)]
fn part_1(input: &str) -> i32 {
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

    #[test]
    fn test_part_1() {
        let input = r#"
"#
        .trim_start_matches('\n');

        let result = part_1(input);
        assert_eq!(result, 1);
    }

    #[test]
    fn run_part_1() {
        let result = part_1(&read_input(file!()));
        assert_eq!(result, 1)
    }

    #[test]
    fn test_part_2() {
        let input = r#"
"#
        .trim_start_matches('\n');

        let result = part_1(input);
        assert_eq!(result, 1);
    }

    #[test]
    fn run_part_2() {
        let result = part_2(&read_input(file!()));
        assert_eq!(result, 1)
    }
}
