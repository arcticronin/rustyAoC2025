use std::arch::x86_64;

use rayon::vec;

  #[derive(Clone, Debug)]
    struct MathProblem{
        numbers: Vec<i64>,
        operator: char,
    }
    impl MathProblem {
        fn new(numbers: Vec<i64>, operator: char) -> Self {
            MathProblem { numbers, operator }
        }

        fn evaluate(&self) -> i64 {
            match self.operator {
                '+' => self.numbers.iter().sum(),
                '*' => self.numbers.iter().product(),
                _ => panic!("Unsupported operator"),
            }
        }
    }
    
fn main() {
    //let input = include_str!("../../inputs/input_06/input.txt");
    let input = include_str!("../../inputs/input_06/test.txt");
    let math_problems = parse_part1(input);
    for problem in math_problems.iter() {
        println!("math problem: {:?} = {}", problem, problem.evaluate());
    }
    let total: i64 = math_problems.iter().map(|p| p.evaluate()).sum();
    println!("Part1: {}", total);


    let math_problems2 = parse_part2(input);
    println!("=========================");
    
                                            
    println!("Part2: {:?}", math_problems2);
    let total: i64 = math_problems2.iter().map(|p| p.evaluate()).sum();
    println!("Part2: {}", total);
}

fn parse_part1(input: &str) -> Vec<MathProblem> {
    let mut lines: Vec<&str> = input.lines().collect();
    let op_line = lines.pop().unwrap(); 
    let operators: Vec<char> = op_line
        .split_whitespace()
        .map(|s| s.chars().next().unwrap()) 
        .collect();
    let grid: Vec<Vec<u32>> = lines
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect()
        })
        .collect();
    let width = grid[0].len();  // it is 4 but i still make it dynamic
    let mut columns: Vec<Vec<u32>> = Vec::new();
    for i in 0..width {
        let col: Vec<u32> = grid.iter().map(|row| row[i]).collect();
        columns.push(col);
    }
    columns.into_iter().zip(operators)
        .map(|(numbers, operator)| MathProblem::new(numbers.into_iter().map(|n| n as i64).collect(), operator))
        .collect()
}


fn parse_part2(input: &str) -> Vec<MathProblem> {
    let mut lines: Vec<&str> = input.lines().collect();
    if lines.is_empty() { return Vec::new(); }
    let _op_line = lines.pop(); 
    let maxlen = lines.iter().map(|l| l.len()).max().unwrap_or(0);
    let grid: Vec<Vec<u32>> = lines
        .iter()
        .map(|line| {
            let mut row_vec: Vec<u32> = line.chars()
                .map(|c| {
                    if c == ' ' {
                        0 
                    } else {
                        c.to_digit(10).unwrap_or(0) 
                    }
                })
                .collect();
            while row_vec.len() < maxlen {
                row_vec.push(0);
            }
            row_vec
        })
        .collect();
    let split_numbers: Vec<Vec<u32>> = split_by_empty_columns(&grid)
                                                                        .iter()
                                                                        .map(|x| adjust_decimals(x.clone()))
                                                                        .collect();
    let mut lines: Vec<&str> = input.lines().collect();
    let op_line = lines.pop().unwrap(); 
    let operators: Vec<char> = op_line
        .split_whitespace()
        .map(|s| s.chars().next().unwrap()) 
        .collect();
    split_numbers.into_iter().zip(operators)
        .map(|(numbers, operator)| MathProblem::new(numbers.into_iter().map(|n| n as i64).collect(), operator))
        .collect()
}

fn split_by_empty_columns(grid: &Vec<Vec<u32>>) -> Vec<Vec<Vec<u32>>> {
    if grid.is_empty() { return vec![]; }
    let height = grid.len();
    let width = grid[0].len();
    let mut results = Vec::new();
    let mut start_col = 0;
    for col in 0..width {
        let is_empty_col = (0..height).all(|row| grid[row][col] == 0);
        if is_empty_col {
            if col > start_col {
                let chunk = extract_chunk(grid, start_col, col);
                results.push(chunk);
            }
            start_col = col + 1;
        }
    }
    if start_col < width {
        results.push(extract_chunk(grid, start_col, width));
    }

    results
}

fn extract_chunk(grid: &Vec<Vec<u32>>, start: usize, end: usize) -> Vec<Vec<u32>> {
    grid.iter()
        .map(|row| row[start..end].to_vec())
        .collect()
}

fn adjust_decimals(matrix: Vec<Vec<u32>>) -> Vec<u32> {
    if matrix.is_empty() { return vec![]; }
    let rows = matrix.len();
    let cols = matrix[0].len();
    let matrix_nd = ndarray::Array2::from_shape_vec(
        (rows, cols), 
        matrix.into_iter().flatten().collect()
    ).unwrap();
    let v_pow = ndarray::Array1::from_shape_fn(cols, |i| 10u32.pow((cols - 1 - i) as u32));
    let result_array = matrix_nd.dot(&v_pow);
    result_array.to_vec()
}