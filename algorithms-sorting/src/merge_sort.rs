// O(N logN)
//

fn merge(arr: &mut [u32], mid: usize) {
    // Create temporary vectors to support the merge.__rust_force_expr
    let left_half = arr[..mid].to_vec();
    let right_half = arr[mid..].to_vec();

    // Indexes to track the positions while merging.
    let mut l = 0;
    let mut r = 0;

    for v in arr {
        // Choose either the smaller element, of from whichever vec is not exhausted
        if r == right_half.len() || (l < left_half.len() && left_half[l] < right_half[r]) {
            *v = left_half[l];
            l += 1;
        } else {
            *v = right_half[r];
            r += 1;
        }
    }
}

pub fn merge_sort(arr: &mut [u32]) {
    if arr.len() > 1 {
        let mid = arr.len() / 2;
        // Sort the left half recursively
        merge_sort(&mut arr[..mid]);
        // Sort the right half recursively
        merge_sort(&mut arr[mid..]);
        // Combine the two halves
        merge(arr, mid);
    }
}

#[cfg(test)]
mod tests {
    use crate::merge_sort::*;

    #[test]
    fn test_merge_sort() {
        let mut res = vec!(10, 8, 4, 3, 1, 9, 2, 7, 5, 6);
        merge_sort(&mut res);
        assert_eq!(res, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }
}
