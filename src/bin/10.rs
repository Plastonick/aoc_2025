use std::collections::HashSet;

advent_of_code::solution!(10);

type Problem = (usize, Vec<usize>);

pub fn part_one(input: &str) -> Option<usize> {
    let problems = input.lines().map(parse_line).collect::<Vec<Problem>>();

    let mut sum = 0;
    for problem in problems {
        sum += fewest_presses_for(&problem, 0, HashSet::from([0]));
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<usize> {
    None
}

fn fewest_presses_for(problem: &Problem, presses: usize, current: HashSet<usize>) -> usize {
    let target = problem.0;

    if current.contains(&target) {
        return presses;
    }

    let new = current
        .iter()
        .flat_map(|curr| problem.1.iter().map(move |adder| curr ^ adder))
        .collect::<HashSet<usize>>();

    fewest_presses_for(&problem, presses + 1, new)
}

fn parse_line(line: &str) -> Problem {
    let mut chars = line.chars().into_iter();

    // represented as a binary number
    let desired_lights = chars
        .by_ref()
        .skip(1)
        .take_while(|x| x != &']')
        .map(|x| match x {
            '.' => 0,
            '#' => 1,
            _ => panic!("Unexpected light char!"),
        })
        .enumerate()
        .map(|(i, d)| d * 2_usize.pow(i as u32))
        .sum::<usize>();

    // represented as binary numbers
    let switches = chars
        .by_ref()
        .skip(1)
        .take_while(|x| x != &'{')
        .collect::<String>() // this is eugh, all this silly string walking to then just collect...
        .split(' ')
        .map(|str| {
            str.trim_matches(|x| match x {
                ' ' | '(' | ')' => true,
                _ => false,
            })
            .split(',')
            .filter_map(|d| d.parse::<usize>().ok())
            .map(|d| 2_usize.pow(d as u32))
            .sum::<usize>()
        })
        .collect::<Vec<usize>>();

    (desired_lights, switches)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
