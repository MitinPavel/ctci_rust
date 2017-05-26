/// # Is Unique
///
/// ## Assumptions:
///
/// * ASCII symbols only

/// ## Using an additional data structure: HashSet.
mod using_set {
    use std::collections::HashSet;

    pub fn is_unique(string: String) -> bool {
        if string.len() > 128 { return false };

        let mut seen_chars: HashSet<char> = HashSet::new();

        !string.chars().any(|c| !seen_chars.insert(c))
    }
}

/// ## Mutating the input string.
mod mutable_str {
    use std::mem;

    pub fn is_unique(string: &mut str) -> bool {
        if string.len() > 128 { return false };

        let mut bytes: &mut [u8] = unsafe { mem::transmute(string) };
        bytes.sort();

        !bytes.windows(2).any(|pair| pair[0] == pair[1])
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_checks_using_set() {
        assert_eq!(true,  super::using_set::is_unique("".to_string()));
        assert_eq!(true,  super::using_set::is_unique("a".to_string()));
        assert_eq!(true,  super::using_set::is_unique("ab".to_string()));
        assert_eq!(true,  super::using_set::is_unique("abc".to_string()));
        assert_eq!(false, super::using_set::is_unique("aba".to_string()));
        assert_eq!(false, super::using_set::is_unique("aab".to_string()));
        assert_eq!(false, super::using_set::is_unique("baa".to_string()));
        assert_eq!(false, super::using_set::is_unique("abab".to_string()));
    }

    #[test]
    fn it_checks_mutable_str() {
        assert_eq!(true,  super::mutable_str::is_unique("".to_string().as_mut_str()));
        assert_eq!(true,  super::mutable_str::is_unique("a".to_string().as_mut_str()));
        assert_eq!(true,  super::mutable_str::is_unique("ab".to_string().as_mut_str()));
        assert_eq!(true,  super::mutable_str::is_unique("abc".to_string().as_mut_str()));
        assert_eq!(false, super::mutable_str::is_unique("aba".to_string().as_mut_str()));
        assert_eq!(false, super::mutable_str::is_unique("aab".to_string().as_mut_str()));
        assert_eq!(false, super::mutable_str::is_unique("baa".to_string().as_mut_str()));
        assert_eq!(false, super::mutable_str::is_unique("abab".to_string().as_mut_str()));
    }
}
