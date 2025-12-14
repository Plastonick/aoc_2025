advent_of_code::solution!(12, 1);

type PresentShape = String;

pub fn part_one(input: &str) -> Option<usize> {
    let parts = input.split("\n\n").collect::<Vec<&str>>();
    let shapes = parts
        .iter()
        .take(parts.len() - 1)
        .enumerate()
        .map(|(i, part)| part.lines().skip(1).collect::<PresentShape>())
        .collect::<Vec<_>>();

    let regions = parts
        .iter()
        .skip(parts.len() - 1)
        .flat_map(|region_str| {
            region_str
                .lines()
                .filter_map(|line| line.split_once(": "))
                .map(|(size_str, counts)| {
                    let (width, length) = size_str
                        .split_once('x')
                        .map(|(w, l)| (w.parse::<usize>().unwrap(), l.parse::<usize>().unwrap()))
                        .unwrap();
                    (
                        (width, length),
                        counts
                            .split_whitespace()
                            .filter_map(|x| x.parse::<usize>().ok())
                            .collect::<Vec<usize>>(),
                    )
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    Some(
        regions
            .into_iter()
            .filter(|(size, counts)| can_fit(&size, &counts, &shapes))
            .count(),
    )
}

fn can_fit(size: &(usize, usize), counts: &Vec<usize>, shapes: &Vec<PresentShape>) -> bool {
    // don't really have a clue atm how to achieve this!
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
