/// Example implementation of the Zero Matrix algorithm
use ctci01_08_zero_matrix::{zero_matrix, print_matrix};

// Constants for matrix dimensions
const ROWS: usize = 6;
const COLS: usize = 4;
const DEFAULT_VALUE: i32 = 1;

fn main() {
    // Create a matrix with all 1's
    let mut example_matrix = vec![vec![DEFAULT_VALUE; COLS]; ROWS];
    
    // Place zeros at specific positions to demonstrate the algorithm
    example_matrix[2][2] = 0; // Zero in the middle
    example_matrix[0][2] = 0; // Zero in the first row
    
    println!("Original Matrix:");
    println!("--------------");
    print_matrix(&example_matrix);
    
    // Apply the zero matrix algorithm
    zero_matrix(&mut example_matrix);
    
    println!("\nMatrix after zeroing rows and columns with zeros:");
    println!("-----------------------------------------");
    print_matrix(&example_matrix);
    
    // Explain what happened
    println!("\nNote: All rows and columns that contained a zero");
    println!("have been set entirely to zero.");
}
