use itertools::Itertools;

fn main() {
    // Note: ensure this path exists or use a hardcoded string for testing
    let input = include_str!("../../inputs/input_03/input.txt");
//     let input  = "987654321111111
// 811111111111119
// 234234234234278
// 818181911112111";
    
    println!("{:?}", parse_file(input));
}

fn parse_file(input: &str) -> i64 {
    input
        .lines()
        .filter(|s| !s.is_empty())
        .map(|s| {
            let digits: Vec<u32> = s.chars()
                .filter_map(|c| c.to_digit(10))
                .collect();
            if digits.len() < 2 {
                return digits.get(0).cloned().unwrap_or(0) as i64; 
            }
            let (tens_idx, &tens_val) = digits[..digits.len() - 1]
                .iter()
                .enumerate()
                .fold((0, &0), |(best_i, best_val), (curr_i, curr_val)| {
                    if curr_val > best_val {
                        (curr_i, curr_val)
                    } else {
                        (best_i, best_val)
                    }
                });
            let &units_val = digits[tens_idx + 1..]
                .iter()
                .max()
                .unwrap();
            (tens_val as i64 * 10) + units_val as i64
        })
        .sum::<i64>()
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1_example() {
        let example = "987654321111111
811111111111119
234234234234278
818181911112111";
        
        let result = parse_file(example);
        println!("Result Part 1: {:?}", result);
    }
}