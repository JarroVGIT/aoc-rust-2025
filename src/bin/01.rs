advent_of_code::solution!(1);

#[allow(unused_variables)]
pub fn part_one(input: &str) -> Option<usize> {
    let mut dial = 50;
    let mut zero = 0;
    input.lines().for_each(|l| {
        let (dir, count) = l.split_at(1);
        let count = count.parse::<i32>().unwrap();
        let count = count % 100;
        match dir {
            "L" => dial -= count,
            _ => dial += count,
        }
        if dial >= 100 {
            dial -= 100;
        }
        if dial < 0 {
            dial += 100;
        }
        if dial == 0 {
            zero += 1;
        }
    });
    Some(zero as usize)
}

#[allow(unused_variables)]
pub fn part_two(input: &str) -> Option<usize> {
    let mut dial = 50;
    let mut zero = 0;
    input.lines().for_each(|l| {
        let (dir, count) = l.split_at(1);
        let count = count.parse::<i32>().unwrap();
        zero += count / 100;
        let count = count % 100;
        match dir {
            "L" => dial -= count,
            _ => dial += count,
        }
        if dial >= 100 {
            dial -= 100;
            zero += 1;
        }
        if dial < 0 {
            dial += 100;
            zero += 1;
        }
    });
    Some(zero as usize)
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
        assert_eq!(result, Some(6));
    }
}
