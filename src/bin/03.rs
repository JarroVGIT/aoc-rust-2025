use itertools::Itertools;

advent_of_code::solution!(3);

#[allow(unused_variables)]
pub fn part_one(input: &str) -> Option<usize> {
    let result: usize = input
        .lines()
        .map(|l| {
            let (_, hi) = l
                .char_indices()
                .take_while(|&(idx, _)| idx < l.len() - 1)
                .max_by_key(|&(_, c)| c.to_digit(10).unwrap())
                .unwrap();
            // max_by_key returns the last maximum :(
            let hi_idx = l.find(hi).unwrap();
            let lo = l
                .chars()
                .skip(hi_idx + 1)
                .map(|c| c.to_digit(10).unwrap())
                .max()
                .unwrap();
            let high = hi.to_digit(10).unwrap() as usize * 10;
            let low = lo as usize;
            high + low
        })
        .sum();
    Some(result)
}

#[allow(unused_variables)]
pub fn part_two(input: &str) -> Option<usize> {
    let result: usize = input
        .lines()
        .map(|l| {
            let nums = l
                .chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .enumerate()
                .collect_vec();
            let mut answer = 0;
            let mut last_idx = 0;
            for pos in 0..12 {
                let skip_last = 11 - pos as usize;
                let &(max_digit_idx, max_digit) = nums[last_idx..(nums.len() - skip_last)]
                    .iter()
                    .rev()
                    .max_by_key(|(_, d)| d)
                    .unwrap();
                last_idx = max_digit_idx + 1;
                answer += max_digit * 10usize.pow(12 - 1 - pos);
            }
            answer
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
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
