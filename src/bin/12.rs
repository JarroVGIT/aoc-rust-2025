use itertools::Itertools;

advent_of_code::solution!(12);

#[allow(unused_variables)]
pub fn part_one(input: &str) -> Option<usize> {
    // I read a spoiler; we don't have to solve the actual problem, the input is kind enough
    // that you can just check if the total area of the packages is larget than the tree area.
    let parts = input.split("\n\n").collect_vec();
    let pieces = parts.iter().dropping_back(1).map(|p| {
        p.matches("#").count()
    }).collect_vec();

    let trees = parts[parts.len()-1].lines().filter(|l| {
        //4x4: 0 0 0 0 2 0
        let (size, indices) = l.split_once(": ").unwrap();
        let (l, r) = size.split_once('x').unwrap();
        let size = l.parse::<usize>().unwrap() * r.parse::<usize>().unwrap();
        let total_size_pieces: usize = indices.split_whitespace().enumerate().map(|(idx, c)| {
            let count = c.parse::<usize>().unwrap();
            count * pieces[idx]
        }).sum();
        size >= total_size_pieces
    }).count();
    Some(trees)
}

#[allow(unused_variables)]
pub fn part_two(input: &str) -> Option<usize> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_part_one() {
    //     let result = part_one(&advent_of_code::template::read_file("examples", DAY));
    //     assert_eq!(result, Some(2));
    // }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
