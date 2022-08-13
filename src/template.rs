pub trait VecExt {
    fn display(&self) -> String;
}
impl<T: std::fmt::Display> VecExt for Vec<T> {
    fn display(&self) -> String {
        self.iter().map(T::to_string).collect::<Vec<_>>().join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::VecExt;

    #[test]
    fn display_test() {
        let vec = vec![1, 2, 3, 4];
        assert_eq!("1 2 3 4", vec.display());

        let vec = vec!["Hello", "World"];
        assert_eq!("Hello World", vec.display());
    }
}
