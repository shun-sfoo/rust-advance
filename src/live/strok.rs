pub fn strtok<'a>(s: &'a mut &str, pattern: char) -> &'a str {
    match s.find(pattern) {
        Some(index) => {
            let prefix = &s[..index];
            let suffix = &s[index + pattern.len_utf8()..];
            *s = suffix;
            prefix
        }
        None => {
            let prefix = *s;
            prefix
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find() {
        let s = "hello world";
        assert_eq!(s.find(' '), Some(5));
    }

    #[test]
    fn test_strok() {
        let pattern = ' ';
        let mut s = "hello world";
        assert_eq!("hello", strtok(&mut s, pattern));
        assert_eq!("world", s);
    }
}
