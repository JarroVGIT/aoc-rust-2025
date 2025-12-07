use std::collections::{HashMap, HashSet};

advent_of_code::solution!(7);

#[allow(unused_variables)]
pub fn part_one(input: &str) -> Option<usize> {
    let splitters: Vec<HashSet<usize>> = input
        .lines()
        .map(|l| {
            l.char_indices()
                .filter_map(|(idx, v)| if v == '^' { Some(idx) } else { None })
                .collect()
        })
        .collect();

    let start = input.lines().next()?.find('S')?;
    let mut beams = HashSet::new();
    beams.insert(start);

    let mut split_count = 0;
    for line in splitters {
        let mut next_beams: HashSet<usize> = beams.clone().difference(&line).copied().collect();
        for beam in beams.intersection(&line).copied() {
            split_count += 1;
            next_beams.insert(beam - 1);
            next_beams.insert(beam + 1);
        }
        beams = next_beams;
    }

    Some(split_count)
}

fn dfs(
    x: usize,
    y: usize,
    grid: &[Vec<char>],
    cache: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if y == grid.len() - 1 {
        return 1;
    }
    if let Some(&n) = cache.get(&(x, y)) {
        return n;
    } else {
        let n = if grid[y + 1][x] == '^' {
            dfs(x - 1, y + 1, grid, cache) + dfs(x + 1, y + 1, grid, cache)
        } else {
            dfs(x, y + 1, grid, cache)
        };
        cache.insert((x, y), n);
        return n;
    }
}

#[allow(unused_variables)]
pub fn part_two(input: &str) -> Option<usize> {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let mut cache = HashMap::new();
    let x = input.lines().next()?.find('S')?;
    Some(dfs(x, 0, &grid, &mut cache))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
