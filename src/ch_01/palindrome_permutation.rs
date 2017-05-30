/// # Palindrome Permutation
///
/// ## Assumptions:
///
/// * ASCII symbols only
/// * Case sensitive

/// ## Using an array of char counts and finding is it more than one odd count
mod char_counts {
    const CHARSET_SIZE: usize = 128;

    pub fn is_palindrome_permutation(string: String) -> bool {
        let mut char_counts: &mut [usize; CHARSET_SIZE] = &mut [0; CHARSET_SIZE];

        for char in string.chars() {
            char_counts[char as usize] += 1;
        }

        let mut odd_counts = 0;

        for i in 0..CHARSET_SIZE {
            let count = char_counts[i];
            if count % 2 != 0 {
                odd_counts += 1;
            }
            if odd_counts > 1 {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn it_tests() {
        assert_eq!(true, super::char_counts::is_palindrome_permutation("".to_string()));
        assert_eq!(true, super::char_counts::is_palindrome_permutation("a".to_string()));
        assert_eq!(true, super::char_counts::is_palindrome_permutation("aa".to_string()));

        assert_eq!(true, super::char_counts::is_palindrome_permutation("aab".to_string()));
        assert_eq!(true, super::char_counts::is_palindrome_permutation("aba".to_string()));
        assert_eq!(true, super::char_counts::is_palindrome_permutation("baa".to_string()));

        assert_eq!(true, super::char_counts::is_palindrome_permutation("baab".to_string()));
        assert_eq!(true, super::char_counts::is_palindrome_permutation("baba".to_string()));
        assert_eq!(true, super::char_counts::is_palindrome_permutation("bbaa".to_string()));
        assert_eq!(true, super::char_counts::is_palindrome_permutation("abab".to_string()));
        assert_eq!(true, super::char_counts::is_palindrome_permutation("abba".to_string()));
        assert_eq!(true, super::char_counts::is_palindrome_permutation("aabb".to_string()));

        assert_eq!(true, super::char_counts::is_palindrome_permutation("cbaab".to_string()));
        assert_eq!(true, super::char_counts::is_palindrome_permutation("cbaba".to_string()));
        assert_eq!(true, super::char_counts::is_palindrome_permutation("cbbaa".to_string()));
        assert_eq!(true, super::char_counts::is_palindrome_permutation("cabab".to_string()));
        assert_eq!(true, super::char_counts::is_palindrome_permutation("cabba".to_string()));
        assert_eq!(true, super::char_counts::is_palindrome_permutation("caabb".to_string()));

        assert_eq!(true, super::char_counts::is_palindrome_permutation("bcaab".to_string()));
        assert_eq!(true, super::char_counts::is_palindrome_permutation("bcaba".to_string()));
        assert_eq!(true, super::char_counts::is_palindrome_permutation("bcbaa".to_string()));
        assert_eq!(true, super::char_counts::is_palindrome_permutation("acbab".to_string()));
        assert_eq!(true, super::char_counts::is_palindrome_permutation("acbba".to_string()));
        assert_eq!(true, super::char_counts::is_palindrome_permutation("acabb".to_string()));

        assert_eq!(true, super::char_counts::is_palindrome_permutation("bacab".to_string()));
        assert_eq!(true, super::char_counts::is_palindrome_permutation("bacba".to_string()));
        assert_eq!(true, super::char_counts::is_palindrome_permutation("bbcaa".to_string()));
        assert_eq!(true, super::char_counts::is_palindrome_permutation("abcab".to_string()));
        assert_eq!(true, super::char_counts::is_palindrome_permutation("abcba".to_string()));
        assert_eq!(true, super::char_counts::is_palindrome_permutation("aacbb".to_string()));

        assert_eq!(true, super::char_counts::is_palindrome_permutation("baacb".to_string()));
        assert_eq!(true, super::char_counts::is_palindrome_permutation("babca".to_string()));
        assert_eq!(true, super::char_counts::is_palindrome_permutation("bbaca".to_string()));
        assert_eq!(true, super::char_counts::is_palindrome_permutation("abacb".to_string()));
        assert_eq!(true, super::char_counts::is_palindrome_permutation("abbca".to_string()));
        assert_eq!(true, super::char_counts::is_palindrome_permutation("aabcb".to_string()));

        assert_eq!(true, super::char_counts::is_palindrome_permutation("baabc".to_string()));
        assert_eq!(true, super::char_counts::is_palindrome_permutation("babac".to_string()));
        assert_eq!(true, super::char_counts::is_palindrome_permutation("bbaac".to_string()));
        assert_eq!(true, super::char_counts::is_palindrome_permutation("ababc".to_string()));
        assert_eq!(true, super::char_counts::is_palindrome_permutation("abbac".to_string()));
        assert_eq!(true, super::char_counts::is_palindrome_permutation("aabbc".to_string()));

        assert_eq!(false, super::char_counts::is_palindrome_permutation("ab".to_string()));
        assert_eq!(false, super::char_counts::is_palindrome_permutation("ab".to_string()));
        assert_eq!(false, super::char_counts::is_palindrome_permutation("abc".to_string()));
        assert_eq!(false, super::char_counts::is_palindrome_permutation("abcd".to_string()));
        assert_eq!(false, super::char_counts::is_palindrome_permutation("aabbcdde".to_string()));
    }
}
