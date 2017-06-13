/// # 1.6 String Compression

// Assumptions
// * Only uppercase and lowercase letters (a-z)
// * There can not be more than 255 (u8 max value) repetitions of a char in a string

fn compress(string: String) -> String {
    let mut previous = '0'; // Assuming input cannot contain '0'.
    let mut count: u8 = 0;
    let mut result = String::new();

    for c in string.chars() {
        if c == previous {
            count += 1;
        } else {
            if count > 0 {
              result.push_str(&format!("{}", count));
            }
            result.push(c);
            previous = c;
            count = 1;
        }
    }

    if count > 0 {
        result.push_str(&format!("{}", count));
    }

    if result.len() < string.len() {
        result
    } else {
        string
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_tests_compress() {
        assert_eq!("a2b1c5a3".to_string(), super::compress("aabcccccaaa".to_string()));

        assert_eq!("x".to_string(), super::compress("x".to_string()));
        assert_eq!("ab".to_string(), super::compress("ab".to_string()));
        assert_eq!("aa".to_string(), super::compress("aa".to_string()));
        assert_eq!("abc".to_string(), super::compress("abc".to_string()));
        assert_eq!("blah".to_string(), super::compress("blah".to_string()));

        assert_eq!("D3".to_string(), super::compress("DDD".to_string()));
        assert_eq!("DDd".to_string(), super::compress("DDd".to_string()));
        assert_eq!("DdD".to_string(), super::compress("DdD".to_string()));
        assert_eq!("Ddd".to_string(), super::compress("Ddd".to_string()));
        assert_eq!("dDD".to_string(), super::compress("dDD".to_string()));
        assert_eq!("dDd".to_string(), super::compress("dDd".to_string()));
        assert_eq!("ddD".to_string(), super::compress("ddD".to_string()));
        assert_eq!("d3".to_string(), super::compress("ddd".to_string()));

        assert_eq!("y4".to_string(), super::compress("yyyy".to_string()));
        assert_eq!("yyyz".to_string(), super::compress("yyyz".to_string()));
        assert_eq!("yyzy".to_string(), super::compress("yyzy".to_string()));
        assert_eq!("yyzz".to_string(), super::compress("yyzz".to_string()));
        assert_eq!("yzyy".to_string(), super::compress("yzyy".to_string()));
        assert_eq!("yzyz".to_string(), super::compress("yzyz".to_string()));
        assert_eq!("yzzy".to_string(), super::compress("yzzy".to_string()));
        assert_eq!("yzzz".to_string(), super::compress("yzzz".to_string()));
        assert_eq!("zyyy".to_string(), super::compress("zyyy".to_string()));
        assert_eq!("zyyz".to_string(), super::compress("zyyz".to_string()));
        assert_eq!("zyzy".to_string(), super::compress("zyzy".to_string()));
        assert_eq!("zyzz".to_string(), super::compress("zyzz".to_string()));
        assert_eq!("zzyy".to_string(), super::compress("zzyy".to_string()));
        assert_eq!("zzyz".to_string(), super::compress("zzyz".to_string()));
        assert_eq!("zzzy".to_string(), super::compress("zzzy".to_string()));
        assert_eq!("z4".to_string(), super::compress("zzzz".to_string()));

        assert_eq!("y5".to_string(), super::compress("yyyyy".to_string()));
        assert_eq!("y4z1".to_string(), super::compress("yyyyz".to_string()));
        assert_eq!("yyyzy".to_string(), super::compress("yyyzy".to_string()));
        assert_eq!("y3z2".to_string(), super::compress("yyyzz".to_string()));
        assert_eq!("yyzyy".to_string(), super::compress("yyzyy".to_string()));
        assert_eq!("yyzyz".to_string(), super::compress("yyzyz".to_string()));
        assert_eq!("yyzzy".to_string(), super::compress("yyzzy".to_string()));
        assert_eq!("y2z3".to_string(), super::compress("yyzzz".to_string()));
        assert_eq!("yzyyy".to_string(), super::compress("yzyyy".to_string()));
        assert_eq!("yzyyz".to_string(), super::compress("yzyyz".to_string()));
        assert_eq!("yzyzy".to_string(), super::compress("yzyzy".to_string()));
        assert_eq!("yzyzz".to_string(), super::compress("yzyzz".to_string()));
        assert_eq!("yzzyy".to_string(), super::compress("yzzyy".to_string()));
        assert_eq!("yzzyz".to_string(), super::compress("yzzyz".to_string()));
        assert_eq!("yzzzy".to_string(), super::compress("yzzzy".to_string()));
        assert_eq!("y1z4".to_string(), super::compress("yzzzz".to_string()));
        assert_eq!("z1y4".to_string(), super::compress("zyyyy".to_string()));
        assert_eq!("zyyyz".to_string(), super::compress("zyyyz".to_string()));
        assert_eq!("zyyzy".to_string(), super::compress("zyyzy".to_string()));
        assert_eq!("zyyzz".to_string(), super::compress("zyyzz".to_string()));
        assert_eq!("zyzyy".to_string(), super::compress("zyzyy".to_string()));
        assert_eq!("zyzyz".to_string(), super::compress("zyzyz".to_string()));
        assert_eq!("zyzzy".to_string(), super::compress("zyzzy".to_string()));
        assert_eq!("zyzzz".to_string(), super::compress("zyzzz".to_string()));
        assert_eq!("z2y3".to_string(), super::compress("zzyyy".to_string()));
        assert_eq!("zzyyz".to_string(), super::compress("zzyyz".to_string()));
        assert_eq!("zzyzy".to_string(), super::compress("zzyzy".to_string()));
        assert_eq!("zzyzz".to_string(), super::compress("zzyzz".to_string()));
        assert_eq!("z3y2".to_string(), super::compress("zzzyy".to_string()));
        assert_eq!("zzzyz".to_string(), super::compress("zzzyz".to_string()));
        assert_eq!("z4y1".to_string(), super::compress("zzzzy".to_string()));
        assert_eq!("z5".to_string(), super::compress("zzzzz".to_string()));
    }
}
