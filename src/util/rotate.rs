pub fn rotate90<T: Clone>(grid: &[Vec<T>]) -> Vec<Vec<T>> {
    if grid.is_empty() {
        return Vec::new();
    }
    let cols = grid[0].len();
    (0..cols)
        .map(|x| grid.iter().rev().map(|row| row[x].clone()).collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::util::rotate::rotate90;

    #[test]
    fn test() {
        let a = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let b = rotate90(&a);
        assert_eq!(b, vec![vec![4, 1], vec![5, 2], vec![6, 3]]);
    }
}
