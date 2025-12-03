advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let instructions = parse(input);

    let mut dial_pos = 50;
    let mut zero_counts = 0;

    for (direction, magnitude) in instructions {
        dial_pos += direction * magnitude;
        dial_pos = dial_pos.rem_euclid(100);

        if dial_pos == 0 {
            zero_counts += 1;
        }
    }

    Some(zero_counts)
}

pub fn part_two(input: &str) -> Option<i32> {
    let instructions = parse(input);

    let mut dial_pos = 50;
    let mut zero_counts = 0;

    for (direction, magnitude) in instructions {
        let mut remaining_magnitude = magnitude;
        while remaining_magnitude > 0 {
            let next_zero_in = if dial_pos == 0 {
                100
            } else if direction == -1 {
                dial_pos
            } else {
                100 - dial_pos
            };

            let turn_for = next_zero_in.min(remaining_magnitude);

            dial_pos += direction * turn_for;
            dial_pos = dial_pos.rem_euclid(100);

            remaining_magnitude -= turn_for;

            if turn_for >= next_zero_in {
                zero_counts += 1;
            }
        }
    }

    Some(zero_counts)
}

fn parse(input: &str) -> Vec<(i32, i32)> {
    input
        .lines()
        .map(|x| x.split_at(1))
        .map(|(d, m)| (if d == "L" { -1 } else { 1 }, m.parse::<i32>().unwrap()))
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
