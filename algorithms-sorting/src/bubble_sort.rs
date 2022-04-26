pub fn bubble_sort(arr: &mut [u32]) -> &mut [u32] {
    let mut length = arr.len();
    let mut sorted = false;

    while !sorted {
        sorted = true;
        for i in 0..length - 1 {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                sorted = false
            }
        }
        length -= 1;
    }

    arr
}

pub fn bubble_sort1(arr: &mut [u32]) -> &mut [u32] {
    let length = arr.len();

    for i in 0..length {
        for j in 0..length - 1 {
            if arr[j] > arr[j + 1] {
                let temp = arr[j];
                arr[j] = arr[j + 1];
                arr[j + 1] = temp;
            }
        }
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
        println!("{:?}", sorted);
        assert_eq!(sorted, [0, 1, 2, 4, 5, 6, 44, 63, 87, 99, 283]);
    }

    #[test]
    fn test_bubble_sort1() {
        let mut v1 = [99, 44, 6, 2, 1, 5, 63, 87, 283, 4, 0];
        let sorted = bubble_sort1(&mut v1);
        println!("{:?}", sorted);
        assert_eq!(sorted, [0, 1, 2, 4, 5, 6, 44, 63, 87, 99, 283]);
    }
}
