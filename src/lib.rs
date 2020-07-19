pub fn firstn(s: &str, n: usize) -> &str {
    let idx = s
        .char_indices()
        .nth(n)
        .map(|(i, _)| i)
        .unwrap_or(s.len());

    return &s[..idx];
}

pub fn lastn(s: &str, n: usize) -> &str {
    let mut new_n = 0;
    let mut d = 0;  // default

    // hacky?
    if n == 0 {
        return "";
    }

    if n > s.len() {
        return s;
    }
    //

    if n > 0 {
        new_n = n - 1;
    }

    if s.len() > 0 {
        d = s.len() - 1;
    }

    let idx = s
        .char_indices()
        .rev()
        .nth(new_n)
        .map(|(i, _)| i)
        .unwrap_or(d);

    return &s[idx..];
}

pub fn slice(s: &str, start: usize, end: usize) -> &str {
    let mut start_byte = s.len();
    let mut end_byte = s.len();

    if end < start {
        return "";
    }

    for (i, (byte_idx, _)) in s.char_indices().enumerate() {
        if i == start {
            start_byte = byte_idx;
        }

        if i == end {
            end_byte = byte_idx;
            break;
        }
    }

    return unsafe { s.get_unchecked(start_byte..end_byte) };
}

#[cfg(test)]
mod tests {
    use crate::{firstn, lastn, slice};

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_nth() {
        assert_eq!("", firstn("test", 0));
        assert_eq!("t", firstn("test", 1));
        assert_eq!("te", firstn("test", 2));
        assert_eq!("tes", firstn("test", 3));
        assert_eq!("test", firstn("test", 4));
        assert_eq!("test", firstn("test", 10));
        assert_eq!("Hello", firstn("Hello, World!", 5));

        assert_eq!("", firstn("", 0));
        assert_eq!("", firstn("", 1));
        assert_eq!("", firstn("", 5));
    }

    #[test]
    fn test_last() {
        assert_eq!("", lastn("test", 0));
        assert_eq!("t", lastn("test", 1));
        assert_eq!("st", lastn("test", 2));
        assert_eq!("est", lastn("test", 3));
        assert_eq!("test", lastn("test", 4));
        assert_eq!("test", lastn("test", 10));
        assert_eq!("World!", lastn("Hello, World!", 6));

        assert_eq!("", lastn("", 0));
        assert_eq!("", lastn("", 1));
        assert_eq!("", lastn("", 11));
    }

    #[test]
    fn test_slice() {
        assert_eq!("", slice("test", 0, 0));
        assert_eq!("", slice("test", 3, 3));
        assert_eq!("t", slice("test", 0, 1));
        assert_eq!("es", slice("test", 1, 3));
        assert_eq!("st", slice("test", 2, 10));

        assert_eq!("", slice("test", 10, 15));
        assert_eq!("", slice("", 0, 5));

        assert_eq!("", slice("test", 3, 1));
    }
}
