/// # 1.8 Zero Matrix

mod optimizing_col_iteration {
    pub fn set_zeros(m: &mut Vec<Vec<u8>>) {
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
}

// The solution was rigorously translated
// from the book second solution (as close as possible to the original pseudocode).
mod first_col_and_row_optimization {
    pub fn set_zeros(m: &mut Vec<Vec<u8>>) {
        let mut first_row_has_zero = false;
        let mut first_col_has_zero = false;

        // Check if the first row has a zero.
        for j in 0..m[0].len() {
            if m[0][j] == 0 {
                first_row_has_zero = true;
                break
            }
        }

        // Check if the first column has a zero
        for i in 0..m.len() {
            if m[i][0] == 0 {
                first_col_has_zero = true;
                break
            }
        }

        // Check for zeros in the rest of the matrix
        for i in 1..m.len() {
            for j in 1..m[0].len() {
                if m[i][j] == 0 {
                    m[i][0] = 0;
                    m[0][j] = 0;
                }
            }
        }

        // Nullify rows based on values in first column
        for i in 1..m.len() {
            if m[i][0] == 0 {
                nullify_row(m, i)
            }
        }

        // Nullify columns based on values in first row
        for j in 1..m[0].len() {
            if m[0][j] == 0 {
                nullify_column(m, j);
            }
        }

        // Nullify the first row
        if first_row_has_zero {
            nullify_row(m, 0)
        }

        // Nullify the first column
        if first_col_has_zero {
            nullify_column(m, 0);
        }
    }

    fn nullify_row(matrix: &mut Vec<Vec<u8>>, row: usize) {
        for j in 0..matrix[0].len() {
            matrix[row][j] = 0
        }
    }

    fn nullify_column(matrix: &mut Vec<Vec<u8>>, col: usize) {
        for i in 0..matrix.len() {
            matrix[i][col] = 0
        }
    }
}

#[cfg(test)]
mod test {
    fn check_set_zeros_3_3<F>(test: F) where F: Fn(&mut Vec<Vec<u8>>) -> () {
        let mut actual = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9]
        ];
        test(&mut actual);
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
        test(&mut actual);
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
        test(&mut actual);
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
        test(&mut actual);
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
        test(&mut actual);
        assert_eq!(vec![
            vec![1, 2, 0],
            vec![0, 0, 0],
            vec![7, 8, 0]
        ], actual);
    }

    fn check_set_zeros_2_3<F>(test: F) where F: Fn(&mut Vec<Vec<u8>>) -> () {
        let mut actual = vec![
            vec![1, 2, 3],
            vec![4, 0, 6]
        ];
        test(&mut actual);
        assert_eq!(vec![
            vec![1, 0, 3],
            vec![0, 0, 0]
        ], actual);
    }

    fn check_set_zeros_2_2<F>(test: F) where F: Fn(&mut Vec<Vec<u8>>) -> () {
        let mut actual = vec![
            vec![1, 0],
            vec![1, 1]
        ];
        test(&mut actual);
        assert_eq!(vec![
            vec![0, 0],
            vec![1, 0]
        ], actual)
    }

    fn check_set_zeros_1_1_one<F>(test: F) where F: Fn(&mut Vec<Vec<u8>>) -> () {
        let mut actual = vec![
            vec![1]
        ];

        let expected = vec![
            vec![1]
        ];

        test(&mut actual);

        assert_eq!(expected, actual)
    }

    fn check_set_zeros_1_1_zero<F>(test: F) where F: Fn(&mut Vec<Vec<u8>>) -> () {
        let mut actual = vec![
            vec![0]
        ];

        let expected = vec![
            vec![0]
        ];

        test(&mut actual);

        assert_eq!(expected, actual)
    }

    #[test]
    fn check_optimizing_col_iteration() {
        check_set_zeros_3_3(super::optimizing_col_iteration::set_zeros);
        check_set_zeros_2_3(super::optimizing_col_iteration::set_zeros);
        check_set_zeros_2_2(super::optimizing_col_iteration::set_zeros);
        check_set_zeros_1_1_zero(super::optimizing_col_iteration::set_zeros);
        check_set_zeros_1_1_one(super::optimizing_col_iteration::set_zeros);
    }

    #[test]
    fn check_first_col_and_row_optimization() {
        check_set_zeros_3_3(super::first_col_and_row_optimization::set_zeros);
        check_set_zeros_2_3(super::first_col_and_row_optimization::set_zeros);
        check_set_zeros_2_2(super::first_col_and_row_optimization::set_zeros);
        check_set_zeros_1_1_zero(super::first_col_and_row_optimization::set_zeros);
        check_set_zeros_1_1_one(super::first_col_and_row_optimization::set_zeros);
    }
}
