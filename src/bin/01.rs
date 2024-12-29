advent_of_code::solution!(1);
use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<i32> {
    let mut col1 = Vec::new();
    let mut col2 = Vec::new();

    for line in input.lines() {
        let numbers: Vec<i32> = line
            .split_whitespace()
            .filter_map(|n| n.parse::<i32>().ok())
            .collect();
        col1.push(numbers[0]);
        col2.push(numbers[1]);
    }
    col1.sort();
    col2.sort();
    let mut sum = 0;
    for i in 0..col1.len() {
        sum += (col1[i] - col2[i]).abs();
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut col1 = Vec::new();
    let mut map = HashMap::new();
    for line in input.lines() {
        let numbers: Vec<i32> = line
            .split_whitespace()
            .filter_map(|n| n.parse::<i32>().ok())
            .collect();
        col1.push(numbers[0]);
        map.entry(numbers[1]).and_modify(|e| *e += 1).or_insert(1);
    }

    let mut sum = 0;

    for x in col1.iter() {
        if map.contains_key(x) {
            sum += *x * map.get(x).unwrap();
        }
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
