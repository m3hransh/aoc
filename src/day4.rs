use std::fs;

pub fn main() {
    let input = fs::read_to_string("inputs/day4.txt");
    let puzzle = parse_puzzle_input(&input.unwrap());

    let words_num = find_all_word(&puzzle, &['X', 'M', 'A', 'S']);
    println!("Number of words found: {}", words_num);
}

fn parse_puzzle_input(text: &str) -> Vec<Vec<char>> {
    text.lines().map(|x| x.chars().collect()).collect()
}

fn find_all_word(puzzle: &[Vec<char>], word: &[char]) -> usize {
    let mut count = 0;
    let rows = puzzle.len();
    let cols = puzzle[0].len();
    let directions = vec![
        (0, 1),  // horizontal
        (1, 0),  // vertical
        (0, -1), // backward
        (-1, 0),
        (1, 1),   // diagonal
        (1, -1),  // diagonal
        (-1, 1),  // diagonal
        (-1, -1), // diagonal
    ];
    for i in 0..rows {
        for j in 0..cols {
            for direction in &directions {
                let c = search_word(puzzle, i, j, direction, word);
                // if c > 0 {
                //     println!("Number: {}", count);
                //     print_matrix(&puzzle, i, j, direction);
                // }
                count += c
            }
        }
    }
    return count;
}

fn search_word(
    puzzle: &[Vec<char>],
    i: usize,
    j: usize,
    direction: &(i32, i32),
    word: &[char],
) -> usize {
    if puzzle[i][j] != word[0] || word.len() == 0 {
        return 0;
    }
    if word.len() == 1 {
        return 1;
    }
    let rows = puzzle.len();
    let cols = puzzle.len();

    let new_word = &word[1..];
    let new_i = i as i32 + direction.0;
    let new_j = j as i32 + direction.1;
    if new_i >= 0 && new_i < rows as i32 && new_j >= 0 && new_j < cols as i32 {
        return search_word(puzzle, new_i as usize, new_j as usize, direction, new_word);
    } else {
        return 0;
    }
}
#[cfg(test)]
mod tests {
    use super::*; // Import everything from the outer scope
    #[test]
    fn test_parsing() {
        let puzzle = "..#\n#.#\n.#.".to_string();
        assert_eq!(
            parse_puzzle_input(&puzzle),
            vec![
                vec!['.', '.', '#'],
                vec!['#', '.', '#'],
                vec!['.', '#', '.']
            ]
        );
    }
    #[test]
    fn test_search_word() {
        let puzzle = vec![
            vec!['.', '.', '#'],
            vec!['#', '.', '#'],
            vec!['2', '#', '.'],
        ];
        let word = vec!['.', '#', '2'];
        assert_eq!(search_word(&puzzle, 0, 0, &(1, 0), &word), 1);
    }

    #[test]
    fn test_find_all_word() {
        let input = fs::read_to_string("inputs/day4_test.txt").unwrap();
        let puzzle = parse_puzzle_input(&input);
        let word = vec!['X', 'M', 'A', 'S'];
        assert_eq!(find_all_word(&puzzle[..], &word), 18);
    }
}
