/// # URLify

mod char_to_shift {
    use std::collections::HashMap;

    pub fn urlify(mut chars: &mut [char], len: usize) -> &[char] {
        let mut shifts: HashMap<usize, usize> = HashMap::new();
        let mut shift = 0;

        for i in 0..len {
            shifts.insert(i, shift);
            if chars[i] == ' ' {
                shift += 2; // `%20` needs 2 extra places.
            }
        }

        for i in (0..len).rev() {
            chars[i + shifts[&i]] = chars[i];
            if chars[i] == ' ' {
                chars[i] = '%';
                chars[i + 1] = '2';
                chars[i + 2] = '0';
            }
        }

        chars
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_checks_char_to_shift_01() {
        let mut cs1: [char; 5] = ['a', ' ', 'b', ' ', ' '];
        assert_eq!(&['a', '%', '2', '0', 'b'], super::char_to_shift::urlify(&mut cs1, 3));

        let mut cs2: [char; 2] = ['a', 'a'];
        assert_eq!(&['a', 'a'], super::char_to_shift::urlify(&mut cs2, 2));

        let mut cs3: [char; 4] = ['a', ' ', ' ', ' '];
        assert_eq!(&['a', '%', '2', '0'], super::char_to_shift::urlify(&mut cs3, 2));
    }
}
