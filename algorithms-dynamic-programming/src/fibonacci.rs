// 0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233.....
// O(2 ^ N) reduce time complexity
// optimal
use std::collections::HashSet;

pub fn fibonacci(n: i32) -> i32 {
    if n < 2 {
        n
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}

pub fn fibonacci_1(n: i32) -> i32 {
    let mut history: HashSet<i32> = HashSet::new();
    if n < 2 {
        return n;
    }

    if history.contains(&n) {
        let num = history.get(&n).unwrap();
        return *num;
    }

    return fibonacci_1(n - 1) + fibonacci(n - 2);
}

pub fn fibonacci_2(n: i32) -> i32 {
    let mut answer: Vec<i32> = vec![0, 1];
    for i in 2..=n {
        answer.push(answer[i as usize - 2] + answer[i as usize - 1]);
    }
    return answer.pop().unwrap();
}

#[cfg(test)]
mod tests {
    use crate::fibonacci::*;
    #[test]
    fn name() {
        let num = fibonacci(3);
        assert_eq!(num, 2);
    }

    #[test]
    fn test_fibonacci_1() {
        let num = fibonacci_1(11);
        assert_eq!(num, 89);
    }

    #[test]
    fn test_fibonacci_2() {
        let num = fibonacci_2(11);
        assert_eq!(num, 89);
    }
}
