use std::collections::HashMap;
use crate::has_pair::has_pair_with_sum;

mod has_pair;

fn main() {
    // let boolean_value = contains_common_item1(vec!['a', 'b', 'c'], vec!['x', 'y', 'g']);

    let boolean_value = has_pair_with_sum(vec!(1, 7, 4, 5), 8);

    println!("{}", boolean_value);
}

// O(n ^ 2)
fn contains_common_item1(arr1: Vec<char>, arr2: Vec<char>) -> bool {
    for i in 0..arr1.len() {
        for j in 0..arr2.len() {
            if arr1[i] == arr2[j] {
                return true;
            }
        }
    }
    return false;
}

// O(a + b)
fn contains_common_item2(arr1: Vec<char>, arr2: Vec<char>) -> bool {
    let mut map = HashMap::new();

    // loop through first array and create object where properties === items in the array
    for i in 0..arr1.len() {
        map.insert(arr1[i].clone(), true);
    }

    // loop through second array and check if item in second array exists on created object.
    for j in 0..arr2.len() {
        let found = map.get_key_value(&arr2[j]);
        match found {
            Some((key, _)) => {
                println!("Found {:?}", key);
                return true;
            }
            _ => {}
        }
    }

    return false;
}
