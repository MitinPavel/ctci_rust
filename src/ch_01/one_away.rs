/// # 1.5 One Away

pub fn is_one_away(s1: String, s2: String) -> bool {
    let length_diff = ((s1.len() as i32) - (s2.len() as i32)).abs();

    if length_diff == 0 {
        is_exactly_one_char_substituted(s1, s2)
    } else if length_diff == 1 {
        is_exactly_one_char_added(s1, s2)
    } else {
        false
    }
}

fn is_exactly_one_char_substituted(s1: String, s2: String) -> bool {
    let bytes_1 = s1.as_bytes();
    let bytes_2 = s2.as_bytes();
    let mut diff_count = 0;

    for i in 0..s1.len() {
        if bytes_1[i] != bytes_2[i] {
            diff_count += 1;
        }
        if diff_count > 1 {
            return false
        }
    }

    diff_count == 1
}

fn is_exactly_one_char_added(s1: String, s2: String) -> bool {
    if s1.len() > s2.len() {
        is_exactly_one_char_added_to(s2, s1)
    } else {
        is_exactly_one_char_added_to(s1, s2)
    }
}

fn is_exactly_one_char_added_to(shorter_string: String, longer_string: String) -> bool {
    let shorter_bytes = shorter_string.as_bytes();
    let longer_bytes = longer_string.as_bytes();

    let mut shorter_index = 0;
    let mut longer_index = 0;

    while longer_index < longer_bytes.len() && shorter_index < shorter_bytes.len() {
        if longer_bytes[longer_index] != shorter_bytes[shorter_index] {
            if shorter_index != longer_index {
                return false
            }
            longer_index += 1
        } else {
            shorter_index += 1;
            longer_index += 1;
        }
    }

    true
}

#[cfg(test)]
mod test {
    #[test]
    fn check() {
        assert_eq!(true, super::is_one_away("a".to_string(), "b".to_string()));
        assert_eq!(true, super::is_one_away("ab".to_string(), "ac".to_string()));
        assert_eq!(true, super::is_one_away("ab".to_string(), "cb".to_string()));
        assert_eq!(true, super::is_one_away("ab".to_string(), "abc".to_string()));
        assert_eq!(true, super::is_one_away("ab".to_string(), "acb".to_string()));
        assert_eq!(true, super::is_one_away("ab".to_string(), "cab".to_string()));
        assert_eq!(true, super::is_one_away("a".to_string(), "ab".to_string()));
        assert_eq!(true, super::is_one_away("a".to_string(), "ba".to_string()));
        assert_eq!(true, super::is_one_away("ab".to_string(), "a".to_string()));
        assert_eq!(true, super::is_one_away("ab".to_string(), "b".to_string()));

        assert_eq!(false, super::is_one_away("a".to_string(), "a".to_string()));
        assert_eq!(false, super::is_one_away("abc".to_string(), "def".to_string()));
        assert_eq!(false, super::is_one_away("abc".to_string(), "abcde".to_string()));
        assert_eq!(false, super::is_one_away("abc".to_string(), "yzabc".to_string()));
    }
}
