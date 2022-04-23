use std::collections::HashSet;

pub fn has_pair_with_sum(arr1: Vec<i32>, sum: i32) -> bool {
    let len = arr1.len();
    for i in 0..len {
        for j in i + 1..len {
            if arr1[i] + arr1[j] == sum {
                return true;
            }
        }
    }
    return false;
}

pub fn has_pair_with_sum_2(arr1: Vec<i32>, sum: i32) -> bool {
    let mut vec1: HashSet<i32> = HashSet::new();
    let len = arr1.len();
    for i in 0..len {
        let res = vec1.get(&arr1[i]);
        match res {
            Some(value) => {
                if *value == arr1[i] {
                    return true;
                }
            }
            _ => {}
        }
        vec1.insert(sum - arr1[i]);
    }

    return false;
}
