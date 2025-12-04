use itertools::Itertools;

#[derive(Debug)]
struct Safe {
    position: i32,
    maxposition: i32,
    zeros: u32,
}

impl Safe {
    fn new(position: i32, maxposition: i32) -> Safe {
        Safe {
            position,
            maxposition,
            zeros: 0,
        }
    }

   fn apply_instruction(&mut self, instruction: &Instruction) {
        let delta = match instruction.direction {
            'L' => -instruction.clicks,
            'R' => instruction.clicks,
            _ => panic!("Invalid direction"),
        };
        let new_raw_position = self.position + delta;
        let next_position = new_raw_position.rem_euclid(self.maxposition);
        let current_cycles = self.position.div_euclid(self.maxposition); // This is usually 0 if position is normalized
        let next_cycles = new_raw_position.div_euclid(self.maxposition);
        let mut passes = (next_cycles - current_cycles).abs();
        if delta > 0 {
            let count = (new_raw_position / self.maxposition) - (self.position / self.maxposition);
            self.zeros += count as u32;
        } else {
            let start_idx = self.position - 1;
            let end_idx = new_raw_position - 1;
            let count = start_idx.div_euclid(self.maxposition) - end_idx.div_euclid(self.maxposition);
            self.zeros += count as u32;
        }
        self.position = next_position;
    }
}

#[derive(Debug)]
struct Instruction {
    direction: char,
    clicks: i32,
}

impl Instruction {
    fn from_str(s: &str) -> Instruction {
        let direction = s.chars().next().unwrap();
        let clicks: i32 = s[1..].trim().parse().expect("Failed to parse number");
        Instruction { direction, clicks }
    }
}

fn main() {
    let input = include_str!("../../inputs/input_01/input.txt");
    println!("Part 1,2: {}", part12(input));
}

fn part12(input: &str) -> String {
    let initial_safe = Safe::new(50, 100);
    let (final_safe, landed_count) = input
    .lines()
    .fold((initial_safe, 0), 
    |(mut safe, count), line| {
        let instruction = Instruction::from_str(line.trim());
        safe.apply_instruction(&instruction);
        let new_count = if safe.position == 0 { count + 1 } else { count };
        (safe, new_count)
    });

    format!("Part1: {}, Part2: {}", landed_count, final_safe.zeros)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        let example = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
        assert_eq!(part12(example), "3");
    }
}
