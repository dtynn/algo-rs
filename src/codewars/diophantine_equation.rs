pub fn solequa(n: u64) -> Vec<(u64, u64)> {
    // your code
    let mut res = Vec::new();
    let max = (n as f64).sqrt().floor() as u64;
    for i in 1..max + 1 {
        let diff = n - i.pow(2);
        if diff % (4 * i) == 0 {
            let y = diff / (4 * i);
            let x = i + 2 * y;
            res.push((x, y));
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::solequa;

    fn testing(n: u64, exp: Vec<(u64, u64)>) -> () {
        assert_eq!(solequa(n), exp)
    }

    #[test]
    fn basics_solequa() {
        testing(5, vec![(3, 1)]);
        testing(20, vec![(6, 2)]);
        testing(9001, vec![(4501, 2250)]);
        testing(9004, vec![(2252, 1125)]);
        testing(
            90005,
            vec![(45003, 22501), (9003, 4499), (981, 467), (309, 37)],
        );
    }
}
