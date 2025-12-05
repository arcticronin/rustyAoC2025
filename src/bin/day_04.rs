fn main() {
    // Note: ensure this path exists or use a hardcoded string for testing
    let input = include_str!("../../inputs/input_04/input.txt");
//     let input = "..@@.@@@@.
// @@@.@.@.@@
// @@@@@.@.@@
// @.@@@@..@.
// @@.@@@@.@@
// .@@@@@@@.@
// .@.@.@.@@@
// @.@@@.@@@@
// .@@@@@@@@.
// @.@.@@@.@.";

    let matrix = parse_file(input);
    
    // We work directly on the padded matrix for the simulation
    let mut padded = pad_matrix(&matrix);
    
    let rows = matrix.len();
    let cols = matrix[0].len(); // Assuming rectangular matrix

    let mut iteration_count = 1;
    let mut total_removed = 0;

    loop {
        // Vector to store the coordinates (row, col) of cells to remove this round
        let mut to_remove: Vec<(usize, usize)> = Vec::new();

        // 1. Scan phase: Find candidates for removal
        // Iterate through the valid "inner" area (indices 1 to rows/cols)
        for r in 1..=rows {
            for c in 1..=cols {
                // We only check cells that are currently alive (1)
                if padded[r][c] == 1 {
                    
                    // Sum neighbors
                    let neighbor_sum = 
                        padded[r-1][c-1] + padded[r-1][c] + padded[r-1][c+1] +
                        padded[r][c-1]   + /* center */     padded[r][c+1] +
                        padded[r+1][c-1] + padded[r+1][c] + padded[r+1][c+1];

                    // If less than 4 neighbors, mark for removal
                    if neighbor_sum < 4 {
                        to_remove.push((r, c));
                    }
                }
            }
        }

        // 2. Check termination condition
        if to_remove.is_empty() {
            break;
        }

        // 3. Update phase: Apply removals
        let count_this_round = to_remove.len();
        total_removed += count_this_round;

        for (r, c) in to_remove {
            padded[r][c] = 0;
        }

        println!("Iteration {}: removed {} items", iteration_count, count_this_round);
        iteration_count += 1;
    }

    println!("--------------------------------");
    println!("Simulation finished.");
    println!("Total items removed: {}", total_removed);
}

// Adds a border of 0s around the matrix
fn pad_matrix(matrix: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let rows = matrix.len();
    let cols = matrix[0].len(); // Assuming non-empty matrix
    
    // Create new matrix with size +2 on both dimensions, filled with 0
    let mut padded = vec![vec![0u8; cols + 2]; rows + 2];

    // Copy original data into the center
    for r in 0..rows {
        for c in 0..cols {
            padded[r + 1][c + 1] = matrix[r][c];
        }
    }
    
    padded
}

fn parse_file(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '@' => 1,
                    '.' => 0,
                    _ => panic!("Unexpected character: {}", c),
                })
                .collect()
        })
        .collect()
}