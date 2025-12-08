use std::{cmp::Ordering, collections::HashMap};

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

#[allow(unused_variables)]
pub fn part_one(input: &str) -> Option<usize> {
    let fuses: Vec<(usize, usize, usize)> = input.lines().map(|l| {
        let mut line = l.split('-');
        let x = line.next().unwrap().parse().unwrap();
        let y = line.next().unwrap().parse().unwrap();
        let z = line.next().unwrap().parse().unwrap();
        (x,y,z)
    }).collect_vec();

    let mut distances = HashMap::new();
    for a in 0..fuses.len() {
        for b in (a+1)..fuses.len() {
            let dis = distance(fuses[a], fuses[b]);
            distances.insert((a,b), Distance(dis));
        }
    }

    let shortest: Vec<((usize, usize), Distance)> = distances.into_iter().sorted_by_key(|(_,v)| *v).take(1000).collect_vec();

    

    None
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
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
