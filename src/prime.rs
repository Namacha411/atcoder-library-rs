pub trait PrimeExt {
    fn is_prime(&self) -> bool;
    fn prime_factorize(&self) -> Vec<(Self, Self)>
    where
        Self: Sized;
}

macro_rules! impl_integer {
    ($($ty: ty), *) => {
        $(
            impl PrimeExt for $ty {
                fn is_prime(&self) -> bool {
                    let n = *self;
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
                fn prime_factorize(&self) -> Vec<($ty, $ty)> {
                    let mut res = Vec::new();
                    let mut n = *self;
                    for i in (2..).take_while(|&i| i*i <= *self) {
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
            }
        )*
    };
}

impl_integer!(u8, u16, u32, u64, u128, usize);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn is_prime_test() {
        assert!(!1u64.is_prime());
        assert!(2u64.is_prime());
        assert!(3u64.is_prime());
        assert!(1000000007u64.is_prime());
    }

    #[test]
    fn prime_factorize_test() {
        assert_eq!(1u64.prime_factorize(), []);
        assert_eq!(2u64.prime_factorize(), [(2, 1)]);
        assert_eq!(3u64.prime_factorize(), [(3, 1)]);
        assert_eq!(24u64.prime_factorize(), [(2, 3), (3, 1)]);
        assert_eq!(360u64.prime_factorize(), [(2, 3), (3, 2), (5, 1)]);
        assert_eq!(1000000007u64.prime_factorize(), [(1000000007, 1)]);
    }
}
