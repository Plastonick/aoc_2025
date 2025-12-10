use std::collections::HashSet;

advent_of_code::solution!(10);

// lights target (bin), switches (bin), switches (vec of targets), joltages
type Problem = (usize, Vec<usize>, Vec<Vec<usize>>, Vec<usize>);

pub fn part_one(input: &str) -> Option<usize> {
    let problems = input.lines().map(parse_line).collect::<Vec<Problem>>();

    let mut sum = 0;
    for problem in problems {
        sum += fewest_presses_for_lights(&problem, 0, HashSet::from([0]));
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<usize> {
    let problems = input.lines().map(parse_line).collect::<Vec<Problem>>();

    let mut sum = 0;
    for problem in problems {
        sum += fewest_presses_for_joltages(&problem, 0, HashSet::from([vec![0; problem.3.len()]]));
    }

    // 17599824 is too high
    Some(sum)
}

fn fewest_presses_for_lights(problem: &Problem, presses: usize, current: HashSet<usize>) -> usize {
    let target = problem.0;

    if current.contains(&target) {
        return presses;
    }

    let new = current
        .iter()
        .flat_map(|curr| problem.1.iter().map(move |adder| curr ^ adder))
        .collect::<HashSet<usize>>();

    fewest_presses_for_lights(&problem, presses + 1, new)
}

// possible solution is to represent the joltages in base-n
fn fewest_presses_for_joltages(
    problem: &Problem,
    presses: usize,
    current: HashSet<Vec<usize>>,
) -> usize {
    let target = &problem.3;

    if current.contains(target) {
        return presses;
    }

    let new = current
        .iter()
        .flat_map(|curr| {
            problem.2.iter().map(move |adder| {
                let mut new = curr.clone();

                // dbg!(curr, adder);
                for indx in adder {
                    new[*indx] += 1;
                }

                new
            })
        })
        .filter(|x| !x.iter().enumerate().any(|(i, &el)| el > target[i]))
        .collect::<HashSet<Vec<usize>>>();

    fewest_presses_for_joltages(&problem, presses + 1, new)
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
            .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();

    let switches_bin = switches
        .iter()
        .map(|switch| switch.iter().map(|&d| 2_usize.pow(d as u32)).sum())
        .collect::<Vec<usize>>();

    let joltages = chars
        .by_ref()
        .take_while(|x| x != &'}')
        .collect::<String>()
        .split(',')
        .filter_map(|d| d.parse::<usize>().ok())
        .collect::<Vec<usize>>();

    (desired_lights, switches_bin, switches, joltages)
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
        assert_eq!(result, Some(33));
    }
}
