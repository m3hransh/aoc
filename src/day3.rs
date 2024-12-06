use regex::Regex;
use std::fs;

pub fn main() {
    let input = fs::read_to_string("inputs/day3.txt");

    match input {
        Ok(value) => match parse_instruction(&value) {
            Some(result) => println!("The result is {}", result),
            None => println!("Error parsing the input file"),
        },
        Err(e) => println!("Error: {}", e),
    }
}

pub fn parse_instruction(text: &str) -> Option<i32> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let mut result = 0;
    for cap in re.captures_iter(text) {
        let a = cap[1].parse::<i32>().ok()?;
        let b = cap[2].parse::<i32>().ok()?;
        result += a * b;
    }
    return Some(result);
}

#[cfg(test)]
mod tests {
    use super::*; // Import everything from the outer scope
    #[test]
    fn safe_test() {
        let text = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(
            parse_instruction(text),
            Some(2 * 4 + 5 * 5 + 11 * 8 + 8 * 5)
        );
    }
}
