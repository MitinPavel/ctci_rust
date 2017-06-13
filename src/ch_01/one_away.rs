/// # 1.5 One Away
///
/// Assumptions:
/// * ASCII charset
/// * If two strings are equal `is_one_edit_away` should return `false`.
///   For some reason the book solutions (6th edition) do return `true`
///   in case of identical strings.

mod divide_and_conquer {
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
}

mod single_method {
    pub fn is_one_edit_away(first: String, second: String) -> bool {
        if ((first.len() as i32) - (second.len() as i32)).abs() > 1 {
            return false;
        }

        let b1 = if first.len() < second.len() { first.as_bytes() } else { second.as_bytes() };
        let b2 = if first.len() < second.len() { second.as_bytes() } else { first.as_bytes() };

        let mut index_1 = 0;
        let mut index_2 = 0;
        let mut found_difference = false;

        while index_2 < b2.len() && index_1 < b1.len() {
            if b1[index_1] != b2[index_2] {
                if found_difference {
                    return false
                }
                found_difference = true;

                if b1.len() == b2.len() {
                    index_1 += 1
                }
            } else {
                index_1 += 1;
            }
            index_2 += 1;
        }

        // The book example (6th edition)
        // returns `true` unconditionally, so "a" and "a" is one edit replace.
        found_difference || b1.len() != b2.len()
    }
}

#[cfg(test)]
mod tests {
    fn test_one_away<F>(test: F) where F: Fn(String, String) -> bool {
        assert_eq!(true, test("a".to_string(), "b".to_string()));
        assert_eq!(true, test("ab".to_string(), "ac".to_string()));
        assert_eq!(true, test("ab".to_string(), "cb".to_string()));
        assert_eq!(true, test("ab".to_string(), "abc".to_string()));
        assert_eq!(true, test("ab".to_string(), "acb".to_string()));
        assert_eq!(true, test("ab".to_string(), "cab".to_string()));
        assert_eq!(true, test("a".to_string(), "ab".to_string()));
        assert_eq!(true, test("a".to_string(), "ba".to_string()));
        assert_eq!(true, test("ab".to_string(), "a".to_string()));
        assert_eq!(true, test("ab".to_string(), "b".to_string()));

        assert_eq!(false, test("a".to_string(), "a".to_string()));
        assert_eq!(false, test("ab".to_string(), "ab".to_string()));
        assert_eq!(false, test("abc".to_string(), "def".to_string()));
        assert_eq!(false, test("abc".to_string(), "abcde".to_string()));
        assert_eq!(false, test("abc".to_string(), "yzabc".to_string()));
    }

    #[test]
    fn it_tests_divide_and_conquer() {
        test_one_away(super::divide_and_conquer::is_one_edit_away);
    }

    #[test]
    fn it_tests_single_method() {
        test_one_away(super::single_method::is_one_edit_away);
    }
}
