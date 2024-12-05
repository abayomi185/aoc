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

    const INPUT: &str = r#"
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
