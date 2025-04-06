
#![feature(linked_list_cursors)]

use std::collections::{HashSet, LinkedList};
use std::env;

fn remove_duplicates_using_hashset(list : &mut LinkedList<i32>) {
    let mut unique_numbers = HashSet::new();


    let mut cursor = list.cursor_front_mut();

    while let Some(val) = cursor.current() {
        if unique_numbers.contains(val) {
            cursor.remove_current();
        } else {
            unique_numbers.insert(*val);
            cursor.move_next();
        }
    }
}

fn get_values_from_args() -> LinkedList<i32> {
    if env::args().len() > 1 {
        let args: Vec<String> = env::args().skip(1).collect();
        let arg_values_result : Result<Vec<i32>, _> = args.iter().map(|s| s.parse::<i32>()).collect();
        let arg_values = match arg_values_result {
            Ok(arg_values) => arg_values,
            Err(_) => [0].to_vec(),
        };
        let mut list = LinkedList::new();
        arg_values.iter().for_each(|ele| list.push_back(*ele));
        return list;
    } else {
        let default_values = [1, 2, 3, 3, 5, 4, 1, 2, 5];
        let list = LinkedList::from(default_values);
        return list;
    }
}

fn main() {
    let mut list = get_values_from_args();
    println!("List: {:?}", list);
    remove_duplicates_using_hashset(&mut list);
    println!("List: {:?}", list);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_list() {
        let mut list = LinkedList::new();
        remove_duplicates_using_hashset(&mut list);
        assert_eq!(list, LinkedList::new());
    }

    #[test]
    fn test_no_duplicates() {
        let mut list = LinkedList::from([1, 2, 3, 4, 5]);
        let expected = LinkedList::from([1, 2, 3, 4, 5]);
        remove_duplicates_using_hashset(&mut list);
        assert_eq!(list, expected);
    }

    #[test]
    fn test_consecutive_duplicates() {
        let mut list = LinkedList::from([1, 1, 2, 2, 3, 3]);
        let expected = LinkedList::from([1, 2, 3]);
        remove_duplicates_using_hashset(&mut list);
        assert_eq!(list, expected);
    }

    #[test]
    fn test_non_consecutive_duplicates() {
        let mut list = LinkedList::from([1, 2, 3, 1, 2, 3]);
        let expected = LinkedList::from([1, 2, 3]);
        remove_duplicates_using_hashset(&mut list);
        assert_eq!(list, expected);
    }

    #[test]
    fn test_all_same_elements() {
        let mut list = LinkedList::from([5, 5, 5, 5, 5]);
        let expected = LinkedList::from([5]);
        remove_duplicates_using_hashset(&mut list);
        assert_eq!(list, expected);
    }

    #[test]
    fn test_multiple_duplicates_of_multiple_values() {
        let mut list = LinkedList::from([1, 2, 2, 3, 1, 4, 4, 5, 3, 5]);
        let expected = LinkedList::from([1, 2, 3, 4, 5]);
        remove_duplicates_using_hashset(&mut list);
        assert_eq!(list, expected);
    }
}
