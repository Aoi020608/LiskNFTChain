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
