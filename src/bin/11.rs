use std::collections::HashMap;

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<usize> {
    let map = build_map(input);

    let mut wave = map.get("you").unwrap().to_owned();
    let mut n_paths = 0;
    while wave.len() > 0 {
        wave = wave
            .into_iter()
            .filter_map(|x| {
                if x == "out" {
                    n_paths += 1;
                    None
                } else {
                    map.get(&x).map(|x| x.to_owned())
                }
            })
            .flatten()
            .collect::<Vec<_>>();
    }

    Some(n_paths)
}

fn build_map(input: &str) -> HashMap<String, Vec<String>> {
    input
        .lines()
        .map(|x| {
            let (source_str, targets_str) = x.split_once(": ").unwrap();
            let targets = targets_str
                .split_whitespace()
                .map(|x| x.to_string())
                .collect::<Vec<String>>();

            (source_str.to_string(), targets)
        })
        .collect::<HashMap<String, Vec<String>>>()
}

pub fn part_two(input: &str) -> Option<usize> {
    #[cfg(not(test))]
    let input = input;

    #[cfg(test)]
    let input = get_test_example_part_2();

    let map = build_map(&input);

    let mut wave = map
        .get("svr")
        .unwrap()
        .to_owned()
        .into_iter()
        .map(|x| ((x, false, false), 1))
        .collect::<HashMap<(String, bool, bool), usize>>();

    let mut n_paths = 0;
    while wave.len() > 0 {
        wave = wave
            .into_iter()
            .filter_map(|((node, dac, fft), paths)| {
                let visited_dac = dac || node == "dac";
                let visited_fft = fft || node == "fft";

                if node == "out" {
                    if dac && fft {
                        n_paths += paths;
                    }

                    None
                } else {
                    map.get(&node).map(|x| {
                        x.to_owned()
                            .into_iter()
                            .map(move |x| ((x, visited_dac, visited_fft), paths))
                    })
                }
            })
            .flatten()
            .fold(HashMap::new(), |mut acc, ((node, dac, fft), paths)| {
                *acc.entry((node, dac, fft)).or_insert(0) += paths;
                acc
            });
    }

    Some(n_paths)
}

fn get_test_example_part_2() -> String {
    "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out"
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
