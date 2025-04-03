
use std::{collections::HashSet, env};

fn is_unique_brute(s : &str) -> bool {
    for i in 0..s.len() {
        for j in i+1..s.len() {
            if s.as_bytes()[i] == s.as_bytes()[j] {
                return false;
            }
        }
    }
    true
}

fn is_unique_set(s : &str) -> bool {
    let mut characters = HashSet::new();
    for c in s.chars() {
        if characters.contains(&c) {
            return false;
        } else {
            characters.insert(c);
        }
    }
    true
}

fn is_unique_sort(s : &mut str) -> bool {
    if s.len() == 0 {
        return true;
    }

    let mut sorted_s : Vec<char> = s.chars().collect();
    sorted_s.sort_unstable();

    let last_char = sorted_s[0];
    for c in &sorted_s[1..] {
        if *c == last_char {
            return false;
        }
    }
    true
}

fn main() {
    let mut args : Vec<String> = env::args().collect();
    for i in 1..args.len() {
        let is_unique0 = is_unique_brute(&args[i]);
        let is_unique1 = is_unique_set(&args[i]);
        let is_unique2 = is_unique_sort(&mut args[i]);
        println!("{}. Input string \"{}\" \t is_unique (brute): {} \t is_unique (use set): {} \t is_unique (sort): {}", 
            i, args[i], is_unique0, is_unique1, is_unique2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_unique() {
        let mut args  = vec![("abcxyz".to_string(), true), ("axayat".to_string(), false)];

        for i in 0..args.len() {
            let is_unique0 = is_unique_brute(&args[i].0);
            let is_unique1 = is_unique_set(&args[i].0);
            let is_unique2 = is_unique_sort(&mut args[i].0);
            assert!(is_unique0 == args[i].1 && is_unique1 == args[i].1 && is_unique2 == args[i].1);
        }
    }
}