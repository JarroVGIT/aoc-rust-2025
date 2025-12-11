use std::collections::VecDeque;

use itertools::Itertools;

advent_of_code::solution!(10);

#[allow(unused_variables)]
pub fn part_one(input: &str) -> Option<usize> {
    let rows = input.lines().map(|l| {
        let parts = l.split(' ').collect_vec();
        let target = parts[0].strip_prefix('[').unwrap().strip_suffix(']').unwrap().char_indices().fold(0u32, |acc, (idx, c)| {
            if c == '#' {
                acc | 1 << idx
            } else {
                acc
            }
        });
        let last_pos = parts.len() - 1;
        let buttons = parts[1..last_pos].iter().map(|b| {
            (*b).strip_prefix('(').unwrap().strip_suffix(')').unwrap().split(',').fold(0u32, |acc, b| {
                let b = b.parse::<u32>().unwrap();
                acc | 1 << b
            })
        }).collect_vec();
        (target, buttons)
    }).collect_vec();

    let mut result = 0;

    for (target, row) in rows {
        let mut queue = VecDeque::new();

        // steps, state, last
        queue.push_back((0, 0, usize::MAX));
        while let Some((steps, state, last)) = queue.pop_front() {
            if state == target {
                result += steps;
                break;
            }
            for (idx, &button) in row.iter().enumerate() {
                if last == idx {
                    continue;
                }
                queue.push_back((steps+1, state ^ button, idx));
            }
        }
    }

    Some(result)
}

#[allow(unused_variables)]
pub fn part_two(input: &str) -> Option<usize> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
