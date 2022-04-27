// insertion sort
// almost sorted
// 
pub fn insertion_sort(arr: &mut [u32]) -> &mut [u32] {
    let length = arr.len();

    for i in 1..length {
        let cur = arr[i];
        let mut j = i - 1;

        while arr[j] > cur {
            arr.swap(j + 1, i);
            if j == 0 {
                break;
            }
            j -= 1;
        }
    }

    arr
}




#[cfg(test)]
mod tests {
    use crate::insertion_sort::*;
    #[test]
    fn test_insertion_sort() {
        let mut v1 = [99, 44, 6, 2, 1, 5, 63, 87, 283, 4, 0];
        let sorted = insertion_sort(&mut v1);
        println!("{:?}", sorted);
        assert_eq!(sorted, [0, 1, 2, 4, 5, 6, 44, 63, 87, 99, 283]);
    }
}

