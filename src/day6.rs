use std::collections::HashSet;
use std::fs;
use std::thread;
use std::time::Duration;

const GUARD: char = '🦀';
const BLOCK: char = '📦';
const SEEN: char = '✨';
const FREE: char = '⬛';

fn main() {
    let text = fs::read_to_string("inputs/day6.txt").expect("File is not Found");
    let mut map = parse_input(&text);

    search_map(&mut map);
}

fn parse_input(text: &str) -> Vec<Vec<char>> {
    text.lines()
        .map(|line| {
            line.chars()
                .map(|x| match x {
                    '.' => FREE,
                    '#' => BLOCK,
                    '^' => GUARD,
                    _ => x,
                })
                .collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>()
}

fn search_map(map: &mut Vec<Vec<char>>) {
    let mut position: (isize, isize) = find_guard(map).expect("No Guard Founded");
    let mut direction = (-1, 0);
    let mut seen_positions: HashSet<(isize, isize)> = HashSet::new();
    clear_screen(); // Clear the terminal before starting
    move_cursor_to_top_left();
    print_map(map);
    while is_inbound(position, map.len(), map[0].len()) {
        map[position.0 as usize][position.1 as usize] = SEEN;
        seen_positions.insert(position);
        let mut new_position = take_step(position, direction);

        while is_inbound(new_position, map.len(), map[0].len())
            && map[new_position.0 as usize][new_position.1 as usize] == BLOCK
        {
            direction = rotate_90_clockwise(direction);
            new_position = take_step(position, direction);
        }
        position = new_position;
        if is_inbound(new_position, map.len(), map[0].len()) {
            map[position.0 as usize][position.1 as usize] = GUARD;
        }
        move_cursor_to_top_left();
        print_map(&map[..]);
        println!(
            "👀: {}",
            format_with_color(&seen_positions.len().to_string(), "blue")
        );
        // thread::sleep(Duration::from_secs(1));
    }
}

fn find_guard(map: &Vec<Vec<char>>) -> Option<(isize, isize)> {
    for (i, row) in map.iter().enumerate() {
        for (j, &item) in row.iter().enumerate() {
            if item == GUARD {
                return Some((i as isize, j as isize));
            }
        }
    }
    return None;
}
fn is_inbound(position: (isize, isize), rows: usize, cols: usize) -> bool {
    return (position.0 >= 0
        && position.0 < rows as isize
        && position.1 >= 0
        && position.1 < cols as isize);
}
fn take_step(position: (isize, isize), direction: (isize, isize)) -> (isize, isize) {
    return (position.0 + direction.0, position.1 + direction.1);
}

fn rotate_90_clockwise(vector: (isize, isize)) -> (isize, isize) {
    let (x, y) = vector;
    (y, -x)
}
fn clear_screen() {
    print!("\x1B[2J"); // ANSI escape code to clear the screen
}

fn move_cursor_to_top_left() {
    print!("\x1B[H"); // ANSI escape code to move cursor to the top-left corner
}

fn print_map(matrix: &[Vec<char>]) {
    for row in matrix {
        for &cell in row {
            print!("{:4} ", cell); // Adjust width for uniform spacing
        }
        println!();
    }
}

fn format_with_color(text: &str, color: &str) -> String {
    let color_code = match color.to_lowercase().as_str() {
        "black" => "30",
        "red" => "31",
        "green" => "32",
        "yellow" => "33",
        "blue" => "34",
        "magenta" => "35",
        "cyan" => "36",
        "white" => "37",
        _ => "0", // Default color (no color)
    };

    format!("\x1B[{}m{}\x1B[0m", color_code, text)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_input() {
        let text = ".#.\n\
                    .^.\n\
                    #..";
        assert_eq!(
            parse_input(text),
            vec![
                vec!['.', '#', '.'],
                vec!['.', '^', '.'],
                vec!['#', '.', '.']
            ]
        );
    }
    fn test_find_guard() {
        let map = vec![
            vec!['.', '#', '.'],
            vec!['.', '^', '.'],
            vec!['#', '.', '.'],
        ];
        assert_eq!(find_guard(&map), Some((1, 1)));
    }
}
