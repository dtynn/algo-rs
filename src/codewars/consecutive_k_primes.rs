use std::collections::HashMap;

pub fn consec_kprimes(k: i32, arr: Vec<i32>) -> i32 {
    if arr.len() < 2 {
        return 0;
    }

    let mut count = 0i32;

    let mut memo = Vec::new();
    for _ in 0..k {
        memo.push(HashMap::new());
    }

    for i in 0..arr.len() - 1 {
        if is_kprime(k, arr[i], &mut memo) && is_kprime(k, arr[i + 1], &mut memo) {
            count += 1;
        }
    }

    count
}

fn is_kprime(k: i32, num: i32, memo: &mut Vec<HashMap<i32, bool>>) -> bool {
    if let Some(exists) = memo[(k - 1) as usize].get(&num) {
        return *exists;
    }

    let is: bool;

    if k == 1 {
        is = !(2..(num / 2 + 1)).any(|i| num % i == 0);
    } else {
        is = (2..(num / 2 + 1))
            .any(|i| num % i == 0 && is_kprime(1, i, memo) && is_kprime(k - 1, num / i, memo));
    }

    memo[(k - 1) as usize].insert(num, is);

    is
}

#[cfg(test)]
mod tests {
    use super::consec_kprimes;

    fn testing(k: i32, arr: Vec<i32>, exp: i32) -> () {
        assert_eq!(consec_kprimes(k, arr), exp)
    }
    #[test]
    fn basics_consec_kprimes() {
        testing(4, vec![10175, 10185, 10180, 10197], 3);
        testing(
            2,
            vec![
                10081, 10071, 10077, 10065, 10060, 10070, 10086, 10083, 10078, 10076, 10089, 10085,
                10063, 10074, 10068, 10073, 10072, 10075,
            ],
            2,
        );
        testing(6, vec![10064], 0);
        testing(
            1,
            vec![10054, 10039, 10053, 10051, 10047, 10043, 10037, 10034],
            0,
        );
        testing(
            3,
            vec![
                10158, 10182, 10184, 10172, 10179, 10168, 10156, 10165, 10155, 10161, 10178, 10170,
            ],
            5,
        );
        testing(
            2,
            vec![
                10110, 10102, 10097, 10113, 10123, 10109, 10118, 10119, 10117, 10115, 10101, 10121,
                10122,
            ],
            7,
        );
    }

}

mod better {
    #![allow(dead_code)]

    fn prime_factors(n: i32) -> i32 {
        let mut nb = n;
        let mut i = 2;
        let mut cnt = 0;
        while nb > 1 {
            while nb % i == 0 {
                cnt += 1;
                nb /= i;
            }
            i += 1;
        }
        cnt
    }

    pub fn consec_kprimes(k: i32, arr: Vec<i32>) -> i32 {
        let mut i: usize = 0 as usize;
        let mut cnt = 0;
        while i < arr.len() - 1 {
            if (prime_factors(arr[i]) == k) && (prime_factors(arr[i + 1]) == k) {
                cnt += 1
            }
            i += 1;
        }
        cnt
    }
}
