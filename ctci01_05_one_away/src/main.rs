
use std::env;

fn one_away(s1 : &str, s2 : &str) -> bool {
    
    let l1 = s1.len();
    let l2 = s2.len();
    let d = usize::abs_diff(l1, l2);
    if d > 1 {
        return false;
    }
    if l1 == l2 {
        return replacement_iter(s1, s2);
    }
    if l1 > l2 {
        return remove_iter(s1, s2);
    }
    else {
        return remove_iter(s2, s1);
    }
}

// we know s1 and s2 are of the same length
fn replacement_iter(s1 : &str, s2 : &str) -> bool {
    // first we clone the underlying string so that we can edit
    let a1 = s1.to_owned();
    let a2 = s2.to_owned();

    let mut edit_done = false;
    // crazy iterator shit for rust... not sure if this language is really as cool as they say
    // it makes you write a lot of ugly code
    let it1 = a1.chars().into_iter();
    let it2 = a2.chars().into_iter();

    for mut it in it1.zip(it2) {
        if it.0 != it.1 && edit_done {
            return false;
        }
        if it.0 != it.1 {
            it.0 = it.1;
            edit_done = true;
        }
    }
    true
}

// we can safely assume that s1 is exactly one char longer than s2
fn remove_iter(s1 : &str, s2 : &str) -> bool {
    // first we clone the underlying string so that we can edit
    let a1 = s1.to_owned();
    let a2 = s2.to_owned();

    let mut edit_done = false;
    // crazy iterator shit for rust... not sure if this language is really as cool as they say
    // it makes you write a lot of ugly code
    let mut it1 = a1.chars().into_iter();
    let mut it2 = a2.chars().into_iter();

    let mut c1 = it1.next();
    let mut c2 = it2.next();
    while c2 != None {
        if c1.unwrap() != c2.unwrap() && edit_done {
            return false;
        }
        if c1.unwrap() != c2.unwrap() {
            c1 = it1.next();
            edit_done = true;
        }
        else {
            c1 = it1.next();
            c2 = it2.next();
        }
    }
    true
}

fn main() {
    let args : Vec<String> = env::args().collect();

    let mut s1 = "abcdefg";
    let mut s2 = "axxdefg";
    //let mut s1 = "";
    //let mut s2 = "";

    if args.len() > 2 {
        s2 = &args[2];
        s1 = &args[1];
    }
    else if args.len() > 1 {
        s2 = &args[1];
    }

    let result = one_away(s1, s2);
    println!("Distance of \"{}\" and \"{}\" is a one_away: {}", s1, s2, result);
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
            ("abcde", "abc", false)
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