use std::collections::HashMap;

use itertools::Itertools;

advent_of_code::solution!(4);

const DIRS: [(isize, isize); 8] = [
    (-1, 0),
    (-1, -1),
    (0, -1),
    (1, -1),
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
];

#[allow(unused_variables)]
pub fn part_one(input: &str) -> Option<usize> {
    let mut grid = HashMap::new();
    for (y, row) in input.lines().enumerate() {
        for (x, val) in row.char_indices() {
            if val == '@' {
                grid.insert((x as isize, y as isize), val);
            }
        }
    }

    let result = grid
        .keys()
        .filter(|&(x, y)| {
            DIRS.iter()
                .filter(|(dx, dy)| grid.contains_key(&(*x + *dx, *y + *dy)))
                .count()
                < 4
        })
        .count();

    Some(result)
}

#[allow(unused_variables)]
pub fn part_two(input: &str) -> Option<usize> {
    let mut grid = HashMap::new();
    for (y, row) in input.lines().enumerate() {
        for (x, val) in row.char_indices() {
            if val == '@' {
                grid.insert((x as isize, y as isize), val);
            }
        }
    }

    let mut remove = grid
        .keys()
        .filter(|&(x, y)| {
            DIRS.iter()
                .filter(|(dx, dy)| grid.contains_key(&(*x + *dx, *y + *dy)))
                .count()
                < 4
        })
        .copied()
        .collect_vec();
    let mut result = 0;
    while !remove.is_empty() {
        result += remove.len();
        remove.iter().for_each(|c| {
            grid.remove(c);
        });
        remove.clear();
        remove = grid
            .keys()
            .filter(|&(x, y)| {
                DIRS.iter()
                    .filter(|(dx, dy)| grid.contains_key(&(*x + *dx, *y + *dy)))
                    .count()
                    < 4
            })
            .copied()
            .collect_vec();
    }

    Some(result)
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
