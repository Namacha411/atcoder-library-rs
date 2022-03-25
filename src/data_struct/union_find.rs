pub struct UnionFind {
    n: usize,
    parent_or_size: Vec<i32>,
}

impl UnionFind {
    pub fn new(size: usize) -> Self {
        Self {
            n: size,
            parent_or_size: vec![-1; size],
        }
    }

    pub fn merge(&mut self, a: usize, b: usize) -> usize {
        let mut x = self.parent(a);
        let mut y = self.parent(b);
        if x == y {
            return x;
        }
        if -self.parent_or_size[x] < -self.parent_or_size[y] {
            std::mem::swap(&mut x, &mut y);
        }
        self.parent_or_size[x] += self.parent_or_size[y];
        self.parent_or_size[y] = x as i32;
        x
    }

    pub fn same(&mut self, a: usize, b: usize) -> bool {
        self.parent(a) == self.parent(b)
    }

    pub fn parent(&mut self, a: usize) -> usize {
        if self.parent_or_size[a] < 0 {
            return a;
        }
        self.parent_or_size[a] = self.parent(self.parent_or_size[a] as usize) as i32;
        self.parent_or_size[a] as usize
    }

    pub fn size(&mut self, a: usize) -> usize {
        let x = self.parent(a);
        -self.parent_or_size[x] as usize
    }

    pub fn make_groups(&mut self) -> Vec<Vec<usize>> {
        let mut parent_buf = vec![0; self.n];
        let mut group_size = vec![0; self.n];
        for i in 0..self.n {
            parent_buf[i] = self.parent(i);
            group_size[parent_buf[i]] += 1;
        }
        let mut res = vec![Vec::new(); self.n];
        for i in 0..self.n {
            res[i].reserve(group_size[i]);
        }
        for i in 0..self.n {
            res[parent_buf[i]].push(i);
        }
        res
            .into_iter()
            .filter(|x| !x.is_empty())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uf_test() {
        let mut d = UnionFind::new(4);
        d.merge(0, 1);
        assert!(d.same(0, 1));
        d.merge(1, 2);
        assert!(d.same(0, 2));
        assert_eq!(d.size(0), 3);
        assert!(!d.same(0, 3));
        assert_eq!(d.make_groups(), vec![vec![0, 1, 2], vec![3]]);
    }
}
