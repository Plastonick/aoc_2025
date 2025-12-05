use std::collections::HashSet;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<usize> {
    let (fresh_ranges, ingredient_ids) = parse(input);

    Some(
        ingredient_ids
            .iter()
            .filter(|&id| {
                fresh_ranges
                    .iter()
                    .any(|(low, high)| id >= low && id <= high)
            })
            .count(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let (mut fresh_ranges, _) = parse(input);

    fresh_ranges.sort_by(|a, b| a.0.cmp(&b.0));

    let mut merged_ranges = Vec::new();
    let mut merged_range_indexes = HashSet::new();

    for (i, (l, h)) in fresh_ranges.iter().enumerate() {
        if merged_range_indexes.contains(&i) {
            // this range has already been merged
            continue;
        }

        let mut new_range = (*l, *h);
        for (i2, (l2, h2)) in fresh_ranges.iter().skip(i + 1).enumerate() {
            if *h2 < new_range.0 || new_range.1 < *l2 {
                // no clash, ignore this range

                continue;
            }

            // if there's a clash, increase the new range and set this range as skipped
            merged_range_indexes.insert(i2 + i + 1);
            new_range = (new_range.0.min(*l2), new_range.1.max(*h2));
        }

        merged_ranges.push(new_range)
    }

    Some(merged_ranges.iter().map(|(l, h)| 1 + h - l).sum())
}

fn parse(input: &str) -> (Vec<(usize, usize)>, Vec<usize>) {
    input
        .split_once("\n\n")
        .map(|(fresh_range_str, ingred_ids_str)| {
            (
                fresh_range_str
                    .lines()
                    .filter_map(|x| {
                        x.split_once('-')
                            .map(|(l, r)| (l.parse().unwrap(), r.parse().unwrap()))
                    })
                    .collect(),
                ingred_ids_str
                    .lines()
                    .filter_map(|x| x.parse().ok())
                    .collect(),
            )
        })
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }
}
