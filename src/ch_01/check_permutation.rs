/// # Check Permutations

// TODO
// * [Rust] Enhance/extend `String` struct

// Voluntary decisions:
// * [Rust] Use `String` (instead of `&str`)
// * Return `true` for empty strings if size is equal (treating whitespace chars as regular chars)

/// ## Comparing sorted characters
mod using_sorting {
    pub fn is_permutations(s1: String, s2: String) -> bool {
        if s1.len() != s2.len() { return false }

        let mut v1: Vec<char> = s1.chars().collect();
        v1.sort();

        let mut v2: Vec<char> = s2.chars().collect();
        v2.sort();

        v1 == v2
    }
}

/// ## Building "char" counter
/// Assumption: ASCII charset
mod char_counter {
    pub fn is_permutations(s1: String, s2: String) -> bool {
        if s1.len() != s2.len() { return false }

        let mut char_counts: [i32; 128] = [0; 128];

        for c in s1.chars() {
            char_counts[c as usize] += 1;
        }

        for c in s2.chars() {
            let i = c as usize;
            char_counts[i] -= 1;
            if char_counts[i] < 0 {
                return false;
            }
        }

        char_counts.into_iter().all(|x| *x == 0)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_checks_using_sorting() {
        assert_eq!(true, super::using_sorting::is_permutations("".to_string(), "".to_string()));
        assert_eq!(true, super::using_sorting::is_permutations(" ".to_string(), " ".to_string()));
        assert_eq!(true, super::using_sorting::is_permutations("a".to_string(), "a".to_string()));
        assert_eq!(true, super::using_sorting::is_permutations("ab".to_string(), "ab".to_string()));
        assert_eq!(true, super::using_sorting::is_permutations("ab".to_string(), "ba".to_string()));
        assert_eq!(false, super::using_sorting::is_permutations("a".to_string(), "b".to_string()));
        assert_eq!(false, super::using_sorting::is_permutations("ab".to_string(), "bc".to_string()));
        assert_eq!(false, super::using_sorting::is_permutations("a".to_string(), "ab".to_string()));
        assert_eq!(false, super::using_sorting::is_permutations(" ".to_string(), "  ".to_string()));
    }

    #[test]
    fn it_checks_char_counter() {
        assert_eq!(true, super::char_counter::is_permutations("".to_string(), "".to_string()));
        assert_eq!(true, super::char_counter::is_permutations(" ".to_string(), " ".to_string()));
        assert_eq!(true, super::char_counter::is_permutations("a".to_string(), "a".to_string()));
        assert_eq!(true, super::char_counter::is_permutations("ab".to_string(), "ab".to_string()));
        assert_eq!(true, super::char_counter::is_permutations("ab".to_string(), "ba".to_string()));
        assert_eq!(false, super::char_counter::is_permutations("a".to_string(), "b".to_string()));
        assert_eq!(false, super::char_counter::is_permutations("ab".to_string(), "bc".to_string()));
        assert_eq!(false, super::char_counter::is_permutations("a".to_string(), "ab".to_string()));
        assert_eq!(false, super::char_counter::is_permutations(" ".to_string(), "  ".to_string()));
    }
}
