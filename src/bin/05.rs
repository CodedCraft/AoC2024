use std::collections::{HashMap, HashSet};
advent_of_code::solution!(5);

fn parse_input(input: &str) -> (HashMap<i32, HashSet<i32>>, Vec<Vec<i32>>) {
    let input: Vec<&str> = input.split("\n\n").collect();
    let mut prio_map: HashMap<i32, HashSet<i32>> = HashMap::new();
    for line in input[0].lines() {
        if let Some((key, value)) = line.split_once('|') {
            if let (Ok(k), Ok(v)) = (key.parse::<i32>(), value.parse::<i32>()) {
                prio_map.entry(k).or_insert_with(HashSet::new).insert(v);
            }
        }
    }

    let mut print_order =  Vec::new();
    for line in input[1].lines() {
        let order: Vec<i32> = line.split(',')
            .map(|v| v.parse::<i32>().unwrap())
            .collect();
        print_order.push(order);
    }
    (prio_map, print_order)
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut correct_numbers = 0;
    let (prio_map, print_order) = parse_input(input);
    for mut line in print_order {
        let mut correct =  true;
        let middle = line[(line.len()-1)/2];
        while let Some(num) = line.pop() {
            for x in line.iter() {
                if let Some(val) = prio_map.get(&num) {
                    if val.contains(x) {
                        correct = false;
                    }
                }
            }
        }
        if correct {
            correct_numbers += middle;
        }
    }
    Some(correct_numbers as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (prio_map, print_order) = parse_input(input);
    let mut middle_sum = 0;
    for mut line in print_order {
        let mut correct =  true;
        let mut to_fix =  line.clone();
        while let Some(num) = line.pop() {
            for x in line.iter() {
                if let Some(val) = prio_map.get(&num) {
                    if val.contains(x) {
                        correct = false;
                    }
                }
            }
        }


        if !correct {
            while let Some(num) = to_fix.pop() {
                if let Some(x) = prio_map.get(&num){
                    println!("Num: {} -> {:?}", num, x);
            }
            }
            println!("{:?}", to_fix);
        }

        println!("----------------------------------------------------");
    }

    Some(middle_sum as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
