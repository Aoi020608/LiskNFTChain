pub fn selection_sort(arr: &mut [u32]) -> &mut [u32] {
    let mut length = arr.len();
    let mut sorted = false;

    let mut number = 0;

    while number < length {
        let mut smallet_num = arr[number];
        let mut temp_index = number;
        sorted = true;

        for i in number + 1..length {
            if smallet_num > arr[i] {
                smallet_num = arr[i];
                temp_index = i;
            }
        }
        arr.swap(number, temp_index);

        length -= 1;
        number += 1;
    }

    arr
}

pub fn selection_sort_2(arr: &mut [u32]) -> &mut [u32] {
    let mut length = arr.len();

    for i in 0..length {
        // let mut smallet_number = arr[i];
        let mut temp_index = i;
        for j in (i + 1)..length {
            if arr[temp_index] > arr[j] {
                // arr[temp_index] = arr[j];
                temp_index = j;
            }
        }
        arr.swap(i, temp_index);
    }
    arr
}

pub fn selection_sort_3(arr: &mut [u32]) -> &mut [u32] {
    let len = arr.len();
    for left in 0..len {
        let mut smallet = left;
        for right in (left + 1)..len {
            if arr[right] < arr[smallet] {
                smallet = right;
            }
        }
        arr.swap(smallet, left);
    }

    arr
}

// pub fn selection_sort_4(arr: &mut [u32]) -> &mut [u32] {

// }

pub fn insertion_sort_3(arr: &mut [u32]) -> &mut [u32] {
    for i in 1..arr.len() {
        let cur = arr[i];
        let mut j = i - 1;

        while arr[j] > cur {
            arr.swap(j + 1, j);
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
    use crate::selection_sort::*;
    #[test]
    fn test_selection_sort() {
        let mut v1 = [99, 44, 6, 2, 1, 5, 63, 87, 283, 4, 0];
        let sorted = selection_sort_2(&mut v1);
        println!("{:?}", sorted);
        assert_eq!(sorted, [0, 1, 2, 4, 5, 6, 44, 63, 87, 99, 283]);
    }

    #[test]
    fn test_selection_3() {
        let mut v1 = [99, 44, 6, 2, 1, 5, 63, 87, 283, 4, 0];
        let sorted = selection_sort_3(&mut v1);
        println!("{:?}", sorted);
        assert_eq!(sorted, [0, 1, 2, 4, 5, 6, 44, 63, 87, 99, 283]);
    }
}
