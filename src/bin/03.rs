use regex::Regex;

advent_of_code::solution!(3);

fn remove_dont_do(input: &str) -> String {
    let no_do: Vec<&str> = input.split("do()").collect();
    let mut no_dont: Vec<&str> = Vec::new();
    for line in no_do.iter() {
        if let Some(index) = line.find("don't()") {
            no_dont.push(&line[..index]);
        } else {
            no_dont.push(line);
        };
    }
    let result = no_dont.concat();
    // let re = Regex::new(r"don't\(\)(.*?)(do\(\)|$)").unwrap();
    // let result = re.replace_all(input, "").to_string();
    result
}

fn multiply_and_add(input: &str) -> u32 {
    let mut multi_sum = 0;
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut numbers: Vec<(u32, u32)> = Vec::new();
    for cap in re.captures_iter(input) {
        let x: u32 = cap[1].parse().unwrap();
        let y: u32 = cap[2].parse().unwrap();
        numbers.push((x, y));
    }
    for pair in numbers {
        multi_sum += pair.0 * pair.1;
    }
    multi_sum
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(multiply_and_add(input))
}

pub fn part_two(input: &str) -> Option<u32> {
    let processed_string = remove_dont_do(input);
    Some(multiply_and_add(&processed_string))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
