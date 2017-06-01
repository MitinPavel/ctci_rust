/// # 1.4 Palindrome Permutation

/// ## Using an array of char counts and finding is it more than one odd count
/// Assumptions:
/// * ASCII symbols only
/// * Case sensitive
/// * Whitespaces are significant
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

/// ## Optimizing space w/ a bool array
/// Assumptions:
/// * ASCII symbols only
/// * Case sensitive
/// * Whitespaces are significant
mod bool_array {
    const CHARSET_SIZE: usize = 128;

    pub fn is_palindrome_permutation(string: String) -> bool {
        let mut char_oddness: &mut [bool; CHARSET_SIZE] = &mut [false; CHARSET_SIZE];

        for char in string.chars() {
            char_oddness[char as usize] = !char_oddness[char as usize];
        }

        let mut odd_counts = 0;

        for i in 0..CHARSET_SIZE {
            if char_oddness[i] {
                odd_counts += 1;
            }
            if odd_counts > 1 {
                return false;
            }
        }

        true
    }
}

/// ## Optimizing space w/ a bit vector
/// Assumptions:
/// * Only a-z char range is taken into account
mod bit_vector {
    pub fn is_palindrome_permutation(string: String) -> bool {
        let bit_vector = create_bit_vector(string);

        bit_vector == 0 || check_exactly_one_bit_set(bit_vector)
    }

    fn create_bit_vector(string: String) -> i32 {
        let mut bit_vector = 0;

        for char in string.chars() {
            let a = 'a' as i32;
            let z = 'z' as i32;
            let current = char as i32;
            if current >= a && current <= z {
                let index = (current - a) as u8;
                bit_vector = toggle(bit_vector, index);
            }
        }

        bit_vector
    }

    fn toggle(mut bit_vector: i32, index: u8) -> i32 {
        let mask = 1 << index;

        if (bit_vector & mask) == 0 {
            bit_vector |= mask;
        } else {
            bit_vector &= !mask;
        }

        return bit_vector
    }

    fn check_exactly_one_bit_set(bit_vector: i32) -> bool {
        (bit_vector & (bit_vector - 1)) == 0
    }
}

