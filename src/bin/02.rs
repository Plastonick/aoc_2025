advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<usize> {
    let ranges = get_ranges(input);
    let repeat_checks = [2];

    Some(
        ranges
            .into_iter()
            .map(|(a, b)| sum_invalid_ids(a, b, &repeat_checks))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let ranges = get_ranges(input);
    let repeat_checks = [2, 3, 4, 5, 6, 7, 8, 9, 10];

    Some(
        ranges
            .into_iter()
            .map(|(a, b)| sum_invalid_ids(a, b, &repeat_checks))
            .sum(),
    )
}

fn sum_invalid_ids(from: usize, to: usize, check_for: &[usize]) -> usize {
    let mut sum = 0;

    for i in from..=to {
        if is_invalid(i, check_for) {
            sum += i
        }
    }

    sum
}

fn is_invalid(id: usize, check_for: &[usize]) -> bool {
    for i in check_for {
        if has_repeated_digit_seq(id, *i) {
            return true;
        }
    }

    false
}

fn has_repeated_digit_seq(id: usize, times: usize) -> bool {
    let digits = (id.ilog10() + 1) as usize;

    if digits < times {
        return false;
    }

    if digits % times != 0 {
        // can't be repeated n times if it's not got multiple of n digits!
        return false;
    }

    let scale = 10_usize.pow((digits / times) as u32);

    let mut id_rem = id;
    let rhs = id_rem % scale;
    id_rem /= scale;

    while id_rem > 0 {
        let next_piece = id_rem % scale;

        if next_piece != rhs {
            return false;
        }

        id_rem /= scale;
    }

    true
}

fn get_ranges(input: &str) -> Vec<(usize, usize)> {
    input
        .trim()
        .split(',')
        .map(|x| x.split_once('-').unwrap())
        .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
