use itertools::Itertools;

advent_of_code::solution!(6);

#[allow(unused_variables)]
pub fn part_one(input: &str) -> Option<usize> {
    let ops = input
        .lines()
        .rev()
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .collect_vec();
    let nums: Vec<Vec<usize>> = input
        .lines()
        .rev()
        .skip(1)
        .map(|l| {
            l.split_ascii_whitespace()
                .flat_map(|n| n.parse())
                .collect_vec()
        })
        .collect_vec();
    let mut res = 0;
    for (idx, &ops) in ops.iter().enumerate() {
        match ops {
            "*" => res += nums.iter().fold(1, |acc, row| acc * row[idx]),
            "+" => res += nums.iter().fold(0, |acc, row| acc + row[idx]),
            _ => unreachable!(),
        };
    }
    Some(res)
}

#[allow(unused_variables)]
pub fn part_two(input: &str) -> Option<usize> {
    let ops = input
        .lines()
        .rev()
        .next()
        .unwrap()
        .char_indices()
        .filter(|&(_, c)| !c.is_ascii_whitespace())
        .collect_vec();
    let nums: Vec<Vec<char>> = input
        .lines()
        .rev()
        .skip(1)
        .collect_vec()
        .into_iter()
        .rev()
        .map(|l| l.chars().collect_vec())
        .collect_vec();

    // (idx, len, ops)
    let ops = ops
        .iter()
        .zip(
            &ops[1..]
                .iter()
                .chain(std::iter::once(&(nums[0].len() + 1, ' ')))
                .collect_vec(),
        )
        .map(|(&first, &next)| (first.0, next.0 - first.0 - 1, first.1))
        .collect_vec();

    let mut total = 0;
    let mut buf = String::new();
    for (idx, len, ops) in ops {
        let mut sub_total: usize = if ops == '*' { 1 } else { 0 };
        for col in 0..len {
            buf.clear();
            nums.iter().for_each(|row| {
                buf.push(row[idx + col]);
            });
            match ops {
                '*' => sub_total *= buf.trim().parse::<usize>().unwrap(),
                '+' => sub_total += buf.trim().parse::<usize>().unwrap(),
                _ => unreachable!(),
            };
        }
        total += sub_total;
    }
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
