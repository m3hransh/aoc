use std::fs;

fn main() {}

fn parse_input(text: &str) -> Result<(Vec<Vec<usize>>, Vec<Vec<usize>>), String> {
    let parts = text.split("\n\n").collect::<Vec<&str>>();
    // Ensure there are exactly two parts (rules and updates)
    if parts.len() != 2 {
        return Err(
            "Input must have exactly two sections separated by double newlines".to_string(),
        );
    }
    let rules = parts[0]
        .lines()
        .map(|line| {
            line.split('|')
                .map(|n| n.parse::<usize>())
                .collect::<Result<Vec<usize>, _>>()
                .map_err(|e| format!("Failed to parse rule: {} (error: {})", line, e))
        })
        .collect::<Result<Vec<Vec<usize>>, String>>()?;
    let updates = parts[1]
        .lines()
        .map(|line| {
            line.split(',')
                .map(|n| n.parse::<usize>())
                .collect::<Result<Vec<usize>, _>>()
                .map_err(|e| format!("Failed to parse rule: {} (error: {})", line, e))
        })
        .collect::<Result<Vec<Vec<usize>>, String>>()?;
    return Ok((rules, updates));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_input() {
        let text = "75|13\n\
                   53|13\n\n\
                   75,47,61,53,29\n\
                   97,61,53,29,13\n";
        assert_eq!(
            parse_input(text).unwrap(),
            (
                vec![vec![75, 13], vec![53, 13]],
                vec![vec![75, 47, 61, 53, 29], vec![97, 61, 53, 29, 13]]
            )
        );
    }
}
