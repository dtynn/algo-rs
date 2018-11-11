use std::collections::HashMap;

pub fn duplicate_encode(word: &str) -> String {
    let mut exists: HashMap<char, char> = HashMap::new();
    word.chars().for_each(|ch: char| {
        let lower = ch.to_ascii_lowercase();
        if let Some(_) = exists.get(&lower) {
            exists.insert(lower, ')');
        } else {
            exists.insert(lower, '(');
        }
    });

    word.chars()
        .map(|ch: char| exists.get(&ch.to_ascii_lowercase()).unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::duplicate_encode;

    #[test]
    fn run_tests() {
        assert_eq!(duplicate_encode("din"), "(((");
        assert_eq!(duplicate_encode("recede"), "()()()");
        assert_eq!(duplicate_encode("Success"), ")())())", "should ignore case");
        assert_eq!(duplicate_encode("(( @"), "))((");
    }
}
