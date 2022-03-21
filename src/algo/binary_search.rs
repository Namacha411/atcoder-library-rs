pub fn lower_bound<T>(iterable: &[T], key: T) -> usize
where T: Ord
{
    let mut first = 0;
    let mut length = Some(iterable.len());

    while let Some(len) = length {
        if len == 0 {
            break;
        }
        let half = len / 2;
        let middle = first + half;
        if iterable[middle] < key {
            first = middle;
            first += 1;
            length = len.checked_sub(half + 1);
        } else {
            length = Some(half);
        }
    }

    first
}

pub fn upper_bound<T>(iterable: &[T], key: T) -> usize
where T: Ord
{
    let mut first = 0;
    let mut length = Some(iterable.len());

    while let Some(len) = length {
        if len == 0 {
            break;
        }
        let half = len / 2;
        let middle = first + half;
        if key < iterable[middle] {
            length = Some(half);
        } else {
            first = middle;
            first += 1;
            length = len.checked_sub(half + 1);
        }
    }

    first
}

#[cfg(test)]
mod lower_bound_tests {
    use super::lower_bound;

    #[test]
    fn lower_bound_test1() {
        let data = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let index = lower_bound(&data, 3);
        assert_eq!(index, 2);
    }

    #[test]
    fn lower_bound_test2() {
        let data = vec![1, 2, 4, 5, 6, 7, 8];
        let index = lower_bound(&data, 3);
        assert_eq!(index, 2);
    }

    #[test]
    fn lower_bound_test3() {
        let data = vec![1, 2, 3, 3, 3, 4, 5, 6, 7, 8];
        let index = lower_bound(&data, 3);
        assert_eq!(index, 2);
    }

    #[test]
    fn lower_bound_test4() {
        let data = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let index = lower_bound(&data, 9);
        assert_eq!(index, 8);
    }

    #[test]
    fn lower_bound_test5() {
        let data = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let index = lower_bound(&data, 0);
        assert_eq!(index, 0);
    }
}

#[cfg(test)]
mod upper_bound_tests {
    use super::upper_bound;

    #[test]
    fn upper_bound_test1() {
        let data = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let index = upper_bound(&data, 3);
        assert_eq!(index, 3);
    }

    #[test]
    fn upper_bound_test2() {
        let data = vec![1, 2, 4, 5, 6, 7, 8];
        let index = upper_bound(&data, 3);
        assert_eq!(index, 2);
    }

    #[test]
    fn upper_bound_test3() {
        let data = vec![1, 2, 3, 3, 3, 4, 5, 6, 7, 8];
        let index = upper_bound(&data, 3);
        assert_eq!(index, 5);
    }

    #[test]
    fn upper_bound_test4() {
        let data = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let index = upper_bound(&data, 9);
        assert_eq!(index, 8);
    }

    #[test]
    fn upper_bound_test5() {
        let data = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let index = upper_bound(&data, 0);
        assert_eq!(index, 0);
    }
}