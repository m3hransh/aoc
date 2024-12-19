use std::collections::{HashMap, HashSet};
use std::fs;

fn main() {
    let text_input = fs::read_to_string("inputs/day5.txt").unwrap();
    let (rules, updates) = parse_input(&text_input).unwrap();
    println!(
        "{:?}",
        unviolated_updates_middle(&updates, &rules)
            .iter()
            .sum::<usize>()
    );
}

fn violate(update: &Vec<usize>, rules: &HashMap<usize, HashSet<usize>>) -> bool {
    let mut seen = HashSet::new();
    for &n in update {
        if let Some(rule_set) = rules.get(&n) {
            if !rule_set.is_disjoint(&seen) {
                return true;
            }
        }
        seen.insert(n);
    }
    false
}
fn unviolated_updates_middle(
    updates: &Vec<Vec<usize>>,
    rules: &HashMap<usize, HashSet<usize>>,
) -> Vec<usize> {
    updates
        .iter()
        .filter(|update| !violate(update, rules))
        .map(|x| x[x.len() / 2])
        .collect()
}
fn parse_input(text: &str) -> Result<(HashMap<usize, HashSet<usize>>, Vec<Vec<usize>>), String> {
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
                .map(str::parse::<usize>)
                .collect::<Result<Vec<usize>, _>>()
        })
        .try_fold(
            HashMap::<usize, HashSet<usize>>::new(),
            |mut acc, rule_result| {
                let rule = rule_result.map_err(|e| format!("Failed to get rule: {:?}", e))?;
                if rule.len() < 2 {
                    return Err(format!("Invalid rule: {:?}", rule));
                }
                let key = rule[0];
                let value = rule[1];
                acc.entry(key)
                    .and_modify(|items| {
                        items.insert(value);
                    })
                    .or_insert(HashSet::from([value]));
                Ok(acc)
            },
        )?;
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
                HashMap::from([(75, HashSet::from([13])), (53, HashSet::from([13]))]),
                vec![vec![75, 47, 61, 53, 29], vec![97, 61, 53, 29, 13]]
            )
        );
    }
    #[test]
    fn test_violate() {
        let rules = HashMap::from([(13, HashSet::from([75, 53]))]);
        let updates = vec![vec![75, 47, 61, 53, 29], vec![97, 61, 53, 29, 13]];
        assert_eq!(violate(&updates[0], &rules), false);
        assert_eq!(violate(&updates[1], &rules), true);
    }
    #[test]
    fn test_input() {
        let text = fs::read_to_string("inputs/day5_test.txt").unwrap();
        let (rules, updates) = parse_input(&text).unwrap();
        assert_eq!(unviolated_updates_middle(&updates, &rules), [61, 53, 29]);
    }
}
