use std::arch::x86_64;

  #[derive(Copy, Clone, Debug)]
    struct Ingredient {
        number: i64,
        state: i8,
    }

    #[derive(Debug, Clone, Copy)]
    struct Range {
        start: i64,
        end: i64,
    }
    impl Range {
        fn contains(&self, ingredient: Ingredient) -> bool {
            ingredient.number >= self.start && ingredient.number <= self.end
        }
    }

fn parse_file(input: &str) -> (Vec<Ingredient>, Vec<Range>) {
        let (first, second): (&str, &str) = input.split_once("\n\n").unwrap();
        let ing_list: Vec<i64> = second
            .lines()
            .map(|line| line.parse::<i64>().unwrap())
            .collect();
        let mut ranges: Vec<Range> = first
            .lines()
            .map(|line| {
                let (start, end) = line.split_once("-").unwrap();
                let start_num = start.parse::<i64>().unwrap();
                let end_num = end.parse::<i64>().unwrap();
                Range {
                    start: start_num,
                    end: end_num,
                }
            })
            .collect();
        ranges.sort_by(|a, b| a.start.cmp(&b.start));
        let ingredients: Vec<Ingredient> = ing_list
            .iter()
            .map(|&num| Ingredient {
                number: num,
                state: 0,
            })
            .collect();
        (ingredients, ranges)
    }
    
fn main() {
    let input = include_str!("../../inputs/input_05/input.txt");
//     let input = "3-5
// 10-14
// 16-20
// 12-18

// 1
// 5
// 8
// 11
// 17
// 32";

    let (mut ingredients, ranges) : (Vec<Ingredient>, Vec<Range>) = parse_file(input);
    ingredients.iter_mut().for_each(|ingredient| {
        for range in &ranges {
            if range.contains(*ingredient) {
                ingredient.state = 1;
                break;
            }
        }
    });
    println!("there are {:?} ingredients that are fresh!", (ingredients.iter().filter(|ing| ing.state >= 1).count()));
    let merged_ranges = merge_sorted_ranges(&ranges);
    //println!("sorted and merged ranges: {:?}", merged_ranges);

    merged_ranges.iter().map(|x| x.end - x.start + 1).sum::<i64>();
    let total_covered: i64 = merged_ranges.iter().map(|x| x.end - x.start + 1).sum();
    println!("total covered: {:?}", total_covered);
}


fn merge_sorted_ranges(ranges: &Vec<Range>) -> Vec<Range> {
    if ranges.is_empty() {
        return vec![];
    }
    let mut merged: Vec<Range> = Vec::new();
    let mut current_range = ranges[0].clone();
    for range in ranges.iter().skip(1) {
        if range.start <= current_range.end {
            //println!("merged stuff: {:?} {:?}", current_range, range);
            current_range.end = current_range.end.max(range.end);
        } else {
            merged.push(current_range);
            current_range = range.clone();
        }
    }
    merged.push(current_range);
    merged
}