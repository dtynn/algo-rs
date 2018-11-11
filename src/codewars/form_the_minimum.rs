pub fn min_value(mut digits: Vec<i32>) -> i32 {
    digits.sort();
    digits.dedup();

    digits.into_iter().fold(0, |res, ele| res * 10 + ele)
}

#[cfg(test)]
mod tests {
    use super::min_value;

    #[test]
    fn basic_test() {
        assert_eq!(min_value(vec![1, 3, 1]), 13);
        assert_eq!(min_value(vec![4, 7, 5, 7]), 457);
        assert_eq!(min_value(vec![4, 8, 1, 4]), 148);
    }
}
