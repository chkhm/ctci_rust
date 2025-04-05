/// Checks if s2 is a rotation of s1 using only one call to a substring function.
///
/// # Examples
///
/// ```
/// let s1 = "waterbottle";
/// let s2 = "erbottlewat";
/// assert!(is_string_rotation(s1, s2));
/// ```
///
/// # Algorithm
///
/// The key insight is that if s2 is a rotation of s1, then s2 will be a substring
/// of s1 concatenated with itself (s1 + s1). This allows us to use just one
/// call to the contains method.
fn is_string_rotation(s1: &str, s2: &str) -> bool {
    // If strings are not the same length, one cannot be a rotation of the other
    if s1.len() != s2.len() {
        return false;
    }
    
    // Empty strings are considered rotations of each other
    if s1.is_empty() {
        return true;
    }
    
    // Check if s2 is a substring of s1+s1
    let s1_doubled = format!("{}{}", s1, s1);
    s1_doubled.contains(s2)
}

fn main() {
    let s1 = "waterbottle";
    let s2 = "erbottlewat";
    
    if is_string_rotation(s1, s2) {
        println!("'{}' is a rotation of '{}'", s2, s1);
    } else {
        println!("'{}' is NOT a rotation of '{}'", s2, s1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_valid_rotations() {
        assert!(is_string_rotation("waterbottle", "erbottlewat"));
        assert!(is_string_rotation("abcde", "cdeab"));
        assert!(is_string_rotation("hello", "llohe"));
        assert!(is_string_rotation("a", "a"));
        assert!(is_string_rotation("", ""));
    }
    
    #[test]
    fn test_invalid_rotations() {
        assert!(!is_string_rotation("waterbottle", "waterbottla"));
        assert!(!is_string_rotation("abcde", "abcdf"));
        assert!(!is_string_rotation("hello", "world"));
        assert!(!is_string_rotation("a", "b"));
        assert!(!is_string_rotation("", "a"));
        assert!(!is_string_rotation("abc", "abcd"));
    }
    
    #[test]
    fn test_edge_cases() {
        // Test with empty strings
        assert!(is_string_rotation("", ""));
        
        // Test with single character strings
        assert!(is_string_rotation("a", "a"));
        assert!(!is_string_rotation("a", "b"));
        
        // Test with spaces and special characters
        assert!(is_string_rotation("a b c ", " a b c"));
        assert!(is_string_rotation("a b c ", "c a b "));
        assert!(is_string_rotation("a!@#", "@#a!"));
    }
}
