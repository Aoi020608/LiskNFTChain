fn findFactorialRecursive(num: u32) -> u32 {
    if num <= 1 {
        num
    } else {
        num * findFactorialRecursive(num - 1)
    }
}

#[cfg(test)]
mod tests {
    use crate::factorial::*;
    #[test]
    fn it_works() {
        let number1 = findFactorialRecursive(4);
        assert_eq!(number1, 24);

        let number2 = findFactorialRecursive(3);
        assert_eq!(number2, 6);
    }
}
