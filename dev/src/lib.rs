#![feature(test)]

/* NOTE: _.as_bytes() is fine as long as the given strings only contain
 * symbols between U+0000 and U+007F.
 */

#[must_use]
pub fn lev_rec(a: &[u8], b: &[u8], i: usize, j: usize) -> usize {
    if i == 0 {
        j
    } else if j == 0 {
        i
    } else {
        let penalty: usize = {
            if a[i - 1] == b[j - 1] {
                0
            } else {
                1
            }
        };
        (lev_rec(a, b, i - 1, j) + 1)
            .min(lev_rec(a, b, i, j - 1) + 1)
            .min(lev_rec(a, b, i - 1, j - 1) + penalty)
    }
}

#[must_use]
pub fn lev_recursive(a: &[u8], b: &[u8]) -> usize {
    lev_rec(a, b, a.len(), b.len())
}

#[must_use]
#[allow(clippy::needless_range_loop)]
pub fn lev_2d_vec(a: &[u8], b: &[u8]) -> usize {
    let height: usize = a.len() + 1;
    let width: usize = b.len() + 1;
    let mut matrix: Vec<Vec<usize>> = vec![vec![0; width]; height];
    for i in 1..height {
        matrix[i][0] = i;
    }
    for j in 1..width {
        matrix[0][j] = j;
    }
    for j in 1..width {
        for i in 1..height {
            let penalty: usize = {
                if a[i - 1] == b[j - 1] {
                    0
                } else {
                    1
                }
            };
            matrix[i][j] = (matrix[i - 1][j] + 1)
                .min(matrix[i][j - 1] + 1)
                .min(matrix[i - 1][j - 1] + penalty);
        }
    }
    matrix[height - 1][width - 1]
}

#[must_use]
#[allow(clippy::needless_range_loop)]
pub fn lev_1d_vec(a: &[u8], b: &[u8]) -> usize {
    let height: usize = a.len() + 1;
    let width: usize = b.len() + 1;
    let mut matrix: Vec<usize> = vec![0; height * width];
    macro_rules! select {
        ($j:expr, $i:expr $(,)?) => {
            $j + ($i * width)
        };
    }
    for i in 1..height {
        matrix[i * width] = i;
    }
    for j in 1..width {
        matrix[j] = j;
    }
    for j in 1..width {
        for i in 1..height {
            let penalty: usize = {
                if a[i - 1] == b[j - 1] {
                    0
                } else {
                    1
                }
            };
            matrix[select!(j, i)] = (matrix[select!(j, i - 1)] + 1)
                .min(matrix[select!(j - 1, i)] + 1)
                .min(matrix[select!(j - 1, i - 1)] + penalty);
        }
    }
    *matrix.last().unwrap()
}

#[cfg(test)]
mod tests {
    extern crate test;

    use super::*;
    use test::Bencher;

    macro_rules! test_cases {
        ($f:expr $(,)?) => {{
            assert_eq!($f("sitting".as_bytes(), "kitten".as_bytes()), 3);
            assert_eq!($f("flaw".as_bytes(), "lawn".as_bytes()), 2);
            assert_eq!($f("saturday".as_bytes(), "sunday".as_bytes()), 3);
            assert_eq!($f("gumbo".as_bytes(), "gambol".as_bytes()), 2)
        }};
    }

    macro_rules! bench_case {
        ($b:expr, $f:expr $(,)?) => {
            $b.iter(|| $f("sitting".as_bytes(), "kitten".as_bytes()))
        };
    }

    #[test]
    fn test_lev_recursive() {
        test_cases!(lev_recursive)
    }

    #[bench]
    fn bench_lev_recursive(b: &mut Bencher) {
        bench_case!(b, lev_recursive)
    }

    #[test]
    fn test_lev_2d_vec() {
        test_cases!(lev_2d_vec)
    }

    #[bench]
    fn bench_lev_2d_vec(b: &mut Bencher) {
        bench_case!(b, lev_2d_vec)
    }

    #[test]
    fn test_lev_1d_vec() {
        test_cases!(lev_1d_vec)
    }

    #[bench]
    fn bench_lev_1d_vec(b: &mut Bencher) {
        bench_case!(b, lev_1d_vec)
    }
}
