// O(N logN)
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

// Step 1: Divide the array into 2 parts
// Step 2: Sort one half of the array
// Step 3: Sort second half of the array
// Step 4: Merge the 2 halfs
// Step 5: Perform these operations recursively
fn merge_sort_1(mut arr: Vec<i32>, left: usize, right: usize) -> Vec<i32> {
    if right - 1 > left {
        let mid = left + (right - left) / 2;
        arr = merge_sort_1(arr, left, mid);
        arr = merge_sort_1(arr, mid, right);
        arr = merge1(arr, left, mid, right);
    }
    arr
}

fn merge1(mut arr: Vec<i32>, left: usize, mid: usize, right: usize) -> Vec<i32> {
    let n1 = mid - left;
    let n2 = right - mid;
    let mut L1 = arr.clone();
    let mut R1 = arr.clone();
    let L = &L1[left..mid];
    let R = &R1[mid..right];

    // Merge the temp arrays back into arr[l..r]
    let mut i = 0;
    let mut j = 0;
    let mut k = left;
    while i < n1 && j < n2 {
        if L[i] < R[j] {
            arr[k] = L[i];
            i = i + 1;
        } else {
            arr[k] = R[j];
            j = j + 1;
        }
        k = k + 1;
    }

    while i < n1 {
        arr[k] = L[i];
        i = i + 1;
        k = k + 1;
    }

    // Copy the remaining elements of R[], if there are any
    while j < n2 {
        arr[k] = R[j];
        j = j + 1;
        k = k + 1;
    }

    arr
}

#[cfg(test)]
mod tests {
    use crate::merge_sort::*;

    #[test]
    fn test_merge_sort() {
        let mut res = vec![10, 8, 4, 3, 1, 9, 2, 7, 5, 6];
        merge_sort(&mut res);
        assert_eq!(res, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }

    #[test]
    fn test_merge_sort1() {
        let mut arr: Vec<i32> = vec![64, 34, 25, 8, 22, 11, 9];
        arr = merge_sort_1(arr.clone(), 0, arr.len());
        assert_eq!(arr, vec!(8, 9, 11, 22, 25, 34, 64));
    }
}
