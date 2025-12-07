use advent_of_code::rotate_270;

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<usize> {
    let lines = input.trim().split("\n").collect::<Vec<&str>>();
    let (numbers_str, ops_str) = lines.split_at(lines.len() - 1);

    let numbers = numbers_str
        .iter()
        .map(|l| {
            l.trim()
                .split_whitespace()
                .filter_map(|num| num.parse().ok())
                .collect()
        })
        .collect::<Vec<Vec<usize>>>();

    let sum = get_ops(ops_str)
        .iter()
        .enumerate()
        .map(|(i, op)| {
            numbers
                .iter()
                .filter_map(|v| v.get(i))
                .copied() // horrible
                .reduce(|a, b| match op {
                    '+' => a + b,
                    '*' => a * b,
                    _ => panic!("Unexpected operation!"),
                })
                .iter()
                .sum::<usize>()
        })
        .sum();

    Some(sum)
}

pub fn part_two(input: &str) -> Option<usize> {
    let lines = input.trim().split("\n").collect::<Vec<&str>>();
    let (numbers_str, ops_str) = lines.split_at(lines.len() - 1);

    let longest_line = numbers_str.iter().map(|l| l.len()).max().unwrap();
    let matrix: Vec<Vec<char>> = numbers_str
        .iter()
        .map(|l| {
            vec![' '; longest_line]
                .iter()
                .enumerate()
                .filter_map(|(i, _)| l.chars().nth(i).or(Some(' ')))
                .collect()
        })
        .collect();

    let all_numbers = rotate_270(matrix)
        .iter()
        .map(|x| x.iter().collect::<String>().trim().to_owned())
        .collect::<Vec<String>>()
        .join("\n")
        .split("\n\n")
        .map(|numbers_str| {
            numbers_str
                .lines()
                .filter_map(|x| x.parse::<usize>().ok())
                .collect()
        })
        .collect::<Vec<Vec<usize>>>();

    let mut ops = get_ops(ops_str);
    ops.reverse();

    Some(
        all_numbers
            .into_iter()
            .enumerate()
            .filter_map(|(i, numbers)| {
                numbers.into_iter().reduce(|a, b| {
                    let op = ops.get(i).unwrap().clone();

                    match op {
                        '+' => a + b,
                        '*' => a * b,
                        _ => panic!("Unexpected operation!"),
                    }
                })
            })
            .sum(),
    )
}

fn get_ops(ops_str: &[&str]) -> Vec<char> {
    ops_str
        .iter()
        .flat_map(|l| {
            l.trim()
                .chars()
                .filter_map(|ch| if ch.is_whitespace() { None } else { Some(ch) })
                .collect::<Vec<char>>()
        })
        .collect::<Vec<char>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
