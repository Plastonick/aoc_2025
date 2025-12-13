use std::collections::{HashMap, HashSet};

advent_of_code::solution!(8);

type Coordinate = Vec<usize>;

pub fn part_one(input: &str) -> Option<usize> {
    #[cfg(test)]
    let num_connections = 10;
    #[cfg(not(test))]
    let num_connections = 1000;

    let coords = build_coords(input);
    let distances = build_distances(&coords);

    let connections: HashMap<usize, Vec<usize>> =
        distances
            .iter()
            .take(num_connections)
            .fold(HashMap::new(), |mut acc, ((i, j), _)| {
                acc.entry(*i).or_insert(Vec::new()).push(*j);
                acc.entry(*j).or_insert(Vec::new()).push(*i);
                acc
            });

    let all_positions = connections.keys().map(|x| x.to_owned()).collect::<Vec<_>>();
    let all_pools = build_pools(&connections, all_positions);

    let mut pool_sizes = all_pools.iter().map(|x| x.len()).collect::<Vec<usize>>();
    pool_sizes.sort();

    let skip = pool_sizes.len() - 3;
    let largest = pool_sizes.into_iter().skip(skip).collect::<Vec<usize>>();

    largest.into_iter().reduce(|a, b| a * b)
}

fn build_pools(
    connections: &HashMap<usize, Vec<usize>>,
    all_positions: Vec<usize>,
) -> Vec<HashSet<usize>> {
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
    all_pools
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
pub fn part_two(input: &str) -> Option<usize> {
    let coords = build_coords(input);
    let distances = build_distances(&coords);

    let mut all_pools: Vec<HashSet<usize>> = Vec::new();
    let mut pos_to_pool = coords.iter().map(|_| None).collect::<Vec<Option<usize>>>();

    let mut last_added = None;
    for ((i, j), _) in distances {
        let mapped_positions = pos_to_pool.iter().filter_map(|x| x.to_owned());
        let (min, max) = (
            mapped_positions.clone().min(),
            mapped_positions.clone().max(),
        );

        if mapped_positions.collect::<Vec<_>>().len() == coords.len() && min == max {
            break;
        }

        last_added = Some((i, j));

        let i_pool = pos_to_pool.get(i).unwrap().to_owned();
        let j_pool = pos_to_pool.get(j).unwrap().to_owned();

        if i_pool.is_some() && j_pool.is_some() && i_pool == j_pool {
            // i and j are already part of the same pool, nothing to do
            continue;
        }

        if i_pool.is_none() && j_pool.is_none() {
            // neither are part of a new pool, initialize a new pool and add them in!
            let new_pool = HashSet::from([i, j]);
            all_pools.push(new_pool);
            let pool_index = all_pools.len() - 1;

            pos_to_pool[i] = Some(pool_index);
            pos_to_pool[j] = Some(pool_index);

            continue;
        }

        if i_pool.is_some() != j_pool.is_some() {
            // one is part of a pool, the other isn't! Add the missing one to the pool
            let (pool, missing) = if i_pool.is_some() {
                (i_pool.unwrap(), j)
            } else {
                (j_pool.unwrap(), i)
            };

            all_pools.get_mut(pool).unwrap().insert(missing);
            pos_to_pool[missing] = Some(pool);

            continue;
        }

        if i_pool.is_some() && j_pool.is_some() {
            // they're both in different pools, we need to merge one into the other
            let pool_j = all_pools.get(j_pool.unwrap()).unwrap().clone();
            let pool_i = all_pools.get_mut(i_pool.unwrap()).unwrap();

            // point all elements in pool j to pool i
            for pos in &pool_j {
                pos_to_pool[*pos] = i_pool.clone();
            }
            // extend pool i by the elements of j
            pool_i.extend(pool_j);

            continue;
        }

        panic!("Unexpected condition");
    }

    last_added.map(|(i, j)| coords[i][0] * coords[j][0])
}

fn build_distances(coords: &Vec<Vec<usize>>) -> Vec<((usize, usize), f64)> {
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

    distances
}

fn build_coords(input: &str) -> Vec<Vec<usize>> {
    let coords = input
        .lines()
        .map(|line| {
            line.split(',')
                .filter_map(|ord| ord.parse::<usize>().ok())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    coords
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
        assert_eq!(result, Some(25272));
    }
}
