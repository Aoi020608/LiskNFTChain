pub fn bubble_sort(arr: &mut [u32]) -> &mut [u32] {
    let mut length = arr.len();
    let mut sorted = false;

    while !sorted {
        sorted = true;
        for i in 0..length-1 {
            if arr[i] > arr[i+1] {
                arr.swap(i, i+1);
                sorted = false
            }
        }
        length -= 1;
    }

    arr
}


#[cfg(test)]
mod tests {
    use crate::bubble_sort::*;

    #[test]
    fn test_bubble_sort() {
        let mut v1 = [99, 44, 6, 2, 1, 5, 63, 87, 283, 4, 0];
        let sorted = bubble_sort(&mut v1);
        assert_eq!(sorted, [0, 1, 2, 4, 5, 6, 44, 63, 87, 99, 283]);
    }
}