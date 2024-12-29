advent_of_code::solution!(2);

fn parse(input: &str) -> Vec<Vec<i32>> {
    let mut numbers = Vec::new();
    for line in input.lines() {
        let cur: Vec<i32> = line
            .split_whitespace()
            .filter_map(|e| e.parse::<i32>().ok())
            .collect();
        numbers.push(cur);
    }
    numbers
}
pub fn part_one(input: &str) -> Option<u32> {
    let numbers = parse(input);
    let mut safe = 0;
    #[derive(PartialEq, Clone, Copy)]
    enum Direction {
        Increase,
        Decrease,
        NotSet,
    }

    for line in numbers {
        let mut prev = line[0];
        let mut delta = false;
        let mut dir_cor = false;
        let mut dir = Direction::NotSet;
        for i in 1..line.len() {
            if (prev - line[i]).abs() > 3 {
                delta = true;
            }
            let mut cur_dir = Direction::NotSet;
            if prev - line[i] > 0 {
                cur_dir = Direction::Increase;
            } else if prev - line[i] < 0 {
                cur_dir = Direction::Decrease;
            } else {
                dir_cor = true;
            }

            if dir == Direction::NotSet {
                dir = cur_dir;
            }
            if dir != cur_dir {
                dir_cor = true;
            }
            prev = line[i];
        }
        if delta || dir_cor {
            continue;
        }
        safe += 1;
    }
    Some(safe)
}

pub fn part_two(input: &str) -> Option<u32> {
    let numbers = parse(input);
    let mut safe = 0;

    for line in numbers {
        let mut joker = 1;
        let mut prev = line[0];
        let mut dir = 0;
        for i in 1..line.len() {
            let difference = line[i] - prev;
            if difference.abs() > 3 || difference == 0 {
                joker -= 1;
            } else if difference * dir < 0 {
                joker -= 1;
            }
            dir = if difference < 0 { -1 } else { 1 };
            prev = line[i];
        }
        safe += if joker >= 0 { 1 } else { 0 };
    }
    Some(safe)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
