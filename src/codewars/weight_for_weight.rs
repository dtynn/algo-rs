use std::cmp::{Ord, Ordering};

pub fn order_weight(s: &str) -> String {
    fn cal_weight(s: &str) -> u32 {
        let zero: u32 = '0'.to_digit(10).unwrap();
        s.chars()
            .fold(0, |w, ch| w + (ch.to_digit(10).unwrap() - zero))
    }

    fn compare<'r, 's>(a: &'r String, b: &'s String) -> Ordering {
        let order = cal_weight(a).cmp(&cal_weight(b));
        match order {
            Ordering::Greater | Ordering::Less => order,
            Ordering::Equal => a.cmp(&b),
        }
    };

    let mut weights: Vec<String> = s.split_whitespace().map(|s| s.to_string()).collect();
    weights.sort_by(compare);

    weights.join(" ")
}

#[cfg(test)]
mod tests {
    use super::order_weight;

    fn testing(s: &str, exp: &str) -> () {
        assert_eq!(order_weight(s), exp)
    }

    #[test]
    fn basics_order_weight() {
        testing("103 123 4444 99 2000", "2000 103 123 4444 99");
        testing(
            "2000 10003 1234000 44444444 9999 11 11 22 123",
            "11 11 2000 10003 22 123 1234000 44444444 9999",
        );
    }

}
