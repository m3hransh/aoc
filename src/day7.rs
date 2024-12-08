use std::fs;
fn main() {
    let text_input = fs::read_to_string("inputs/day7.txt").unwrap();
    let report_input = parse_input(&text_input).unwrap();

    println!(
        "Total calibration result: {}",
        calibration_result(report_input)
    )
}

fn calibration_result(test_input: Vec<(usize, Vec<usize>)>) -> usize {
    let mut result = 0;
    for (target, values) in test_input.iter() {
        if testable(target.clone(), values[0], &values[1..]) {
            result += target;
        }
    }
    return result;
}

fn testable(target: usize, current_value: usize, values: &[usize]) -> bool {
    if values.len() == 0 {
        return current_value == target;
    }
    return testable(target, current_value + values[0], &values[1..])
        || testable(target, current_value * values[0], &values[1..]);
}

fn parse_input(text: &str) -> Result<Vec<(usize, Vec<usize>)>, Box<dyn std::error::Error>> {
    text.lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(":").collect();
            if parts.len() < 2 {
                return Err("invalid input format".into());
            }
            let target = parts[0].trim().parse::<usize>()?;
            let values = parts[1]
                .trim()
                .split_whitespace()
                .map(|x| x.parse::<usize>())
                .collect::<Result<Vec<_>, _>>()?;

            Ok((target, values))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_input() {
        let text = "12: 2 3\n3: 1 1";
        assert_eq!(
            parse_input(&text).unwrap(),
            [(12, vec![2, 3]), (3, vec![1, 1])]
        );
    }
}
