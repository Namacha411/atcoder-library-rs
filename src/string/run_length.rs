pub fn run_length_encoding(str: String) -> Vec<(char, usize)> {
    let str = str.chars().collect::<Vec<_>>();
    let mut left = 0;
    let mut res = vec![];
    while left < str.len() {
        let mut right = left + 1;
        while right < str.len() && str[left] == str[right] {
            right += 1;
        }
        res.push((str[left], right - left));
        left = right;
    }
    res
}

pub fn run_length_decoding(rle_data: Vec<(char, usize)>) -> String {
    let mut str = String::new();
    for (c, i) in &rle_data {
        str.push_str(&c.to_string().repeat(*i));
    }
    str
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let s = String::from("aabbbbbccddeeeeeff");
        assert_eq!(s, run_length_decoding(run_length_encoding(s.clone())));
    }

    #[test]
    fn test2() {
        let s = String::from("a");
        assert_eq!(s, run_length_decoding(run_length_encoding(s.clone())));
    }

    #[test]
    fn test3() {
        let s = String::from("abcd");
        assert_eq!(s, run_length_decoding(run_length_encoding(s.clone())));
    }

    #[test]
    fn test4() {
        let s = String::from("aaaaaaaaaaaaaaaaaaaaaa");
        assert_eq!(s, run_length_decoding(run_length_encoding(s.clone())));
    }
}