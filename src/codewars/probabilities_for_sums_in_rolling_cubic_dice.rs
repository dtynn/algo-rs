pub fn rolldice_sum_prob(sum: i32, dice_amount: i32) -> f64 {
    (roll_possible(sum, dice_amount) as f64) / 6f64.powi(dice_amount)
}

fn roll_possible(sum: i32, dice_amount: i32) -> i32 {
    if sum < 1 * dice_amount || sum > 6 * dice_amount {
        return 0;
    }

    if dice_amount == 1 {
        return 1;
    }

    (0..6).fold(0, |poss: i32, num: i32| {
        poss + roll_possible(sum - num - 1, dice_amount - 1)
    })
}

#[cfg(test)]
mod tests {
    use super::rolldice_sum_prob;

    fn assert_fuzzy_eq(actual: f64, expected: f64, eps: f64) {
        assert!(
            (actual - expected).abs() < eps,
            format!("Expected {}, but got {}", expected, actual)
        );
    }

    #[test]
    fn returns_expected() {
        assert_fuzzy_eq(rolldice_sum_prob(11, 2), 0.055555555555, 1e-10);
        assert_fuzzy_eq(rolldice_sum_prob(8, 2), 0.13888888889, 1e-10);
        assert_fuzzy_eq(rolldice_sum_prob(8, 3), 0.0972222222222, 1e-10);
    }
}
