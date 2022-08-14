//! Binary Indexed Tree
//!
//! 数列 $a_1, a_2, a_3, \cdots , a_n$ が与えられたとき以下の様なことができる
//! - iとxが与えられたとき、$a_i$にxを加算する
//! - iが与えられたとき、$a_1 + a_2 + a_3 + \cdots + a_n$を求める
//! 計算量は$O(log(N))$

type T = usize;
#[derive(Clone, Debug)]
pub struct BIT {
    size: usize,
    bit: Vec<T>,
}

impl BIT {
    pub fn new(n: usize) -> BIT {
        BIT {
            size: n + 1,
            bit: vec![0; n + 1],
        }
    }

    pub fn add(&mut self, i: usize, x: T) {
        let mut index = i as isize;
        while index < self.size as isize {
            self.bit[index as usize] += x;
            index += index & -index;
        }
    }

    pub fn sum(&mut self, i: usize) -> T {
        let mut s = 0 as T;
        let mut index = i as isize;
        while index > 0 {
            s += self.bit[index as usize];
            index -= index & -index;
        }
        s
    }

    #[deprecated = "テストしてない、ちゃんと動く保証はない"]
    pub fn sum_range(&mut self, range: std::ops::Range<usize>) -> T {
        let std::ops::Range { start, end } = range;
        self.sum(end - 1) - self.sum(start - 1)
    }
}
