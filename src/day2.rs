use aoc2024::utils;

pub fn main() {
    let input = utils::read_line::<i32>("inputs/day2.txt");

    if safe(&vec![1, 2, 3, 4]) {
        println!("Safe reports: {}", 1);
    } else {
        println!("Safe reports: {}", 0);
    }
    match input {
        Some(input) => {
            let mut valid = 0;
            for a in input {
                if safe(&a) {
                    valid += 1;
                }
            }
            println!("Safe reports: {}", valid);
        }
        None => println!("Error reading input"),
    }
}

fn is_negative(a: i32) -> bool {
    a < 0
}

pub fn safe(a: &[i32]) -> bool {
    if a.len() < 2 {
        return true;
    }
    let negative = is_negative(a[1] - a[0]);

    for i in 0..(a.len() - 1) {
        let diff = a[i + 1] - a[i];

        if is_negative(diff) != negative || diff.abs() > 3 || diff.abs() < 1 {
            return false;
        }
    }
    return true;
}

#[cfg(test)]
mod tests {
    use super::*; // Import everything from the outer scope
    #[test]
    fn safe_test() {
        assert_eq!(safe(&vec![7, 6, 4, 2, 1]), true);
        assert_eq!(safe(&vec![7, 6, 4, 5, 1]), false);
        assert_eq!(safe(&vec![7, 6, 4, 4, 1]), false);
        assert_eq!(safe(&vec![7, 6, 5, 4, 1]), true);
        assert_eq!(safe(&vec![7, 6, 5, 4, 0]), false);
        assert_eq!(safe(&vec![7, 6, 5, 4, 0]), false);
        assert_eq!(safe(&vec![1, 2, 2, 3]), false);
        assert_eq!(safe(&vec![1]), true);
        assert_eq!(safe(&vec![1, 2, 3, 4]), true);
        assert_eq!(safe(&vec![-1, 2, 3, 4]), true);
    }
}
