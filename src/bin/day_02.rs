use itertools::Itertools;
use petgraph::visit::depth_first_search;

#[derive(Debug)]
struct ID_Range {
    first: ID_instance,
    last: ID_instance,
}
impl ID_Range {
    fn new(first: ID_instance, last: ID_instance) -> Self {
        Self { first, last }
    }
    fn sum_invalid_ids(&self) -> u64 {
        let mut sum: u64 = 0;
        if self.first.valid == false {
            sum += self.first.number as u64;
        }
        if self.last.valid == false {
            sum += self.last.number as u64;
        }
        sum
    }
}

#[derive(Debug)]
struct ID_instance {
    number: u64,
    valid: bool,
}

impl ID_instance {
    // 1. The public constructor
    pub fn new(number: u64) -> Self {
        Self {
            number,
            // We call the private helper here
            valid: Self::validate(number),
        }
    }

    // 2. The private validation logic
    // This is not "pub", so only ID_instance can use it.
    // We call it 'validate' to be descriptive.
    fn validate(number: u64) -> bool {
        let digits: Vec<u32> = number
            .to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect();

        if digits.is_empty() {
            return false;
        }

        let half = digits.len() / 2;

        // If it meets the condition, it is NOT valid (return false)
        if digits.len() % 2 == 0 && digits[..half] == digits[half..] {
            false
        } else {
            true
        }
    }
}

fn main() {
    // Note: ensure this path exists or use a hardcoded string for testing
    // let input = include_str!("../../inputs/input_02/input.txt");
    let input = "11-22,95-115";
    println!("{}", part12(input));
}

fn part12(input: &str) -> String {
    // 1. Clean the input (remove newlines if any) and split by commas
    let ranges: Vec<ID_Range> = input
        .replace(['\n', '\r', ' '], "") 
        .split(',')
        .filter(|s| !s.is_empty()) 
        .map(|s| {
            // 2. Parse "start-end"
            let (start_str, end_str) = s.split_once('-').expect("Invalid range format");
            let start_num: u64 = start_str.parse().expect("Invalid start number");
            let end_num: u64 = end_str.parse().expect("Invalid end number");

            // 3. Create instances
            ID_Range::new(ID_instance::new(start_num), ID_instance::new(end_num))
        })
        .collect();

    // 4. Implement counting logic
    let sum_invalid_ids: u64 = ranges
        .iter()
        .map(|range| range.sum_invalid_ids())
        .sum(); // You can also let Rust infer the type if you type the variable
        
    format!("Part1 sum of Invalid IDs: {}", sum_invalid_ids)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        let example = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124";
        print!("{}",example);
        // This will print the output when you run: cargo test -- --nocapture
        let result = part12(example);
        println!("Result: {}", result);
    }
}
