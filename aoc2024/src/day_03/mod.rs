use regex::Regex;

#[allow(dead_code)]
fn part_1(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            Regex::new(r"mul\((\d+),(\d+)\)")
                .unwrap()
                .find_iter(line)
                .map(|m| m.as_str())
                .map(|str_val| str_val.trim_start_matches("mul(").trim_end_matches(')'))
                .map(|joined_num| {
                    joined_num
                        .split(',')
                        .map(|number| number.parse::<i32>().unwrap())
                        .product::<i32>()
                })
                .sum::<i32>()
        })
        .sum()
}

#[allow(dead_code)]
fn part_2(input: &str) -> i32 {
    input
        .replace(['\n', '\r'], "")
        .lines()
        .map(|line| {
            let dos: Vec<(u32, u32)> = Regex::new(r"do\(\)")
                .unwrap()
                .find_iter(line)
                .map(|m| (m.start() as u32, m.end() as u32))
                .collect();

            let donts: Vec<(u32, u32)> = Regex::new(r"don\'t\(\)")
                .unwrap()
                .find_iter(line)
                .map(|m| (m.start() as u32, m.end() as u32))
                .collect();

            let mut prev_range_start: u32 = 0;
            let mut ranges = dos
                .iter()
                .map(|(do_start, _)| {
                    let filtered_donts: Vec<(u32, u32)> = donts
                        .iter()
                        .cloned()
                        .filter(|(donts_start, _)| donts_start < do_start)
                        .filter(|(_, donts_end)| *donts_end > prev_range_start)
                        .collect();

                    if filtered_donts.is_empty() {
                        return (0, 0);
                    }

                    prev_range_start = *do_start;

                    println!("selected {:?}", (filtered_donts[0].1, *do_start));

                    (filtered_donts[0].1, *do_start)
                })
                .filter(|range| range != &(0, 0))
                .collect::<Vec<(u32, u32)>>();

            ranges.sort_by(|a, b| b.1.cmp(&a.1));

            ranges.reverse();

            println!("ranges {:?}", ranges);

            let output: Vec<(u32, u32, &str)> = Regex::new(r"mul\((\d+),(\d+)\)")
                .unwrap()
                .find_iter(line)
                .map(|m| (m.start() as u32, m.end() as u32, m.as_str()))
                .filter(|(start, end, _str_val)| {
                    if ranges.iter().rev().any(|(range_start, range_end)| {
                        *start >= *range_start && *end <= *range_end
                    }) {
                        return false;
                    }
                    true
                })
                .collect();

            println!("output {:?}", output);

            output
                .iter()
                .map(|(_, _, str_val)| str_val.trim_start_matches("mul(").trim_end_matches(')'))
                .map(|joined_num| {
                    joined_num
                        .split(',')
                        .map(|number| number.parse::<i32>().unwrap())
                        .product::<i32>()
                })
                .sum::<i32>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use common::util::read_input;

    use super::*;

    const INPUT_1: &str = r#"
xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
"#;
    const INPUT_2: &str = r#"
xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
"#;
    #[test]
    fn test_part_1() {
        let result = part_1(INPUT_1.trim_start_matches('\n'));
        assert_eq!(result, 161);
    }

    #[test]
    fn run_part_1() {
        let result = part_1(&read_input(file!()));
        assert_eq!(result, 174960292)
    }

    #[test]
    fn test_part_2() {
        let result = part_2(INPUT_2.trim_start_matches('\n'));
        assert_eq!(result, 48);
    }

    #[test]
    fn run_part_2() {
        let result = part_2(&read_input(file!()));
        assert_eq!(result, 56275602)
    }
}
