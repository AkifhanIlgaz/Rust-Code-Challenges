/*
    Find Unique Items

    Write a function that filters duplicates within a Vec<i32>
*/

// fn unique(mut a: Vec<i32>) -> Vec<i32> {
//     a.sort();
//     a.dedup();
//     a
// }

// advanced 1: use generic types
// fn unique<T>(mut a: Vec<T>) -> Vec<T>
// where
//     T: Ord,
// {
//     a.sort();
//     a.dedup();
//     a
// }

use std::{collections::HashSet, hash::Hash};

// advanced 2: keep items in order
fn unique<T: Eq + Hash + Copy + Ord>(mut a: Vec<T>) -> Vec<T> {
    let mut seen = HashSet::new();
    a.retain(|item| seen.insert(*item));
    a.sort();
    a
}

fn main() {
    let input = vec![2, 1, 1];
    let answer = unique(input);
    println!("unique items -> {:?}", answer)
}

#[test]
fn empty_list() {
    let input: Vec<i64> = vec![];
    let expected_output = vec![];
    let actual_output: Vec<_> = unique(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn sorted_list() {
    let input = vec![1, 4, 5];
    let expected_output = vec![1, 4, 5];
    let actual_output = unique(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn unsorted_list() {
    let input = vec![1, 5, 2];
    let expected_output = vec![1, 2, 5];
    let actual_output = unique(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn unsorted_list_with_duplicates() {
    let input = vec![1, 5, 2, 2, 1];
    let expected_output = vec![1, 2, 5];
    let actual_output = unique(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn sorted_list_with_duplicates() {
    let mut input = vec![1, 5, 2, 2, 1];
    input.sort_by(|x, y| x.partial_cmp(y).unwrap());
    let expected_output = vec![1, 2, 5];
    let actual_output = unique(input);
    assert_eq!(actual_output, expected_output);
}
