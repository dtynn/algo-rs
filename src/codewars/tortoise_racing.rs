pub fn race(v1: i32, v2: i32, g: i32) -> Option<Vec<i32>> {
    if v1 >= v2 {
        return None;
    }

    let t = g * 3600 / (v2 - v1);
    let h = t / 3600;
    let s = t % 3600;

    Some(vec![h, s / 60, s % 60])
}

#[cfg(test)]
mod test {
    use super::race;

    #[test]
    fn basic_tests() {
        assert_eq!(race(720, 850, 70), Some(vec![0, 32, 18]));
        assert_eq!(race(80, 100, 40), Some(vec![2, 0, 0]));
        assert_eq!(race(80, 91, 37), Some(vec![3, 21, 49]));
        assert_eq!(race(820, 81, 550), None);
    }
}
