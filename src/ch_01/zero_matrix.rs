/// # 1.8 Zero Matrix

mod optimizing_col_iteration_1 {
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
            // `m[row] = vec![0; m[row].len()]` doesn't perform better
            // according to benchmarks on matrices 300x300
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

mod optimizing_col_iteration_2 {
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
            m[row] = vec![0; m[row].len()];
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

#[cfg(test)]
mod tests {
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
    fn check_optimizing_col_iteration_1() {
        check_set_zeros_3_3(super::optimizing_col_iteration_1::set_zeros);
        check_set_zeros_2_3(super::optimizing_col_iteration_1::set_zeros);
        check_set_zeros_2_2(super::optimizing_col_iteration_1::set_zeros);
        check_set_zeros_1_1_zero(super::optimizing_col_iteration_1::set_zeros);
        check_set_zeros_1_1_one(super::optimizing_col_iteration_1::set_zeros);
    }

    #[test]
    fn check_optimizing_col_iteration_2() {
        check_set_zeros_3_3(super::optimizing_col_iteration_2::set_zeros);
        check_set_zeros_2_3(super::optimizing_col_iteration_2::set_zeros);
        check_set_zeros_2_2(super::optimizing_col_iteration_2::set_zeros);
        check_set_zeros_1_1_zero(super::optimizing_col_iteration_2::set_zeros);
        check_set_zeros_1_1_one(super::optimizing_col_iteration_2::set_zeros);
    }
 
}

#[cfg(test)]
mod benchmark_tests {
    use test::Bencher;

    static MAX_SIZE: usize = 300;

    #[bench]
    fn bench_optimizing_col_iteration_1(b: &mut Bencher) {
        b.iter(|| {
            let mut matrices: Vec<Vec<Vec<u8>>> = build_matrices(MAX_SIZE);

            for i in 0..matrices.len() {
                super::optimizing_col_iteration_1::set_zeros(&mut matrices[i])
            }

        });
    }

    #[bench]
    fn bench_optimizing_col_iteration_2(b: &mut Bencher) {
        b.iter(|| {
            let mut matrices: Vec<Vec<Vec<u8>>> = build_matrices(MAX_SIZE);

            for i in 0..matrices.len() {
                super::optimizing_col_iteration_2::set_zeros(&mut matrices[i])
            }

        });
    }

    fn build_matrices(count: usize) -> Vec<Vec<Vec<u8>>> {
        let mut result: Vec<Vec<Vec<u8>>> = vec![];

        for n in 0..count {
            result.push(vec![vec![0; n + 1]; n + 1]);

            for i in 0..n + 1 {
                for j in 0..n + 1 {
                    result[n][i][j] = ((n + i + j) % 3) as u8;
                }
            }
        }

        result
    }
}
