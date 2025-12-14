advent_of_code::solution!(9);

type Point = (usize, usize);

pub fn part_one(input: &str) -> Option<usize> {
    let coords = parse(input);

    coords
        .iter()
        .enumerate()
        .flat_map(|(i, a)| {
            coords
                .iter()
                .skip(i + 1)
                .map(|b| (a.0.abs_diff(b.0) + 1) * (a.1.abs_diff(b.1) + 1))
        })
        .max()
}

pub fn part_two(input: &str) -> Option<usize> {
    let coords = parse(input);

    let mut sizes = coords
        .iter()
        .enumerate()
        .flat_map(|(i, a)| {
            coords.iter().skip(i + 1).map(move |b| {
                let size = (a.0.abs_diff(b.0) + 1) * (a.1.abs_diff(b.1) + 1);

                ((a, b), size)
            })
        })
        .collect::<Vec<_>>();

    sizes.sort_by(|(_, a), (_, b)| b.cmp(a));

    sizes
        .into_iter()
        .find(|((a, b), size)| is_covered(**a, **b, &coords))
        .map(|(_, size)| size)
}

fn is_covered(a: Point, b: Point, coords: &Vec<Point>) -> bool {
    // the coordinates are guaranteed to be in order
    // we should be able to iterate over the coordinates and determine if there's ever a breaking intersection...

    let (maxima, minima) = ((a.0.max(b.0), a.1.max(b.1)), (a.0.min(b.0), a.1.min(b.1)));

    for i in 0..coords.len() {
        let left = coords[i];
        let right = coords[(i + 1) % coords.len()];
        let centre = ((left.0 + right.0) / 2, (left.1 + right.1) / 2);

        // is either of the left or right sides of the line segment in our rectangle?
        let left_inside = point_inside(&left, &maxima, &minima);
        let centre_inside = point_inside(&centre, &maxima, &minima);
        let right_inside = point_inside(&right, &maxima, &minima);

        // if so, the rectangle is partially outside-of the bounds at least!
        if left_inside != right_inside || left_inside != centre_inside {
            return false;
        }
    }

    println!("apparently this rectangle is covered!");
    dbg!(maxima, minima);

    true
}

fn point_inside(point: &Point, maxima: &Point, minima: &Point) -> bool {
    if point.0 >= maxima.0 {
        false
    } else if point.1 >= maxima.1 {
        false
    } else if point.0 <= minima.0 {
        false
    } else if point.1 <= minima.1 {
        false
    } else {
        true
    }
}

fn parse(input: &str) -> Vec<Point> {
    let coords = input
        .lines()
        .filter_map(|l| {
            l.split_once(',')
                .map(|(x, y)| (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()))
        })
        .collect::<Vec<_>>();
    coords
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24));
    }
}
