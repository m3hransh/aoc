use aoc2024::utils;

pub fn main() {
    let input_list = utils::read_line_list_int::<i32, 2>("inputs/day1.txt").unwrap();
    let left = input_list.iter().map(|x| x[0]).collect::<Vec<i32>>();
    let right = input_list.iter().map(|x| x[1]).collect::<Vec<i32>>();

    println!("Part 1: {}", calculate_list_distance(&left, &right));
}

pub fn calculate_list_distance(a: &[i32], b: &[i32]) -> i32 {
    let mut a = a.to_vec();
    let mut b = b.to_vec();
    a.sort();
    b.sort();

    a.iter().zip(b).map(|(x, y)| (x - y).abs()).sum()
}

#[cfg(test)]
mod tests {
    use super::*; // Import everything from the outer scope
    #[test]
    fn list_distance_calculates_works() {
        assert_eq!(
            calculate_list_distance(&[3, 4, 2, 1, 3, 3], &[4, 3, 5, 3, 9, 3]),
            11
        );
    }
}
