use itertools::Itertools;

#[derive(Debug)]
struct IDRange {
    first: IDInstance,
    last: IDInstance,
}

impl IDRange {
    pub fn new(first: IDInstance, last: IDInstance) -> Self {
        Self { first, last }
    }
    pub fn sum_invalid_ids(&self) -> u64 {
        let range_vector: Vec<u64> = (self.first.number..=self.last.number).collect();
        range_vector
            .into_iter()
            .filter(|&num| num.to_string().len() % 2 == 0)
            .map(|num| IDInstance::new(num))
            .filter(|id| !id.valid)
            .map(|id| id.number)
            .sum()
    }
    pub fn sum_part2_criteria(&self) -> u64 {
        (self.first.number..=self.last.number)
            .filter(|&n| {
                let s = n.to_string();
                let len = s.len() as u64;
                let divisors = get_divisors(len);

                // We keep this number if ANY divisor results in equal chunks
                divisors.iter().any(|&d| {
                    let chunks = split_equally(d, &s);
                    equal_check_shortcirc(&chunks)
                })
            })
            .sum()
    }
}

#[derive(Debug)]
struct IDInstance {
    number: u64,
    valid: bool,
}

impl IDInstance {
    pub fn new(number: u64) -> Self {
        Self {
            number,
            valid: Self::validate(number),
        }
    }

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

        if digits.len() % 2 == 0 && digits[..half] == digits[half..] {
            false
        } else {
            true
        }
    }
}

fn get_divisors(n: u64) -> Vec<u64> {
    let mut divisors = Vec::new();
    for i in 1..=n/2 {
        if n % i == 0 {
            divisors.push(i);
        }
    }
    divisors
}

fn split_equally(chunk_size: u64, s: &str) -> Vec<String> {
    let mut chunks = Vec::new();
    if chunk_size == 0 {
        return chunks;
    }
    for chunk in s.as_bytes().chunks(chunk_size as usize) {
        let str_chunk = std::str::from_utf8(chunk).unwrap();
        chunks.push(str_chunk.to_string());
    }
    chunks
}

fn equal_check_shortcirc(a: &Vec<String>) -> bool {
    if a.is_empty() {
        return false;
    }
    let first = &a[0];
    a.iter().all(|item| item == first)
}

fn main() {
    // Note: ensure this path exists or use a hardcoded string for testing
    let input = include_str!("../../inputs/input_02/input.txt");
    
    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn part1(input: &str) -> String {
    let ranges: Vec<IDRange> = parse_ranges(input);

    let sum_invalid_ids: u64 = ranges
        .iter()
        .map(|range| range.sum_invalid_ids())
        .sum();
        
    format!("Part1 sum of Invalid IDs: {}", sum_invalid_ids)
}

fn part2(input: &str) -> String {
    let ranges: Vec<IDRange> = parse_ranges(input);

    let sum_equal_splits: u64 = ranges
        .iter()
        .map(|range| range.sum_part2_criteria())
        .sum();

    format!("Part2 sum of Equal Split IDs: {}", sum_equal_splits)
}

// Extracted parsing logic since both parts use it
fn parse_ranges(input: &str) -> Vec<IDRange> {
    input
        .replace(['\n', '\r', ' '], "")
        .split(',')
        .filter(|s| !s.is_empty())
        .map(|s| {
            let (start_str, end_str) = s.split_once('-').expect("Invalid range format");
            let start_num: u64 = start_str.parse().expect("Invalid start number");
            let end_num: u64 = end_str.parse().expect("Invalid end number");
            IDRange::new(IDInstance::new(start_num), IDInstance::new(end_num))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        let example = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124";
        
        let result = part1(example);
        println!("Result Part 1: {}", result);
    }

    #[test]
    fn test_part2_example() {
        // I added '121212' which splits into 12,12,12 (sum should increase by 121212)
        let example = "121210-121215"; 
        let result = part2(example);
        println!("Result Part 2: {}", result);
        assert!(result.contains("121212"));
    }
}