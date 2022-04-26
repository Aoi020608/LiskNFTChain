// O(N)
fn findFactorialRecursive(num: u32) -> u32 {
    if num <= 1 {
        num
    } else {
        num * findFactorialRecursive(num - 1)
    }
}

// exponentinal time
// O(2 ^ N)
fn fibonacci_recursive(num: u32) -> u32 {
    if num < 2 {
        num
    } else {
        fibonacci_iterative_recursive(num - 2) + fibonacci_iterative_recursive(num - 1)
    }
}

fn fibonacci_iterative_recursive(num: u32) -> u32 {
    let mut arr: Vec<u32> = vec![0, 1];
    for i in 2..num + 1 {
        let index2 = i as usize - 2 ;
        let num2 = arr[index2];

        let index1 = i as usize - 1;
        let num1 = arr[index1];

        arr.push(num2 + num1);
    }
    arr[num as usize]
}

fn reverse_string(string: &str) -> &str {
    let vec = string.chars().collect();
}

#[cfg(test)]
mod tests {
    use crate::factorial::*;
    #[test]
    fn test_factorial() {
        let number1 = findFactorialRecursive(4);
        assert_eq!(number1, 24);

        let number2 = findFactorialRecursive(3);
        assert_eq!(number2, 6);
    }

    #[test]
    fn test_fibonacci() {
        let number = fibonacci_recursive(3);
        assert_eq!(number, 2);

        let number2 = fibonacci_recursive(4);
        assert_eq!(number2, 3);

        let number3 = fibonacci_recursive(0);
        assert_eq!(number3, 0);
    }

    #[test]
    fn test_fibonacci_iterative() {
        let number = fibonacci_iterative_recursive(3);
        assert_eq!(number, 2);

        let number1 = fibonacci_iterative_recursive(0);
        assert_eq!(number1, 0);
    }
}
