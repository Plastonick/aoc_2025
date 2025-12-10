use std::collections::HashSet;

advent_of_code::solution!(8);

type Coordinate = Vec<usize>;

pub fn part_one(input: &str) -> Option<usize> {
    #[cfg(test)]
    let connections = 10;
    #[cfg(not(test))]
    let connections = 1000;

    let coords = input
        .lines()
        .map(|line| {
            line.split(',')
                .filter_map(|ord| ord.parse::<usize>().ok())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut distances = coords
        .iter()
        .enumerate()
        .flat_map(|(i, a)| {
            coords.iter().enumerate().filter_map(move |(j, b)| {
                if i >= j {
                    None
                } else {
                    Some(((i, j), dist(a, b)))
                }
            })
        })
        .collect::<Vec<((usize, usize), f64)>>();

    distances.sort_by(|(_, a_dist), (_, b_dist)| a_dist.total_cmp(b_dist));

    let mut circuits: Vec<HashSet<usize>> = vec![];
    for ((a, b), _) in distances.iter().take(connections) {
        let existing = circuits
            .iter()
            .enumerate()
            .find(|(_, x)| x.contains(a) || x.contains(b));

        println!(
            "trying to add {} -> {} and {} -> {}",
            a,
            coords
                .get(*a)
                .unwrap()
                .iter()
                .map(|x| format!("{}, ", x.to_string()))
                .collect::<String>(),
            b,
            coords
                .get(*b)
                .unwrap()
                .iter()
                .map(|x| format!("{}, ", x.to_string()))
                .collect::<String>()
        );

        println!();
        if let Some((pos, circuit)) = existing {
            let mut new_circuit = circuit.clone();
            new_circuit.insert(*a);
            new_circuit.insert(*b);

            circuits[pos] = new_circuit;
        } else {
            circuits.push(HashSet::from([*a, *b]));
        }
    }

    let merged_circuits = circuits
        .iter()
        .enumerate()
        .map(|(i, a)| {
            circuits.iter().skip(i).fold(a.clone(), |acc, b| {
                let union = b.union(&acc).map(|x| *x).collect::<HashSet<usize>>();

                if union.len() < a.len() + b.len() {
                    // there's some intersection, so use the merged
                    union
                } else {
                    acc
                }
            })
        })
        .map(|x| x.clone())
        .collect::<Vec<HashSet<usize>>>();

    let mut merged_sizes = merged_circuits
        .iter()
        .map(|x| x.len())
        .collect::<Vec<usize>>();
    merged_sizes.sort();

    dbg!(&merged_sizes);

    let skip = merged_sizes.len() - 3;
    let largest = merged_sizes.into_iter().skip(skip).collect::<Vec<usize>>();

    dbg!(&largest);

    // not 5780... too low?
    largest.into_iter().reduce(|a, b| a * b)
}

pub fn part_two(_input: &str) -> Option<usize> {
    None
}

fn dist(a: &Coordinate, b: &Coordinate) -> f64 {
    a.iter()
        .zip(b)
        .map(|(a, b)| a.abs_diff(*b).pow(2) as f64)
        .sum::<f64>()
        .sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
