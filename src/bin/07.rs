use std::collections::{HashMap, HashSet};

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<usize> {
    let (_, splits) = input.lines().fold(
        (HashSet::new(), 0),
        |(tachyon_beams, existing_splits), line| {
            let mut new_splits = 0;
            let next_tachyons = line
                .chars()
                .enumerate()
                .filter_map(|(i, x)| {
                    match x {
                        // our initial tachyon beam
                        'S' => Some(vec![i]),
                        '^' => {
                            if tachyon_beams.contains(&i) {
                                // splits if there was a tachyon above this
                                new_splits += 1;
                                Some(vec![i - 1, i + 1])
                            } else {
                                // else does nothing
                                None
                            }
                        }
                        '.' => {
                            // allow existing tachyon beams to fall through empty space
                            if tachyon_beams.contains(&i) {
                                Some(vec![i])
                            } else {
                                None
                            }
                        }
                        _ => None,
                    }
                })
                .flatten()
                .collect::<HashSet<usize>>();

            (next_tachyons, new_splits + existing_splits)
        },
    );

    Some(splits)
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(
        input
            .lines()
            .fold(HashMap::new(), |tachyon_beams, line| {
                line.chars()
                    .enumerate()
                    .filter_map(|(i, x)| {
                        // find out in how many realities a tachyon beam reaches position 'i'
                        let realities = *tachyon_beams.get(&i).unwrap_or(&0);

                        match x {
                            // our initial tachyon beam
                            'S' => Some(vec![(i, 1)]),
                            '^' => {
                                if realities > 0 {
                                    Some(vec![(i - 1, realities), (i + 1, realities)])
                                } else {
                                    None
                                }
                            }
                            '.' => {
                                if realities > 0 {
                                    Some(vec![(i, realities)])
                                } else {
                                    None
                                }
                            }
                            _ => None,
                        }
                    })
                    .flatten()
                    .fold(HashMap::new(), |mut acc, (pos, realities)| {
                        *acc.entry(pos).or_insert(0) += realities;
                        acc
                    })
            })
            .values()
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
