use std::collections::HashSet;

type TpGrid = HashSet<(isize, isize)>;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u64> {
    let grid = build_grid(input);

    let mut num_accessible = 0;
    for el in &grid {
        if can_be_accessed(*el, &grid) {
            num_accessible += 1;
        }
    }

    Some(num_accessible)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut grid = build_grid(input);
    let initial_tp_count = grid.len();

    loop {
        let new_grid = grid
            .iter()
            .filter(|&&x| !can_be_accessed(x, &grid))
            .map(|&x| x)
            .collect::<TpGrid>();

        // check if the new grid hasn't been able to remove any tp, if so we're at a halting point
        if new_grid.len() == grid.len() {
            return Some(initial_tp_count - new_grid.len());
        }

        grid = new_grid;
    }
}

fn build_grid(input: &str) -> TpGrid {
    input
        .lines()
        .enumerate()
        .flat_map(|(r, row)| {
            row.chars()
                .enumerate()
                .filter_map(|(c, ch)| {
                    if ch == '@' {
                        Some((r as isize, c as isize))
                    } else {
                        None
                    }
                })
                .collect::<Vec<(isize, isize)>>()
        })
        .collect()
}

fn can_be_accessed(el: (isize, isize), grid: &HashSet<(isize, isize)>) -> bool {
    let num_adjacent = (-1..=1)
        .flat_map(|dr| (-1..=1).map(move |dc| (dr, dc)))
        .filter(|d| d.0 != 0 || d.1 != 0)
        .filter(|d| grid.contains(&(d.0 + el.0, d.1 + el.1)))
        .count();

    num_adjacent < 4
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
