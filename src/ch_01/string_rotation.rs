/// # 1.9 String Rotation

pub fn is_rotation(s1: &str, s2: &str) -> bool {
    if s1 == s2 { return false }
    if s1.len() < 2 { return false }
    if s1.len() != s2.len() { return false }

    let concatenation = format!("{}{}", s2, s2);

    concatenation.contains(s1)
}

#[cfg(test)]
mod tests {
    #[test]
    fn check_is_rotation() {
        assert!(super::is_rotation("waterbottle", "erbottlewat"));
        assert!(super::is_rotation("xy", "yx"));

        assert!(!super::is_rotation("", ""));
        assert!(!super::is_rotation("a", "a"));
        assert!(!super::is_rotation("blah", "blah"));
        assert!(!super::is_rotation("blah", "hello"));
    }
}
