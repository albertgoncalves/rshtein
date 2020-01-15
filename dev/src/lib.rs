#![feature(test)]

use arrayvec::ArrayVec;
use unchecked_index::{unchecked_index, UncheckedIndex};

const CAPACITY: usize = 4096;

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
    for i in 1..height {
        for j in 1..width {
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
pub fn lev_1d_vec(a: &str, b: &str) -> usize {
    let a: &[u8] = a.as_bytes();
    let b: &[u8] = b.as_bytes();
    let height: usize = a.len() + 1;
    let width: usize = b.len() + 1;
    let n: usize = height * width;
    let mut matrix: Vec<usize> = vec![0; n];
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
    for i in 1..height {
        for j in 1..width {
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
    matrix[n - 1]
}

#[must_use]
#[allow(clippy::needless_range_loop)]
pub fn lev_1d_arrayvec(a: &str, b: &str) -> usize {
    let a: &[u8] = a.as_bytes();
    let b: &[u8] = b.as_bytes();
    let height: usize = a.len() + 1;
    let width: usize = b.len() + 1;
    let n: usize = height * width;
    let mut matrix: ArrayVec<[usize; CAPACITY]> = ArrayVec::new();
    macro_rules! select {
        ($j:expr, $i:expr $(,)?) => {
            $j + ($i * width)
        };
    }
    matrix.push(0);
    for j in 1..width {
        matrix.push(j);
    }
    for i in 1..height {
        matrix.push(i);
        for j in 1..width {
            let penalty: usize = {
                if a[i - 1] == b[j - 1] {
                    0
                } else {
                    1
                }
            };
            let value: usize = (matrix[select!(j, i - 1)] + 1)
                .min(matrix[select!(j - 1, i)] + 1)
                .min(matrix[select!(j - 1, i - 1)] + penalty);
            matrix.push(value);
        }
    }
    matrix[n - 1]
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
    let mut matrix: UncheckedIndex<ArrayVec<[usize; CAPACITY]>> =
        unchecked_index(ArrayVec::new());
    macro_rules! select {
        ($j:expr, $i:expr $(,)?) => {
            $j + ($i * width)
        };
    }
    matrix.push_unchecked(0);
    for j in 1..width {
        matrix.push_unchecked(j);
    }
    for i in 1..height {
        matrix.push_unchecked(i);
        for j in 1..width {
            let penalty: usize = {
                if a[i - 1] == b[j - 1] {
                    0
                } else {
                    1
                }
            };
            let value: usize = (matrix.get_unchecked(select!(j, i - 1)) + 1)
                .min(matrix.get_unchecked(select!(j - 1, i)) + 1)
                .min(matrix.get_unchecked(select!(j - 1, i - 1)) + penalty);
            /* NOTE: `push_unchecked` here is *slower*. Why? How? */
            matrix.push(value);
        }
    }
    matrix[n - 1]
}

#[must_use]
#[allow(clippy::needless_range_loop)]
pub fn lev_1d_array(a: &str, b: &str) -> usize {
    let a: &[u8] = a.as_bytes();
    let b: &[u8] = b.as_bytes();
    let height: usize = a.len() + 1;
    let width: usize = b.len() + 1;
    let n: usize = height * width;
    let mut matrix: [usize; CAPACITY] = [0; CAPACITY];
    macro_rules! select {
        ($j:expr, $i:expr $(,)?) => {
            $j + ($i * width)
        };
    }
    for j in 1..width {
        matrix[j] = j;
    }
    for i in 1..height {
        matrix[i * width] = i;
        for j in 1..width {
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
    matrix[n - 1]
}

#[must_use]
#[allow(clippy::missing_safety_doc, clippy::needless_range_loop)]
pub unsafe fn lev_1d_array_unsafe(a: &str, b: &str) -> usize {
    let a: &[u8] = a.as_bytes();
    let b: &[u8] = b.as_bytes();
    let height: usize = a.len() + 1;
    let width: usize = b.len() + 1;
    let n: usize = height * width;
    assert!(n < CAPACITY);
    let mut matrix: UncheckedIndex<[usize; CAPACITY]> =
        unchecked_index([0; CAPACITY]);
    macro_rules! select {
        ($j:expr, $i:expr $(,)?) => {
            $j + ($i * width)
        };
    }
    for j in 1..width {
        matrix[j] = j;
    }
    for i in 1..height {
        matrix[i * width] = i;
        for j in 1..width {
            let penalty: usize = {
                if a[i - 1] == b[j - 1] {
                    0
                } else {
                    1
                }
            };
            matrix[select!(j, i)] = {
                (matrix.get_unchecked(select!(j, i - 1)) + 1)
                    .min(matrix.get_unchecked(select!(j - 1, i)) + 1)
                    .min(matrix.get_unchecked(select!(j - 1, i - 1)) + penalty)
            };
        }
    }
    matrix[n - 1]
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

    macro_rules! bench_case_short {
        ($b:expr, $fn:expr $(,)?) => {
            $b.iter(|| $fn("sitting", "kitten"))
        };
    }

    macro_rules! bench_case_long {
        ($b:expr, $fn:expr $(,)?) => {
            $b.iter(|| {
                $fn(
                    "the quick brown fox jumps over the lazy dog",
                    "pack my box with five dozen liquor jugs",
                )
            })
        };
    }

    macro_rules! test_and_bench {
        (
            $fn:expr,
            $test:ident,
            $bench_short:ident,
            $bench_long:ident $(,)?
        ) => {
            #[test]
            fn $test() {
                test_cases!($fn)
            }

            #[bench]
            fn $bench_short(b: &mut Bencher) {
                bench_case_short!(b, $fn)
            }

            #[bench]
            fn $bench_long(b: &mut Bencher) {
                bench_case_long!(b, $fn)
            }
        };
    }

    macro_rules! test_and_bench_unsafe {
        (
            $fn:expr,
            $test:ident,
            $bench_short:ident,
            $bench_long:ident $(,)?
        ) => {
            #[test]
            fn $test() {
                unsafe { test_cases!($fn) }
            }

            #[bench]
            fn $bench_short(b: &mut Bencher) {
                unsafe { bench_case_short!(b, $fn) }
            }

            #[bench]
            fn $bench_long(b: &mut Bencher) {
                unsafe { bench_case_long!(b, $fn) }
            }
        };
    }

    /* NOTE: `lev_recursive` is too slow for the long benchmark. */
    #[test]
    fn test_lev_recursive() {
        test_cases!(lev_recursive)
    }

    test_and_bench!(
        lev_2d_vec,
        test_lev_2d_vec,
        bench_lev_2d_vec_short,
        bench_lev_2d_vec_long,
    );
    test_and_bench!(
        lev_1d_vec,
        test_lev_1d_vec,
        bench_lev_1d_vec_short,
        bench_lev_1d_vec_long,
    );
    test_and_bench!(
        lev_1d_arrayvec,
        test_lev_1d_arrayvec,
        bench_lev_1d_arrayvec_short,
        bench_lev_1d_arrayvec_long,
    );
    test_and_bench_unsafe!(
        lev_1d_arrayvec_unsafe,
        test_lev_1d_arrayvec_unsafe,
        bench_lev_1d_arrayvec_unsafe_short,
        bench_lev_1d_arrayvec_unsafe_long,
    );
    test_and_bench!(
        lev_1d_array,
        test_lev_1d_array,
        bench_lev_1d_array_short,
        bench_lev_1d_array_long,
    );
    test_and_bench_unsafe!(
        lev_1d_array_unsafe,
        test_lev_1d_array_unsafe,
        bench_lev_1d_array_unsafe_short,
        bench_lev_1d_array_unsafe_long,
    );
}
