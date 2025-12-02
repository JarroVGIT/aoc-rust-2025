use std::ops::RangeInclusive;

use itertools::Itertools;

advent_of_code::solution!(2);

fn is_invalid(n: usize) -> bool {
    let digits = (n.checked_ilog10().unwrap_or(0) + 1) as usize;
    if digits % 2 != 0 {
        return false;
    }
    let half_digits = digits / 2;
    let left = n / 10usize.pow(half_digits as u32);
    let right = n - (left * 10usize.pow(half_digits as u32));
    left == right
}

fn is_invalid_p2(n: &[char]) -> Option<usize> {
    let len = n.len();
    for chunk in 1..=(len / 2) {
        let first_chunk = n.chunks(chunk).next().unwrap();
        if n.chunks(chunk).skip(1).all(|c| c == first_chunk) {
            return Some(
                n.iter()
                    .rev()
                    .enumerate()
                    .map(|(idx, &c)| c.to_digit(10).unwrap() as usize * 10usize.pow(idx as u32))
                    .sum::<usize>(),
            );
        }
    }
    None
}

#[allow(unused_variables)]
pub fn part_one(input: &str) -> Option<usize> {
    let ranges: Vec<RangeInclusive<usize>> = input
        .split(',')
        .map(|s| {
            let (lo, hi) = s.split_once('-').unwrap();
            lo.parse().unwrap()..=hi.parse().unwrap()
        })
        .collect_vec();

    let result: usize = ranges
        .iter()
        .map(|r| r.clone().filter(|&n| is_invalid(n)).sum::<usize>())
        .sum();
    Some(result)
}

#[allow(unused_variables)]
pub fn part_two(input: &str) -> Option<usize> {
    let ranges: Vec<RangeInclusive<usize>> = input
        .split(',')
        .map(|s| {
            let (lo, hi) = s.split_once('-').unwrap();
            lo.parse().unwrap()..=hi.parse().unwrap()
        })
        .collect_vec();

    let result: usize = ranges
        .iter()
        .map(|r| {
            r.clone()
                .filter_map(|n| {
                    let v = format!("{}", n).chars().collect_vec();
                    is_invalid_p2(&v)
                })
                .sum::<usize>()
        })
        .sum();
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
