use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

fn main() {
    let text_input = fs::read_to_string("inputs/day8.txt").unwrap();
    let map_matrix = parse_input(&text_input);

    println!("Antinodes count: {}", antinodes_count(map_matrix));
}

fn antinodes_count(map_matrix: Vec<Vec<char>>) -> usize {
    let mut map: HashMap<char, Vec<(isize, isize)>> = HashMap::new();

    for (i, row) in map_matrix.iter().enumerate() {
        for (j, node) in row.iter().enumerate() {
            if node.is_alphanumeric() {
                map.entry(node.clone())
                    .and_modify(|nodes| nodes.push((i as isize, j as isize)))
                    .or_insert(vec![(i as isize, j as isize)]);
            }
        }
    }
    let i_max = (map_matrix.len() - 1) as isize;

    let j_max = (map_matrix[0].len() - 1) as isize;
    //Iterate through the each node group
    let mut antinodes: HashSet<_> = HashSet::new();
    for (_key, nodes) in map.iter() {
        antinodes.extend(antinodes_group_count(&nodes[..], |n| {
            is_inbound(i_max, j_max, n)
        }));
    }
    return antinodes.len();
}

fn antinodes_group_count<F>(nodes: &[(isize, isize)], is_inbound: F) -> HashSet<(usize, usize)>
where
    F: Fn((isize, isize)) -> bool,
{
    let mut seen: HashSet<(usize, usize)> = HashSet::new();
    for (i, start_node) in nodes[..nodes.len() - 1].iter().enumerate() {
        for node in nodes[i + 1..].iter() {
            if is_inbound((
                start_node.0 + (start_node.0 - node.0),
                start_node.1 + (start_node.1 - node.1),
            )) {
                seen.insert((
                    (start_node.0 + (start_node.0 - node.0)) as usize,
                    (start_node.1 + (start_node.1 - node.1)) as usize,
                ));
            }
            if is_inbound((
                node.0 - (start_node.0 - node.0),
                node.1 - (start_node.1 - node.1),
            )) {
                seen.insert((
                    (node.0 - (start_node.0 - node.0)) as usize,
                    (node.1 - (start_node.1 - node.1)) as usize,
                ));
            }
        }
    }
    return seen;
}
fn is_inbound(max_i: isize, max_j: isize, node: (isize, isize)) -> bool {
    if node.0 >= 0 && node.0 <= max_i && node.1 >= 0 && node.1 <= max_j {
        return true;
    } else {
        return false;
    }
}

fn testable(target: usize, current_value: usize, values: &[usize]) -> bool {
    if values.len() == 0 {
        return current_value == target;
    }
    return testable(target, current_value + values[0], &values[1..])
        || testable(target, current_value * values[0], &values[1..]);
}

fn parse_input(text: &str) -> Vec<Vec<char>> {
    text.lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse() {
        let text = ".1a\na.B";
        assert_eq!(
            parse_input(&text),
            vec![vec!['.', '1', 'a'], vec!['a', '.', 'B']]
        );
    }

    #[test]
    fn test_antidotes_node_list() {
        let nodes = vec![(3, 4), (5, 5), (4, 8)];
        assert_eq!(
            antinodes_group_count(&nodes[..], |n| is_inbound(9, 9, n)),
            HashSet::from([(7, 6), (1, 3), (6, 2), (2, 0)])
        );
    }
}
