use good_lp::{
    Constraint, Expression, ProblemVariables, Solution, SolverModel, constraint, microlp, variable,
};
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
        sum += fewest_presses_for_joltages_lp(&problem);
    }

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
fn fewest_presses_for_joltages_lp(problem: &Problem) -> usize {
    let (_, _, switch_list, joltages) = problem;

    let mut problem = ProblemVariables::new();

    let mut joltage_affected_by = joltages
        .iter()
        .map(|target| (*target, Vec::new()))
        .collect::<Vec<(usize, Vec<usize>)>>();
    let mut total_clicks: Expression = 0.into();
    let switch_clicks_counts = switch_list
        .iter()
        .enumerate()
        .map(|(i, switches)| {
            // which joltages are affected by these switches being clicked
            switches
                .iter()
                .for_each(|s| joltage_affected_by.get_mut(*s as usize).unwrap().1.push(i));

            // and handle clicking this as a variable
            let clicks = problem.add(variable().min(0).initial(0).integer());
            total_clicks += clicks;
            clicks
        })
        .collect::<Vec<_>>();

    let constraints = joltage_affected_by
        .iter()
        .map(|(target, switches_affected_by)| {
            let sum: Expression = switches_affected_by
                .iter()
                .filter_map(|i| switch_clicks_counts.get(*i))
                .fold(Expression::from(0), |acc, &var| acc + var);
            constraint!(sum == *target as i32)
        })
        .collect::<Vec<Constraint>>();

    let solution = problem
        .minimise(total_clicks)
        .using(microlp)
        .with_all(constraints)
        .solve()
        .unwrap();

    switch_clicks_counts
        .iter()
        // add 0.1 to avoid truncation issues for floating point "ints" that would truncate downwards
        .map(|x| (solution.value(x.clone()) + 0.1) as usize)
        .sum()
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
        assert_eq!(result, Some(333));
    }
}
