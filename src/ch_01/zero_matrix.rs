/// # 1.8 Zero Matrix

pub fn zero_matrix(m: &mut Vec<Vec<u8>>) {
    let mut untouched_cols: Vec<usize> = (0..m[0].len()).collect();
    let mut untouched_rows: Vec<usize> = vec![];
    let mut zero_rows: Vec<usize> = vec![];

    for row in 0..m.len() {
        let mut zero_row = false;

        untouched_cols.retain(|col| {
            if m[row][*col] == 0 {
                zero_row = true;
                false
            } else {
                true
            }
        });

        if zero_row {
            zero_rows.push(row)
        } else {
            untouched_rows.push(row)
        }
    }

    for row in zero_rows {
        //OPTIMIZE: Check if `m[row] = vec![0; m[row].len()]` performs better.
        for col in 0..m[row].len() {
            m[row][col] = 0;
        }
    }

    for row in untouched_rows {
        for col in 0..m[row].len() {
            if !untouched_cols.contains(&col) {
                m[row][col] = 0;
            }
        }
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn check_zero_matrix_3_3() {
        let mut actual = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9]
        ];
        super::zero_matrix(&mut actual);
        assert_eq!(vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9]
        ], actual);

        let mut actual = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![0, 8, 9]
        ];
        super::zero_matrix(&mut actual);
        assert_eq!(vec![
            vec![0, 2, 3],
            vec![0, 5, 6],
            vec![0, 0, 0]
        ], actual);

        actual = vec![
            vec![1, 2, 3],
            vec![4, 0, 6],
            vec![7, 8, 9]
        ];
        super::zero_matrix(&mut actual);
        assert_eq!(vec![
            vec![1, 0, 3],
            vec![0, 0, 0],
            vec![7, 0, 9]
        ], actual);

        actual = vec![
            vec![1, 2, 0],
            vec![4, 5, 6],
            vec![7, 8, 9]
        ];
        super::zero_matrix(&mut actual);
        assert_eq!(vec![
            vec![0, 0, 0],
            vec![4, 5, 0],
            vec![7, 8, 0]
        ], actual);

        actual = vec![
            vec![1, 2, 3],
            vec![4, 5, 0],
            vec![7, 8, 9]
        ];
        super::zero_matrix(&mut actual);
        assert_eq!(vec![
            vec![1, 2, 0],
            vec![0, 0, 0],
            vec![7, 8, 0]
        ], actual);
    }

    #[test]
    fn check_zero_matrix_2_3() {
        let mut actual = vec![
            vec![1, 2, 3],
            vec![4, 0, 6]
        ];
        super::zero_matrix(&mut actual);
        assert_eq!(vec![
            vec![1, 0, 3],
            vec![0, 0, 0]
        ], actual);
    }

    #[test]
    fn check_zero_matrix_2_2() {
        let mut actual = vec![
            vec![1, 0],
            vec![1, 1]
        ];
        super::zero_matrix(&mut actual);
        assert_eq!(vec![
            vec![0, 0],
            vec![1, 0]
        ], actual)
    }

    #[test]
    fn check_zero_matrix_1_1_one() {
        let mut actual = vec![
            vec![1]
        ];

        let expected = vec![
            vec![1]
        ];

        super::zero_matrix(&mut actual);

        assert_eq!(expected, actual)
    }
}