#[cfg(test)]
mod test {
    fn test_palindrome_permutation<F>(test: F) where F: Fn(String) -> bool {
        // true for a, b, c
        assert_eq!(true, test("".to_string()));
        assert_eq!(true, test("a".to_string()));
        assert_eq!(true, test("aa".to_string()));

        assert_eq!(true, test("aab".to_string()));
        assert_eq!(true, test("aba".to_string()));
        assert_eq!(true, test("baa".to_string()));

        assert_eq!(true, test("baab".to_string()));
        assert_eq!(true, test("baba".to_string()));
        assert_eq!(true, test("bbaa".to_string()));
        assert_eq!(true, test("abab".to_string()));
        assert_eq!(true, test("abba".to_string()));
        assert_eq!(true, test("aabb".to_string()));

        assert_eq!(true, test("cbaab".to_string()));
        assert_eq!(true, test("cbaba".to_string()));
        assert_eq!(true, test("cbbaa".to_string()));
        assert_eq!(true, test("cabab".to_string()));
        assert_eq!(true, test("cabba".to_string()));
        assert_eq!(true, test("caabb".to_string()));

        assert_eq!(true, test("bcaab".to_string()));
        assert_eq!(true, test("bcaba".to_string()));
        assert_eq!(true, test("bcbaa".to_string()));
        assert_eq!(true, test("acbab".to_string()));
        assert_eq!(true, test("acbba".to_string()));
        assert_eq!(true, test("acabb".to_string()));

        assert_eq!(true, test("bacab".to_string()));
        assert_eq!(true, test("bacba".to_string()));
        assert_eq!(true, test("bbcaa".to_string()));
        assert_eq!(true, test("abcab".to_string()));
        assert_eq!(true, test("abcba".to_string()));
        assert_eq!(true, test("aacbb".to_string()));

        assert_eq!(true, test("baacb".to_string()));
        assert_eq!(true, test("babca".to_string()));
        assert_eq!(true, test("bbaca".to_string()));
        assert_eq!(true, test("abacb".to_string()));
        assert_eq!(true, test("abbca".to_string()));
        assert_eq!(true, test("aabcb".to_string()));

        assert_eq!(true, test("baabc".to_string()));
        assert_eq!(true, test("babac".to_string()));
        assert_eq!(true, test("bbaac".to_string()));
        assert_eq!(true, test("ababc".to_string()));
        assert_eq!(true, test("abbac".to_string()));
        assert_eq!(true, test("aabbc".to_string()));

        // true for x, y, z
        assert_eq!(true, test("x".to_string()));
        assert_eq!(true, test("xx".to_string()));

        assert_eq!(true, test("xxy".to_string()));
        assert_eq!(true, test("xyx".to_string()));
        assert_eq!(true, test("yxx".to_string()));

        assert_eq!(true, test("yxxy".to_string()));
        assert_eq!(true, test("yxyx".to_string()));
        assert_eq!(true, test("yyxx".to_string()));
        assert_eq!(true, test("xyxy".to_string()));
        assert_eq!(true, test("xyyx".to_string()));
        assert_eq!(true, test("xxyy".to_string()));

        assert_eq!(true, test("zyxxy".to_string()));
        assert_eq!(true, test("zyxyx".to_string()));
        assert_eq!(true, test("zyyxx".to_string()));
        assert_eq!(true, test("zxyxy".to_string()));
        assert_eq!(true, test("zxyyx".to_string()));
        assert_eq!(true, test("zxxyy".to_string()));

        assert_eq!(true, test("yzxxy".to_string()));
        assert_eq!(true, test("yzxyx".to_string()));
        assert_eq!(true, test("yzyxx".to_string()));
        assert_eq!(true, test("xzyxy".to_string()));
        assert_eq!(true, test("xzyyx".to_string()));
        assert_eq!(true, test("xzxyy".to_string()));

        assert_eq!(true, test("yxzxy".to_string()));
        assert_eq!(true, test("yxzyx".to_string()));
        assert_eq!(true, test("yyzxx".to_string()));
        assert_eq!(true, test("xyzxy".to_string()));
        assert_eq!(true, test("xyzyx".to_string()));
        assert_eq!(true, test("xxzyy".to_string()));

        assert_eq!(true, test("yxxzy".to_string()));
        assert_eq!(true, test("yxyzx".to_string()));
        assert_eq!(true, test("yyxzx".to_string()));
        assert_eq!(true, test("xyxzy".to_string()));
        assert_eq!(true, test("xyyzx".to_string()));
        assert_eq!(true, test("xxyzy".to_string()));

        assert_eq!(true, test("yxxyz".to_string()));
        assert_eq!(true, test("yxyxz".to_string()));
        assert_eq!(true, test("yyxxz".to_string()));
        assert_eq!(true, test("xyxyz".to_string()));
        assert_eq!(true, test("xyyxz".to_string()));
        assert_eq!(true, test("xxyyz".to_string()));

        // false for a, b, c, d
        assert_eq!(false, test("ab".to_string()));
        assert_eq!(false, test("ab".to_string()));
        assert_eq!(false, test("abc".to_string()));
        assert_eq!(false, test("abcd".to_string()));
        assert_eq!(false, test("aabbcdde".to_string()));
    }

    #[test]
    fn it_tests_char_counts() {
        test_palindrome_permutation(super::char_counts::is_palindrome_permutation);
    }

    #[test]
    fn it_tests_bool_array() {
        test_palindrome_permutation(super::bool_array::is_palindrome_permutation);
    }


    #[test]
    fn it_tests_bit_vector() {
        test_palindrome_permutation(super::bit_vector::is_palindrome_permutation);
    }
}
