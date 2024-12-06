use std::fs;

pub fn read_line_list_int<T, const N: usize>(filename: &str) -> Option<Vec<[T; N]>>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug, // This allows for parsing other types if needed
{
    fs::read_to_string(filename)
        .ok()?
        .lines()
        .map(|x| {
            let parsed_line: Result<Vec<T>, _> =
                x.split_whitespace().map(|y| y.parse::<T>()).collect();
            parsed_line.ok()?.try_into().ok()
        })
        .collect()
}

pub fn read_line<T>(filename: &str) -> Option<Vec<Vec<T>>>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug, // This allows for parsing other types if needed
{
    fs::read_to_string(filename)
        .ok()?
        .lines()
        .map(|x| {
            let parsed_line: Result<Vec<T>, _> =
                x.split_whitespace().map(|y| y.parse::<T>()).collect();
            parsed_line.ok()
        })
        .collect()
}
