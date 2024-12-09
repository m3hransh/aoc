use std::cmp::min;
use std::{fs, num::ParseIntError};
fn main() {
    let text_input = fs::read_to_string("inputs/day9.txt").unwrap();
    let disk = parse_input(&text_input.trim()).unwrap();
    println!("Checksum : {}", compactfile_checksum(disk));
}

fn parse_input(text: &str) -> Result<Vec<u8>, String> {
    text.chars()
        .map(|x| {
            x.to_string()
                .parse::<u8>()
                .map_err(|e| format!("Error parsing '{}' as u8: {}", x, e))
        })
        .collect::<Result<Vec<u8>, _>>()
}

fn compactfile_checksum(mut disk: Vec<u8>) -> usize {
    let mut curr_index = 0;
    let mut end_file = ((disk.len() - 1) / 2) * 2;
    let mut checksum = 0;
    for (id, i) in (0..disk.len() - 1).step_by(2).enumerate() {
        // [2,3, 1, 2]
        //
        // (curr_index:0 + i:0) * id  + (curr_index + i+1) * id
        //  ( disk[i] * current_index + (disk[i] -1)) * id
        // calculate the check sum of the curren files
        checksum += calculate_checksome_block(id, disk[i] as usize, curr_index);

        curr_index += disk[i] as usize;
        while disk[i + 1] > 0 && end_file > i + 1 {
            // get number of replacement files
            let replace_files = min(disk[end_file], disk[i + 1]);
            checksum += calculate_checksome_block(end_file / 2, replace_files as usize, curr_index);

            disk[i + 1] -= replace_files;
            curr_index += replace_files as usize;

            disk[end_file] -= replace_files;
            if disk[end_file] == 0 {
                end_file -= 2;
            }
        }
    }
    return checksum;
}

fn calculate_checksome_block(id: usize, file_num: usize, starting_index: usize) -> usize {
    ((file_num) * (2 * starting_index + file_num - 1) / 2) * id
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_input() {
        let text = "2333";
        assert_eq!(parse_input(&text).unwrap(), [2, 3, 3, 3]);
    }
    #[test]
    fn calculate_checksome_block_test() {
        assert_eq!(calculate_checksome_block(2, 2, 3), 2 * 3 + 2 * 4);
        assert_eq!(calculate_checksome_block(2, 0, 3), 0);
    }
    #[test]
    fn test_checksum() {
        let disk = vec![2, 3, 3, 3, 2, 2];
        assert_eq!(
            compactfile_checksum(disk),
            0 * 0 + 0 * 1 + 2 * 2 + 2 * 3 + 1 * 4 + 1 * 5 + 1 * 6
        );
    }
    #[test]
    fn test_checksum2() {
        let disk = vec![2, 3, 0, 3, 4];
        assert_eq!(
            compactfile_checksum(disk),
            0 * 0 + 0 * 1 + 2 * 2 + 2 * 3 + 2 * 4 + 2 * 5
        );
    }
    #[test]
    fn test_checksum_sample() {
        let disk = parse_input("2333133121414131402").unwrap();
        assert_eq!(compactfile_checksum(disk), 1928);
    }
}
