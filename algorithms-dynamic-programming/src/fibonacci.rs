// 0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233.....
pub fn fibonacci(n: i32) -> i32 {
    if n < 2 {
        n
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}

#[cfg(test)]
mod tests {
    use crate::fibonacci::*;
    #[test]
    fn name() {
        let num = fibonacci(3);
        assert_eq!(num, 2);
    }
}
