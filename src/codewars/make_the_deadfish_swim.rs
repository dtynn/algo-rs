pub fn parse(code: &str) -> Vec<i32> {
    let mut output = Vec::new();

    code.chars().fold(0, |value: i32, ch: char| match ch {
        'i' => return value + 1,
        'd' => return value - 1,
        's' => return value * value,
        'o' => {
            output.push(value);
            return value;
        }
        _ => return value,
    });

    output
}

#[cfg(test)]
mod tests {
    use super::parse;

    #[test]
    fn sample_tests() {
        assert_eq!(parse("iiisdoso"), vec![8, 64]);
        assert_eq!(parse("iiisdosodddddiso"), vec![8, 64, 3600]);
    }
}
