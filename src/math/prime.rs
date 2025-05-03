pub fn is_prime(n: u64) -> bool {
    if n == 1 {
        return false;
    }
    for i in (2..).take_while(|&i| i * i <= n) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

pub fn prime_factorize(x: u64) -> Vec<(u64, usize)> {
    let mut res = Vec::new();
    let mut n = x;
    for i in (2..).take_while(|&i| i * i <= x) {
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

pub struct Eratosthenes {
    is_prime: Vec<bool>,
}
impl Eratosthenes {
    pub fn new(n: usize) -> Eratosthenes {
        let mut is_prime = vec![true; n + 1];

        is_prime[0] = false;
        is_prime[1] = false;
        for prime in 2..=n {
            if !is_prime[prime] {
                continue;
            }
            let start = prime * 2;
            for q in (start..=n).step_by(prime) {
                is_prime[q] = false;
            }
        }

        Eratosthenes { is_prime }
    }

    pub fn is_prime(&self, n: usize) -> bool {
        self.is_prime[n]
    }

    pub fn get_primes(&self, range: std::ops::Range<usize>) -> Vec<usize> {
        range.filter(|x| self.is_prime(*x)).collect()
    }
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

    #[test]
    fn eratosthenes1() {
        let p = Eratosthenes::new(20);
        assert!(!p.is_prime(1));
        assert!(p.is_prime(2));
        assert!(p.is_prime(3));
        assert!(!p.is_prime(4));
        assert!(p.is_prime(5));
        assert!(p.is_prime(7));
        assert!(p.is_prime(11));
        assert!(p.is_prime(13));
        assert!(p.is_prime(17));
    }

    #[test]
    fn eratosthenes2() {
        let p = Eratosthenes::new(20);
        assert_eq!(vec![2, 3, 5, 7], p.get_primes(0..10));
        assert_eq!(vec![11, 13, 17, 19], p.get_primes(10..20))
    }
}
