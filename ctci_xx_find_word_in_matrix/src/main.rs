
type Word = Vec<char>;
type Vec2d = Vec<Word>;
type Pos = [usize; 2];
type VecPos = Vec<Pos>;
type VecVecPos = Vec<VecPos>;


fn compute_position_list(c : char, matrix : & Vec2d) -> VecPos {
    let mut result = VecPos::new();
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if matrix[i][j] == c {
                result.push([i, j]);
            }
        }
    }
    result
}

fn compute_all_position_lists( word : &Word, matrix : & Vec2d) -> VecVecPos {
    let mut result = VecVecPos::new();

    for i in 0..word.len() {
        let c = word[i];
        result.push(compute_position_list(c, matrix));
    }
    result
}

fn are_neighbors(pos1 : &Pos, pos2 : &Pos)  -> bool {
    // if one of them is undefined we return true
    if pos1[0] == std::usize::MAX || pos2[0] == std::usize::MAX {
        return true;
    }
    let d1 = pos1[0].abs_diff(pos2[0]);
    let d2 = pos1[1].abs_diff(pos2[1]);

    // For horizontal moves: row is the same (d1 == 0), column differs by 1 (d2 == 1)
    // For vertical moves: column is the same (d2 == 0), row differs by 1 (d1 == 1)
    (d1 == 0 && d2 == 1) || (d1 == 1 && d2 == 0)
}

fn search_word_in_matrix(word : &Word, matrix : &Vec2d) -> VecVecPos {
    
    let idx : usize = 0;
    let mut path = VecPos::new();
    let all_pos = compute_all_position_lists(&word, &matrix); 
    let mut result = VecVecPos::new();

    recursive_search(idx, &mut path, word, matrix, &all_pos, &mut result);

    result
}

fn recursive_search(idx : usize, path : &mut VecPos, word : &Word, matrix : & Vec2d, all_pos : &VecVecPos, result : &mut VecVecPos) {
    if idx >= word.len() {
        let p = path.clone();
        // print!("{:?}", &p);
        result.push(p);
        return;
    }
    for pos in all_pos[idx].iter() {
        let pos_last = match path.last() {
            Some(x) => x,
            None => &[std::usize::MAX, std::usize::MAX]
        };
        
        // Check if this position is a valid next position:
        // 1. It should not already be in the current path (avoid cycles)
        // 2. It should be a neighbor of the last position in the path
        if !path.contains(pos) && are_neighbors(pos, pos_last) {
            path.push(pos.clone());
            recursive_search(idx+1, path, word, matrix, all_pos, result);
            path.pop(); // Backtrack to explore other possibilities
        }
    }
}

fn main() {
    println!("Hello, world!");

    let matrix = 
        vec![ vec![ 'a', 'b', 'c'],
              vec![ 'c', 'd', 'e'],
              vec![ 'd', 'e', 'f'],
              vec![ 'g', 'h', 'i'],
        ];

    let word = vec!['d', 'e', 'f'];

    //let all_pos = compute_all_position_lists(&word, &matrix);
    //println!("All Positions: {:?}", all_pos);
    //let mut path = VecPos::new();
    //let mut result = VecVecPos::new();
    //recursive_search(0, &mut path, &word, &matrix, &all_pos, &mut result);

    let result = search_word_in_matrix(&word, &matrix);

    println!("Len Result: {}", result.len());
    for r in result {
        println!("{:?} ", r);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test helper function to create a standard test matrix
    fn create_test_matrix() -> Vec2d {
        vec![
            vec!['a', 'b', 'c', 'd'],
            vec!['e', 'f', 'g', 'h'],
            vec!['i', 'j', 'k', 'l'],
            vec!['m', 'n', 'o', 'p'],
        ]
    }

    /// Test finding a word horizontally in the matrix
    #[test]
    fn test_horizontal_word() {
        let matrix = create_test_matrix();
        let word = vec!['a', 'b', 'c'];
        
        let results = search_word_in_matrix(&word, &matrix);
        
        // Should find exactly one path
        assert_eq!(results.len(), 1);
        // Path should match the expected positions
        assert_eq!(results[0], vec![[0, 0], [0, 1], [0, 2]]);
    }

    /// Test finding a word vertically in the matrix
    #[test]
    fn test_vertical_word() {
        let matrix = create_test_matrix();
        let word = vec!['a', 'e', 'i', 'm'];
        
        let results = search_word_in_matrix(&word, &matrix);
        
        // Should find exactly one path
        assert_eq!(results.len(), 1);
        // Path should match the expected positions
        assert_eq!(results[0], vec![[0, 0], [1, 0], [2, 0], [3, 0]]);
    }
    /// Test when a word is not found in the matrix
    #[test]
    fn test_word_not_found() {
        let matrix = create_test_matrix();
        let word = vec!['x', 'y', 'z']; // These characters don't exist in the matrix
        
        let results = search_word_in_matrix(&word, &matrix);
        
        // Should find no paths
        assert_eq!(results.len(), 0);
    }

    /// Test with a single character word
    #[test]
    fn test_single_char_word() {
        let matrix = create_test_matrix();
        let word = vec!['a'];
        
        let results = search_word_in_matrix(&word, &matrix);
        
        // Should find exactly one path
        assert_eq!(results.len(), 1);
        // Path should match the expected position
        assert_eq!(results[0], vec![[0, 0]]);
    }

    /// Test with an empty word
    #[test]
    fn test_empty_word() {
        let matrix = create_test_matrix();
        let word = vec![];
        
        let results = search_word_in_matrix(&word, &matrix);
        
        // Should find exactly one path (an empty path)
        assert_eq!(results.len(), 1);
        // The path should be empty
        assert_eq!(results[0], Vec::<Pos>::new());
    }

    /// Test with an empty matrix
    #[test]
    fn test_empty_matrix() {
        let matrix: Vec2d = vec![];
        let word = vec!['a', 'b', 'c'];
        
        let results = search_word_in_matrix(&word, &matrix);
        
        // Should find no paths
        assert_eq!(results.len(), 0);
    }

    /// Test multiple possible paths for the same word
    #[test]
    fn test_multiple_paths() {
        let matrix = vec![
            vec!['a', 'b', 'c'],
            vec!['d', 'e', 'f'],
            vec!['g', 'e', 'f'],
            vec!['j', 'k', 'l'],
        ];
        
        // The word "ef" can be found in both horizontal paths (row 1 and row 2)
        let word = vec!['e', 'f'];
        
        let results = search_word_in_matrix(&word, &matrix);
        
        // Should find exactly two paths
        assert_eq!(results.len(), 2);
        
        // Check that both expected paths are in the results
        let path1 = vec![[1, 1], [1, 2]]; // Second row
        let path2 = vec![[2, 1], [2, 2]]; // Third row
        
        let contains_path1 = results.iter().any(|path| *path == path1);
        let contains_path2 = results.iter().any(|path| *path == path2);
        
        assert!(contains_path1, "Results should contain the first horizontal path");
        assert!(contains_path2, "Results should contain the second horizontal path");
    }

    /// Test a word that wraps around the matrix (which should not be valid)
    #[test]
    fn test_word_wrapping() {
        let matrix = create_test_matrix();
        let word = vec!['d', 'a']; // 'd' at [0,3] and 'a' at [0,0] are not neighbors
        
        let results = search_word_in_matrix(&word, &matrix);
        
        // Should find no valid paths as wrapping is not allowed
        assert_eq!(results.len(), 0);
    }
}
