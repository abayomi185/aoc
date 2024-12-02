#[allow(dead_code)]
fn part_2(input: &str) -> i64 {
    let _placeholder = input;
    0
}

#[cfg(test)]
mod tests {
    use common::util::read_input;

    use super::*;
    use crate::util::print_green;
    use std::path::Path;

    #[test]
    fn test_part_2() {
        let input = r#"
"#
        .trim_start_matches('\n');

        let result = part_2(input);
        assert_eq!(result, 0);
    }

    #[test]
    fn run_part_2() {
        let input = &read_input(file!());

        let result = part_2(input);

        let file_name = file!();
        let file_path = Path::new(file_name);
        let file_name = file_path
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .split('.')
            .next()
            .unwrap();

        let mut dir_name: &str = "";

        if let Some(dir) = file_path.parent() {
            dir_name = dir.to_str().unwrap();
            dir_name = dir_name.split('/').last().unwrap();
        }

        print_green(&format!("{dir_name}, {file_name} result: {}", result));
    }
}
