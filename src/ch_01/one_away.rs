/// # 1.5 One Away
///
/// Assumptions:
/// * ASCII charset

pub fn is_one_edit_away(first: String, second: String) -> bool {
    if first.len() == second.len() {
        return is_one_edit_replace(first, second)
    } else if first.len() + 1 == second.len() {
        return is_one_edit_insert(first, second)
    } else if first.len() - 1 == second.len() {
        return is_one_edit_insert(second, first)
    }

    false
}

fn is_one_edit_replace(s1: String, s2: String) -> bool {
    let bytes_1 = s1.as_bytes();
    let bytes_2 = s2.as_bytes();
    let mut found_difference = false;

    for i in 0..bytes_1.len() {
        if bytes_1[i] != bytes_2[i] {
            if found_difference {
                return false
            }

            found_difference = true
        }
    }

    // The book example (6th edition)
    // returns `true` unconditionally, so "a" and "a" is one edit replace.
    return found_difference
}

fn is_one_edit_insert(s1: String, s2: String) -> bool {
    let bytes_1 = s1.as_bytes();
    let bytes_2 = s2.as_bytes();

    let mut index_1 = 0;
    let mut index_2 = 0;

    while index_2 < bytes_2.len() && index_1 < bytes_1.len() {
        if bytes_2[index_2] != bytes_1[index_1] {
            if index_1 != index_2 {
                return false
            }
            index_2 += 1
        } else {
            index_1 += 1;
            index_2 += 1;
        }
    }

    true
}

#[cfg(test)]
mod test {
    #[test]
    fn check() {
        assert_eq!(true, super::is_one_edit_away("a".to_string(), "b".to_string()));
        assert_eq!(true, super::is_one_edit_away("ab".to_string(), "ac".to_string()));
        assert_eq!(true, super::is_one_edit_away("ab".to_string(), "cb".to_string()));
        assert_eq!(true, super::is_one_edit_away("ab".to_string(), "abc".to_string()));
        assert_eq!(true, super::is_one_edit_away("ab".to_string(), "acb".to_string()));
        assert_eq!(true, super::is_one_edit_away("ab".to_string(), "cab".to_string()));
        assert_eq!(true, super::is_one_edit_away("a".to_string(), "ab".to_string()));
        assert_eq!(true, super::is_one_edit_away("a".to_string(), "ba".to_string()));
        assert_eq!(true, super::is_one_edit_away("ab".to_string(), "a".to_string()));
        assert_eq!(true, super::is_one_edit_away("ab".to_string(), "b".to_string()));

        assert_eq!(false, super::is_one_edit_away("a".to_string(), "a".to_string()));
        assert_eq!(false, super::is_one_edit_away("ab".to_string(), "ab".to_string()));
        assert_eq!(false, super::is_one_edit_away("abc".to_string(), "def".to_string()));
        assert_eq!(false, super::is_one_edit_away("abc".to_string(), "abcde".to_string()));
        assert_eq!(false, super::is_one_edit_away("abc".to_string(), "yzabc".to_string()));
    }
}
