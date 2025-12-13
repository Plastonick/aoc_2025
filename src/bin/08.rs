use std::collections::{HashMap, HashSet};

advent_of_code::solution!(8);

type Coordinate = Vec<usize>;

pub fn part_one(input: &str) -> Option<usize> {
    #[cfg(test)]
    let num_connections = 10;
    #[cfg(not(test))]
    let num_connections = 1000;

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

    let mut connections: HashMap<usize, Vec<usize>> =
        distances
            .iter()
            .take(num_connections)
            .fold(HashMap::new(), |mut acc, ((i, j), _)| {
                acc.entry(*i).or_insert(Vec::new()).push(*j);
                acc.entry(*j).or_insert(Vec::new()).push(*i);
                acc
            });

    let all_positions = connections.keys().map(|x| x.to_owned()).collect::<Vec<_>>();
    let mut seen_positions: HashSet<usize> = HashSet::new();
    let mut all_pools = Vec::new();
    for position in all_positions {
        if seen_positions.contains(&position) {
            continue;
        }

        let pool = flood(position, &connections);

        seen_positions.extend(&pool);
        all_pools.push(pool);
    }

    let mut pool_sizes = all_pools.iter().map(|x| x.len()).collect::<Vec<usize>>();
    pool_sizes.sort();

    let skip = pool_sizes.len() - 3;
    let largest = pool_sizes.into_iter().skip(skip).collect::<Vec<usize>>();

    largest.into_iter().reduce(|a, b| a * b)
}

fn flood(from: usize, connections: &HashMap<usize, Vec<usize>>) -> HashSet<usize> {
    let mut pool = HashSet::from([from]);
    let mut wave = vec![from];
    while wave.len() > 0 {
        let next_wave: Vec<usize> = wave
            .iter()
            .flat_map(|pos| {
                connections
                    .get(pos)
                    .unwrap()
                    .iter()
                    .filter(|n| !pool.contains(n))
            })
            .map(|x| x.to_owned())
            .collect();

        pool.extend(next_wave.clone());

        wave = next_wave;
    }

    pool
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
