#![feature(test)]

use arrayvec::ArrayVec;

/* NOTE: _.as_bytes() is fine as long as the given strings only contain symbols
 * between U+0000 and U+007F.
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
pub fn lev_recursive(a: &str, b: &str) -> usize {
    let a: &[u8] = a.as_bytes();
    let b: &[u8] = b.as_bytes();
    lev_rec(a, b, a.len(), b.len())
}

#[must_use]
#[allow(clippy::needless_range_loop)]
pub fn lev_2d_vec(a: &str, b: &str) -> usize {
    let a: &[u8] = a.as_bytes();
    let b: &[u8] = b.as_bytes();
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
    *matrix.last().unwrap().last().unwrap()
}

#[must_use]
#[allow(clippy::needless_range_loop)]
pub fn lev_1d_vec(a: &str, b: &str) -> usize {
    let a: &[u8] = a.as_bytes();
    let b: &[u8] = b.as_bytes();
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

const CAPACITY: usize = 512;

#[must_use]
#[allow(clippy::needless_range_loop)]
pub fn lev_1d_arrayvec(a: &str, b: &str) -> usize {
    let a: &[u8] = a.as_bytes();
    let b: &[u8] = b.as_bytes();
    let height: usize = a.len() + 1;
    let width: usize = b.len() + 1;
    let mut matrix: ArrayVec<[usize; CAPACITY]> = ArrayVec::new();
    for _ in 0..(height * width) {
        matrix.push(0);
    }
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

#[must_use]
#[allow(clippy::missing_safety_doc, clippy::needless_range_loop)]
pub unsafe fn lev_1d_arrayvec_unsafe(a: &str, b: &str) -> usize {
    let a: &[u8] = a.as_bytes();
    let b: &[u8] = b.as_bytes();
    let height: usize = a.len() + 1;
    let width: usize = b.len() + 1;
    let n: usize = height * width;
    assert!(n < CAPACITY);
    let mut matrix: ArrayVec<[usize; CAPACITY]> = ArrayVec::new();
    for _ in 0..n {
        matrix.push_unchecked(0);
    }
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
            matrix[select!(j, i)] = (matrix.get_unchecked(select!(j, i - 1))
                + 1)
            .min(matrix.get_unchecked(select!(j - 1, i)) + 1)
            .min(matrix.get_unchecked(select!(j - 1, i - 1)) + penalty);
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
        ($fn:expr $(,)?) => {{
            assert_eq!($fn("sitting", "kitten"), 3);
            assert_eq!($fn("flaw", "lawn"), 2);
            assert_eq!($fn("saturday", "sunday"), 3);
            assert_eq!($fn("gumbo", "gambol"), 2)
        }};
    }

    macro_rules! bench_case {
        ($b:expr, $fn:expr $(,)?) => {
            $b.iter(|| $fn("sitting", "kitten"))
        };
    }

    macro_rules! test_and_bench {
        ($fn:expr, $test:ident, $bench:ident $(,)?) => {
            #[test]
            fn $test() {
                test_cases!($fn)
            }

            #[bench]
            fn $bench(b: &mut Bencher) {
                bench_case!(b, $fn)
            }
        };
    }

    macro_rules! test_and_bench_unsafe {
        ($fn:expr, $test:ident, $bench:ident $(,)?) => {
            #[test]
            fn $test() {
                unsafe { test_cases!($fn) }
            }

            #[bench]
            fn $bench(b: &mut Bencher) {
                unsafe { bench_case!(b, $fn) }
            }
        };
    }

    test_and_bench!(lev_recursive, test_lev_recursive, bench_lev_recursive);
    test_and_bench!(lev_2d_vec, test_lev_2d_vec, bench_lev_2d_vec);
    test_and_bench!(lev_1d_vec, test_lev_1d_vec, bench_lev_1d_vec);
    test_and_bench!(
        lev_1d_arrayvec,
        test_lev_1d_arrayvec,
        bench_lev_1d_arrayvec,
    );
    test_and_bench_unsafe!(
        lev_1d_arrayvec_unsafe,
        test_lev_1d_arrayvec_unsafe,
        bench_lev_1d_arrayvec_unsafe,
    );
}
