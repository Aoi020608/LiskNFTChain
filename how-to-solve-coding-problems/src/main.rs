use std::collections::HashMap;

fn main() {
    let boolean_value = contains_common_item2(vec!['a', 'b', 'c'], vec!['x', 'y', 'c']);

    println!("{}", boolean_value);
}

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
