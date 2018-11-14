pub fn prime_factors(n: i64) -> String {
    let mut factors: Vec<[i64; 2]> = Vec::new();

    let mut num = n;
    let mut factor = 2i64;

    while num > 1 {
        if num % factor != 0 {
            factor += 1;
            continue;
        }

        num = num / factor;

        if factors.is_empty() {
            factors.push([factor, 1]);
            continue;
        }

        let last_idx = factors.len() - 1;
        if factors[last_idx][0] == factor {
            factors[last_idx][1] += 1;
        } else {
            factors.push([factor, 1]);
        }
    }

    factors
        .into_iter()
        .map(|arr| {
            if arr[1] == 1 {
                format!("({})", arr[0])
            } else {
                format!("({}**{})", arr[0], arr[1])
            }
        }).collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::prime_factors;

    fn testing(n: i64, exp: &str) -> () {
        assert_eq!(&prime_factors(n), exp)
    }

    #[test]
    fn basics_prime_factors() {
        testing(7775460, "(2**2)(3**3)(5)(7)(11**2)(17)");
        testing(17 * 17 * 93 * 677, "(3)(17**2)(31)(677)");
    }
}
