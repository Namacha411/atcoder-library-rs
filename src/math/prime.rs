pub fn is_prime(n: u64) -> bool {
    if n == 1 {
        return false;
    }
    for i in (2..).take_while(|&i| i*i <= n) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

pub fn prime_factorize(x: u64) -> Vec<(u64, usize)> {
    let mut res = Vec::new();
    let mut n = x;
    for i in (2..).take_while(|&i| i*i <= x) {
        if n % i != 0 {
            continue;
        }
        let mut exp = 0;
        while n % i == 0 {
            exp += 1;
            n /= i;
        }
        res.push((i, exp));
    }
    if n != 1 {
        res.push((n, 1));
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn is_prime_test() {
        assert!(!is_prime(64));
        assert!(is_prime(2));
        assert!(is_prime(3));
        assert!(is_prime(7));
    }

    #[test]
    fn prime_factorize_test() {
        assert_eq!(prime_factorize(1), []);
        assert_eq!(prime_factorize(2), [(2, 1)]);
        assert_eq!(prime_factorize(3), [(3, 1)]);
        assert_eq!(prime_factorize(24), [(2, 3), (3, 1)]);
        assert_eq!(prime_factorize(360), [(2, 3), (3, 2), (5, 1)]);
        assert_eq!(prime_factorize(1000000007), [(1000000007, 1)]);
    }
}
