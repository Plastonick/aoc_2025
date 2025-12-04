advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<usize> {
    Some(calc_joltage(input, 2))
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(calc_joltage(input, 12))
}

fn calc_joltage(input: &str, pick: u32) -> usize {
    input
        .lines()
        .map(|x| {
            x.split("")
                .flat_map(|x| x.parse::<usize>())
                .collect::<Vec<usize>>()
        })
        .map(|digits| max_joltage_for_array(&digits, 0, pick))
        .sum()
}

fn max_joltage_for_array(digits: &[usize], carry: usize, pick: u32) -> usize {
    // pick the best value such that we have at least `pick - 1` digits remaining to pick
    let (best_index, best_value) = digits
        .iter()
        .enumerate()
        .take(digits.len() - (pick as usize - 1))
        .reduce(|(acc_i, acc_v), (i, v)| if v > acc_v { (i, v) } else { (acc_i, acc_v) })
        .unwrap();

    let new_carry = carry + (best_value * 10_usize.pow(pick - 1));

    if pick == 1 {
        // if we only wanted to pick one, trivially return here
        new_carry
    } else {
        // otherwise, pick the next `pick - 1` digits
        max_joltage_for_array(&digits[best_index + 1..], new_carry, pick - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
