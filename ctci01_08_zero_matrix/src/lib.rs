//! Implementation of the Zero Matrix algorithm from "Cracking the Coding Interview" (Chapter 1, Question 8)
//!
//! This module provides functionality to modify a 2D matrix such that if an element is 0,
//! its entire row and column are set to 0.

use std::collections::HashSet;

/// Transforms a matrix by zeroing out any row and column that contains a zero.
///
/// Given a matrix, this function identifies all elements with value zero,
/// then sets all elements in the corresponding rows and columns to zero.
///
/// # Arguments
///
/// * `matrix` - A mutable reference to a 2D vector of i32 values
///
/// # Examples
///
/// ```
/// use ctci01_08_zero_matrix::zero_matrix;
///
/// let mut matrix = vec![
///     vec![1, 2, 3],
///     vec![4, 0, 6], // Contains a zero at position (1,1)
///     vec![7, 8, 9]
/// ];
///
/// zero_matrix(&mut matrix);
///
/// // After zeroing, the second row and second column contain all zeros
/// assert_eq!(matrix, vec![
///     vec![1, 0, 3],
///     vec![0, 0, 0],
///     vec![7, 0, 9]
/// ]);
/// ```
///
/// # Notes
///
/// - The function handles matrices with inconsistent row lengths.
/// - For empty matrices or matrices with empty rows, no changes are made.
pub fn zero_matrix(matrix: &mut Vec<Vec<i32>>) {
    // Handle empty matrix case
    if matrix.is_empty() {
        return;
    }

    let mut zero_rows = HashSet::new();
    let mut zero_cols = HashSet::new();

    // First pass: identify rows and columns containing zeros
    for (i, row) in matrix.iter().enumerate() {
        for (j, val) in row.iter().enumerate() {
            if *val == 0 {
                zero_rows.insert(i);
                zero_cols.insert(j);
            }
        }
    }

    // Second pass: zero out identified rows and columns
    for i in &zero_rows {
        zero_out_row(matrix, *i);
    }
    
    for j in &zero_cols {
        zero_out_column(matrix, *j);
    }
}

/// Sets all elements in the specified row to zero.
///
/// # Arguments
///
/// * `matrix` - A mutable reference to a 2D vector of i32 values
/// * `row` - The index of the row to zero out
fn zero_out_row(matrix: &mut Vec<Vec<i32>>, row: usize) {
    if row < matrix.len() {
        for j in 0..matrix[row].len() {
            matrix[row][j] = 0;
        }
    }
}

/// Sets all elements in the specified column to zero.
///
/// # Arguments
///
/// * `matrix` - A mutable reference to a 2D vector of i32 values
/// * `col` - The index of the column to zero out
fn zero_out_column(matrix: &mut Vec<Vec<i32>>, col: usize) {
    for row in matrix.iter_mut() {
        if col < row.len() {
            row[col] = 0;
        }
    }
}

/// Utility function to print a 2D matrix for debugging.
///
/// # Arguments
///
/// * `matrix` - A reference to a 2D vector of i32 values
pub fn print_matrix(matrix: &Vec<Vec<i32>>) {
    for row in matrix {
        println!("{:?}", row);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test that an empty matrix is handled correctly
    #[test]
    fn test_empty_matrix() {
        let mut empty: Vec<Vec<i32>> = Vec::new();
        let expected: Vec<Vec<i32>> = Vec::new();
        zero_matrix(&mut empty);
        assert_eq!(empty, expected);
    }

    /// Test with a single element that is zero
    #[test]
    fn test_single_element_zero() {
        let mut matrix = vec![vec![0]];
        let expected = vec![vec![0]];
        zero_matrix(&mut matrix);
        assert_eq!(matrix, expected);
    }

    /// Test with a single element that is non-zero
    #[test]
    fn test_single_element_non_zero() {
        let mut matrix = vec![vec![5]];
        let expected = vec![vec![5]];
        zero_matrix(&mut matrix);
        assert_eq!(matrix, expected);
    }

    /// Test with a 3x3 matrix containing a single zero
    #[test]
    fn test_single_zero_in_regular_matrix() {
        let mut matrix = vec![
            vec![1, 2, 3],
            vec![4, 0, 6],
            vec![7, 8, 9]
        ];
        let expected = vec![
            vec![1, 0, 3],
            vec![0, 0, 0],
            vec![7, 0, 9]
        ];
        zero_matrix(&mut matrix);
        assert_eq!(matrix, expected);
    }

    /// Test with a 4x4 matrix containing multiple zeros
    #[test]
    fn test_multiple_zeros_in_matrix() {
        let mut matrix = vec![
            vec![1, 0, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 0, 12],
            vec![13, 14, 15, 16]
        ];
        let expected = vec![
            vec![0, 0, 0, 0],
            vec![5, 0, 0, 8],
            vec![0, 0, 0, 0],
            vec![13, 0, 0, 16]
        ];
        zero_matrix(&mut matrix);
        assert_eq!(matrix, expected);
    }

    /// Test with a matrix containing no zeros
    #[test]
    fn test_no_zeros_in_matrix() {
        let mut matrix = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9]
        ];
        let expected = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9]
        ];
        zero_matrix(&mut matrix);
        assert_eq!(matrix, expected);
    }

    /// Test with a matrix where an entire row is already zeros
    #[test]
    fn test_entire_row_already_zero() {
        let mut matrix = vec![
            vec![1, 2, 3],
            vec![0, 0, 0],
            vec![7, 8, 9]
        ];
        let expected = vec![
            vec![0, 0, 0],
            vec![0, 0, 0],
            vec![0, 0, 0]
        ];
        zero_matrix(&mut matrix);
        assert_eq!(matrix, expected);
    }

    /// Test with a matrix where an entire column is already zeros
    #[test]
    fn test_entire_column_already_zero() {
        let mut matrix = vec![
            vec![1, 0, 3],
            vec![4, 0, 6],
            vec![7, 0, 9]
        ];
        let expected = vec![
            vec![0, 0, 0],
            vec![0, 0, 0],
            vec![0, 0, 0]
        ];
        zero_matrix(&mut matrix);
        assert_eq!(matrix, expected);
    }

    /// Test with a matrix containing zeros in the same row
    #[test]
    fn test_zeros_in_same_row() {
        let mut matrix = vec![
            vec![1, 2, 3],
            vec![0, 5, 0],
            vec![7, 8, 9]
        ];
        let expected = vec![
            vec![0, 2, 0],
            vec![0, 0, 0],
            vec![0, 8, 0]
        ];
        zero_matrix(&mut matrix);
        assert_eq!(matrix, expected);
    }

    /// Test with a matrix containing zeros in the same column
    #[test]
    fn test_zeros_in_same_column() {
        let mut matrix = vec![
            vec![1, 2, 0],
            vec![4, 5, 6],
            vec![7, 8, 0]
        ];
        let expected = vec![
            vec![0, 0, 0],
            vec![4, 5, 0],
            vec![0, 0, 0]
        ];
        zero_matrix(&mut matrix);
        assert_eq!(matrix, expected);
    }

    /// Test with a matrix having rows of different lengths
    #[test]
    fn test_matrix_with_different_row_lengths() {
        let mut matrix = vec![
            vec![1, 2, 3, 4],
            vec![5, 0],
            vec![6, 7, 8]
        ];
        let expected = vec![
            vec![1, 0, 3, 4],
            vec![0, 0],
            vec![6, 0, 8]
        ];
        zero_matrix(&mut matrix);
        assert_eq!(matrix, expected);
    }
}

