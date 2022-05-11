pub trait Substring {
    fn substring(&self, start: usize, end: usize) -> Self;
}

impl Substring for String {
    fn substring(&self, start: usize, end: usize) -> String {
        return self.chars().skip(start).take(end - start).collect();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn with_empty_string() {
        assert_eq!("".to_string(), "".to_string().substring(0, 5))
    }

    #[test]
    fn with_wider_substring() {
        assert_eq!("rt".to_string(), "art".to_string().substring(1, 10))
    }

    #[test]
    fn normal_substring() {
        assert_eq!("ovi".to_string(), "provide".to_string().substring(2, 5))
    }
}
