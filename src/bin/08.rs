use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet, VecDeque},
};

use itertools::Itertools;

advent_of_code::solution!(8);

#[derive(Debug, PartialOrd, PartialEq, Clone, Copy)]
pub struct Distance(f64);

impl Eq for Distance {}
impl Ord for Distance {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn distance(a: (usize, usize, usize), b: (usize, usize, usize)) -> f64 {
    let dx = (a.0 as f64) - (b.0 as f64);
    let dy = (a.1 as f64) - (b.1 as f64);
    let dz = (a.2 as f64) - (b.2 as f64);
    (dx * dx + dy * dy + dz * dz).sqrt()
}

pub fn solve_1(input: &str, connections: usize) -> Option<usize> {
    let fuses: Vec<(usize, usize, usize)> = input
        .lines()
        .map(|l| {
            let mut line = l.split(',');
            let x = line.next().unwrap().parse().unwrap();
            let y = line.next().unwrap().parse().unwrap();
            let z = line.next().unwrap().parse().unwrap();
            (x, y, z)
        })
        .collect_vec();

    let mut distances = HashMap::new();
    for a in 0..fuses.len() {
        for b in (a + 1)..fuses.len() {
            let dis = distance(fuses[a], fuses[b]);
            distances.insert((a, b), Distance(dis));
        }
    }

    let shortest = distances
        .into_iter()
        .sorted_by_key(|(_, v)| *v)
        .take(connections);

    let mut adj: HashMap<usize, Vec<usize>> = HashMap::new();
    for ((a, b), _) in shortest {
        adj.entry(a).or_default().push(b);
        adj.entry(b).or_default().push(a);
    }

    let mut seen = HashSet::new();
    let mut components = vec![];

    for idx in 0..fuses.len() {
        if seen.contains(&idx) || !adj.contains_key(&idx) {
            continue;
        }

        let mut new_component = HashSet::new();
        let mut q = VecDeque::new();

        q.push_back(idx);
        seen.insert(idx);
        while let Some(coord) = q.pop_front() {
            new_component.insert(coord);
            for neighbor in adj.get(&coord).unwrap() {
                if seen.insert(*neighbor) {
                    q.push_back(*neighbor);
                }
            }
        }
        components.push(new_component);
    }

    components.sort_by_key(|c| c.len());
    let result: usize = components.iter().rev().take(3).map(|c| c.len()).product();

    Some(result)
}

#[allow(unused_variables)]
pub fn part_one(input: &str) -> Option<usize> {
    solve_1(input, 1000)
}

#[allow(unused_variables)]
pub fn part_two(input: &str) -> Option<usize> {
    let fuses: Vec<(usize, usize, usize)> = input
        .lines()
        .map(|l| {
            let mut line = l.split(',');
            let x = line.next().unwrap().parse().unwrap();
            let y = line.next().unwrap().parse().unwrap();
            let z = line.next().unwrap().parse().unwrap();
            (x, y, z)
        })
        .collect_vec();

    let total_boxes = input.lines().count();

    let mut distances = HashMap::new();
    for a in 0..fuses.len() {
        for b in (a + 1)..fuses.len() {
            let dis = distance(fuses[a], fuses[b]);
            distances.insert((a, b), Distance(dis));
        }
    }
    let mut shortest = distances
        .into_iter()
        .sorted_by_key(|(_, v)| *v)
        .rev()
        .map(|(k, _)| k)
        .collect_vec();
    let mut components: Vec<HashSet<usize>> = vec![];

    loop {
        let (a, b) = shortest.pop().unwrap();
        let component_a_idx = components.iter().position(|c| c.contains(&a));
        let component_b_idx = components.iter().position(|c| c.contains(&b));
        match (component_a_idx, component_b_idx) {
            (Some(c_a), Some(c_b)) => {
                if c_a == c_b {
                    continue;
                } else {
                    let new: HashSet<usize> =
                        components[c_a].union(&components[c_b]).copied().collect();
                    if c_a > c_b {
                        components.remove(c_a);
                        components.remove(c_b);
                    } else {
                        components.remove(c_b);
                        components.remove(c_a);
                    }

                    components.push(new);
                }
            } //both already in component.
            (Some(c_a), None) => {
                components[c_a].insert(b);
            }
            (None, Some(c_b)) => {
                components[c_b].insert(a);
            }
            (None, None) => components.push(HashSet::from([a, b])), // create new component.
        }
        if components.len() == 1 && components[0].len() == total_boxes {
            return Some(fuses[a].0 * fuses[b].0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = solve_1(&advent_of_code::template::read_file("examples", DAY), 10);
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(25272));
    }
}
