use aoc2024::utils::read_line_list_int;

#[test]
fn input_test() {
    let input = read_line_list_int::<i32, 2>("inputs/day1.txt").unwrap();
    assert_eq!(input.len(), 1000);
}
