/// # 1.7 Rotate Matrix

pub fn rotate_matrix(m: &mut Vec<Vec<u16>>) -> bool {
    if m.len() == 0 || m.iter().any(|row| row.len() != m.len()) {
        return false
    }

    let n = m.len();

    for layer in 0..(n / 2) {
        let first = layer;
        let last = n - 1 - layer;
        for i in first..last {
            let offset = i - first;

            let top = m[first][i]; // Save top

            // left -> top
            m[first][i] = m[last - offset][first];

            // bottom -> left
            m[last - offset][first] = m[last][last - offset];

            // right -> bottom
            m[last][last - offset] = m[i][last];

            // top -> right
            m[i][last] = top;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    #[test]
    fn check_rotate_matrix_1_1() {
        let mut actual = vec![
            vec![1]
        ];

        let expected = vec![
            vec![1]
        ];

        super::rotate_matrix(&mut actual);

        assert_eq!(expected, actual)
    }

    #[test]
    fn check_rotate_matrix_2_2() {
        let mut actual = vec![
            vec![1, 2],
            vec![3, 4]
        ];

        let expected = vec![
            vec![3, 1],
            vec![4, 2]
        ];

        super::rotate_matrix(&mut actual);

        assert_eq!(expected, actual)
    }

    #[test]
    fn check_rotate_matrix_3_3() {
        let mut actual = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9]
        ];

        let expected = vec![
            vec![7, 4, 1],
            vec![8, 5, 2],
            vec![9, 6, 3]
        ];

        super::rotate_matrix(&mut actual);

        assert_eq!(expected, actual)
    }

    #[test]
    fn check_rotate_matrix_4_4() {
        let mut actual = vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12],
            vec![13, 14, 15, 16]
        ];

        let expected = vec![
            vec![13, 9, 5, 1],
            vec![14, 10, 6, 2],
            vec![15, 11, 7, 3],
            vec![16, 12, 8, 4]
        ];

        super::rotate_matrix(&mut actual);

        assert_eq!(expected, actual)
    }
}
