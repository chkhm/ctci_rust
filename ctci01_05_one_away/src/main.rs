use std::env;

/// Determines if two strings are one edit away from each other.
/// 
/// An edit can be one of three operations:
/// - Insert a character
/// - Remove a character
/// - Replace a character
/// 
/// # Arguments
/// * `s1` - First string to compare
/// * `s2` - Second string to compare
/// 
/// # Returns
/// `true` if strings are one or zero edits away from each other, `false` otherwise
fn one_away(s1: &str, s2: &str) -> bool {
    // Check if length difference is more than 1
    let l1 = s1.len();
    let l2 = s2.len();
    let d = usize::abs_diff(l1, l2);
    if d > 1 {
        return false;
    }
    
    // Equal length strings - need to check for replacements
    if l1 == l2 {
        return replacement_check(s1, s2);
    }
    
    // Different length strings - need to check for insertions/deletions
    if l1 > l2 {
        return insertion_check(s1, s2);
    } else {
        return insertion_check(s2, s1);
    }
}

/// Checks if two strings of the same length are one character replacement away.
/// 
/// # Arguments
/// * `s1` - First string to compare
/// * `s2` - Second string to compare
/// 
/// # Returns
/// `true` if strings are at most one replacement away, `false` otherwise
fn replacement_check(s1: &str, s2: &str) -> bool {
    let mut differences = 0;
    
    // Zip iterators to compare characters at the same positions
    for (c1, c2) in s1.chars().zip(s2.chars()) {
        if c1 != c2 {
            differences += 1;
            
            // More than one difference means more than one edit
            if differences > 1 {
                return false;
            }
        }
    }
    
    true
}

/// Checks if two strings are one character insertion/deletion away.
/// 
/// This function assumes that `s1` is exactly one character longer than `s2`.
/// 
/// # Arguments
/// * `s1` - Longer string (exactly one character longer than s2)
/// * `s2` - Shorter string
/// 
/// # Returns
/// `true` if removing one character from s1 can make it equal to s2, `false` otherwise
fn insertion_check(s1: &str, s2: &str) -> bool {
    let mut it1 = s1.chars().peekable();
    let mut it2 = s2.chars().peekable();
    let mut skipped = false;
    
    // Compare characters until we reach the end of the shorter string
    while let Some(&c2) = it2.peek() {
        match it1.peek() {
            Some(&c1) if c1 == c2 => {
                // Characters match, advance both iterators
                it1.next();
                it2.next();
            },
            Some(_) => {
                // Characters don't match
                if skipped {
                    // Already skipped one character, this would be a second mismatch
                    return false;
                }
                
                // Skip the extra character in the longer string
                it1.next();
                skipped = true;
            },
            None => {
                // Reached the end of s1 before s2, which shouldn't happen
                return false;
            }
        }
    }
    
    // If we've made it through the entire shorter string, we're good
    true
}

/// Main function to run the One Away algorithm with command-line arguments.
///
/// Usage:
///   cargo run [string1 string2 string3 string4 ...]
///
/// Strings are processed in pairs. For example:
///   - string1 is compared with string2
///   - string3 is compared with string4
///   - ...
///
/// If an odd number of arguments is provided, the last string is compared 
/// with a default value. If no arguments are provided, default strings are used.
fn main() {
    // Skip program name and collect remaining args
    let args: Vec<String> = env::args().skip(1).collect();
    
    // Default values
    let default_s1 = "abcdefg";
    let default_s2 = "axxdefg";
    
    // Check if no arguments provided
    if args.is_empty() {
        let result = one_away(default_s1, default_s2);
        println!("Are \"{}\" and \"{}\" one edit away from each other? {}", 
                 default_s1, default_s2, result);
        return;
    }
    
    // Process arguments in pairs
    let mut pair_count = 0;
    
    for i in (0..args.len()).step_by(2) {
        pair_count += 1;
        let s1 = &args[i];
        
        // If this is the last argument and we have an odd number of args
        let s2 = if i + 1 < args.len() {
            &args[i + 1]
        } else {
            default_s2  // Use default for odd argument
        };
        
        let result = one_away(s1, s2);
        
        // Print result with formatting
        if args.len() > 2 {
            println!("\nPair {}: ", pair_count);
        }
        println!("Are \"{}\" and \"{}\" one edit away from each other? {}", 
                 s1, s2, result);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_simple() {
        let test_data = vec![
            ("abcdefg", "axxdefg", false), 
            ("", "", true), 
            ("abc", "abc", true), 
            ("abcd", "abc", true),
            ("axbc", "abc", true),
            ("axc", "abc", true),
            ("abcde", "abc", false),
            ("", "a", true)
        ];

        for x in test_data {
            let s1 = x.0;
            let s2 = x.1;
            let expected_result = x.2;
            let result = one_away(s1, s2);
            assert!(result == expected_result);
        }
    }
}