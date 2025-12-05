use std::ops::RangeInclusive;

use itertools::Itertools;

advent_of_code::solution!(5);

#[allow(unused_variables)]
pub fn part_one(input: &str) -> Option<usize> {
    let (ranges, ingredients) = input.split_once("\n\n").unwrap();
    let ranges: Vec<RangeInclusive<usize>> = ranges
        .lines()
        .map(|l| {
            let (lo, hi) = l.split_once('-').unwrap();
            lo.parse().unwrap()..=hi.parse().unwrap()
        })
        .collect_vec();

    let result = ingredients
        .lines()
        .map(|l| l.parse::<usize>().unwrap())
        .filter(|i| ranges.iter().any(|r| r.contains(i)))
        .count();
    Some(result)
}

#[allow(unused_variables)]
pub fn part_two(input: &str) -> Option<usize> {
    let (ranges, _) = input.split_once("\n\n").unwrap();
    let mut ranges = ranges
        .lines()
        .map(|l| {
            let (lo, hi) = l.split_once('-').unwrap();
            let lo = lo.parse::<usize>().unwrap();
            let hi = hi.parse::<usize>().unwrap();
            (lo, hi)
        })
        .collect_vec();
    ranges.sort_by_key(|(lo, _)| *lo);
    let mut condensed = vec![];
    let mut last = (0, 0);
    for (lo, hi) in ranges {
        if last.1 < lo {
            condensed.push((lo, hi));
            last = (lo, hi);
        } else if last.1 == lo {
            // extend last range
            condensed.pop();
            condensed.push((last.0, hi));
            last = (last.0, hi);
        } else {
            // lo < than last hi.
            if hi <= last.1 {
                // range completely in last range.
                continue;
            } else {
                condensed.pop();
                condensed.push((last.0, hi));
                last = (last.0, hi);
            }
        }
    }
    let result = condensed.iter().map(|(lo, hi)| hi - lo + &1).sum();
    Some(result)
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
