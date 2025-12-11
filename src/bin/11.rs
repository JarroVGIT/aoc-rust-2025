use std::collections::HashMap;

use itertools::Itertools;

advent_of_code::solution!(11);

pub fn dfs<'a>(
    device: &'a str,
    connections: &HashMap<&'a str, Vec<&'a str>>,
    cache: &mut HashMap<&'a str, usize>,
) -> usize {
    if let Some(r) = cache.get(device) {
        return *r;
    }
    if device == "out" {
        return 1;
    }
    let mut r = 0;
    if let Some(outputs) = connections.get(device) {
        for output in outputs {
            r += dfs(output, connections, cache)
        }
    }
    cache.insert(device, r);
    r
}

pub fn dfs2<'a>(
    device: &'a str,
    connections: &HashMap<&'a str, Vec<&'a str>>,
    seen_dac: bool,
    seen_fft: bool,
    cache: &mut HashMap<(&'a str, bool, bool), usize>,
) -> usize {
    let seen_dac = seen_dac || device == "dac";
    let seen_fft = seen_fft || device == "fft";

    if device == "out" {
        return if seen_dac && seen_fft { 1 } else { 0 };
    }

    let key = (device, seen_dac, seen_fft);
    if let Some(r) = cache.get(&key) {
        return *r;
    }

    let mut r = 0;
    if let Some(outputs) = connections.get(device) {
        for output in outputs {
            r += dfs2(output, connections, seen_dac, seen_fft, cache)
        }
    }
    cache.insert(key, r);
    r
}

#[allow(unused_variables)]
pub fn part_one(input: &str) -> Option<usize> {
    let connections: HashMap<&str, Vec<&str>> = input
        .lines()
        .map(|l| {
            let parts = l.split(' ').collect_vec();
            let key = parts[0].strip_suffix(':').unwrap();
            let outputs = parts.into_iter().skip(1).collect();
            (key, outputs)
        })
        .collect();
    let mut cache = HashMap::new();
    let result = dfs("you", &connections, &mut cache);
    Some(result)
}

#[allow(unused_variables)]
pub fn part_two(input: &str) -> Option<usize> {
    let connections: HashMap<&str, Vec<&str>> = input
        .lines()
        .map(|l| {
            let parts = l.split(' ').collect_vec();
            let key = parts[0].strip_suffix(':').unwrap();
            let outputs = parts.into_iter().skip(1).collect();
            (key, outputs)
        })
        .collect();

    // cache on (device, seen_dac bool, seen_fft bool)
    let mut cache = HashMap::new();
    let result = dfs2("svr", &connections, false, false, &mut cache);
    Some(result)
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
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(2));
    }
}
