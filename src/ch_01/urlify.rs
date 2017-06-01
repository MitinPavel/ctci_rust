/// # 1.3 URLify


pub fn urlify(mut chars: &mut [char], true_length: usize) -> &[char] {
    let mut space_count: usize = 0;

    for i in 0..true_length {
        if chars[i] == ' ' {
            space_count += 1;
        }
    }

    let mut index = true_length + space_count * 2;

    for i in (0..true_length).rev() {
        if chars[i] == ' ' {
            chars[index - 1] = '0';
            chars[index - 2] = '2';
            chars[index - 3] = '%';
            index -= 3;
        } else {
            chars[index - 1] = chars[i];
            index -= 1;
        }
    }

    chars
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_checks_urlify() {
        let mut cs1: [char; 5] = ['a', ' ', 'b', ' ', ' '];
        assert_eq!(&['a', '%', '2', '0', 'b'], super::urlify(&mut cs1, 3));

        let mut cs2: [char; 2] = ['a', 'a'];
        assert_eq!(&['a', 'a'], super::urlify(&mut cs2, 2));

        let mut cs3: [char; 4] = ['a', ' ', ' ', ' '];
        assert_eq!(&['a', '%', '2', '0'], super::urlify(&mut cs3, 2));

        let mut cs4: [char; 4] = ['a', ' ', ' ', ' '];
        assert_eq!(&['a', ' ', ' ', ' '], super::urlify(&mut cs4, 1));
    }
}
