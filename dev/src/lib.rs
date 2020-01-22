#![feature(test)]

use arrayvec::ArrayVec;
use unchecked_index::{unchecked_index, UncheckedIndex};

const CAP_1D: usize = 256;
const CAP_2D: usize = 4096;

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

/* NOTE: https://github.com/crystal-lang/crystal/blob/41bd18fbea4aec50aad33aa3beb7a0bf30544186/src/levenshtein.cr#L13
 */
#[must_use]
#[allow(clippy::needless_range_loop)]
pub fn lev_1d_vec_min(a: &str, b: &str) -> usize {
    let a: &[u8] = a.as_bytes();
    let b: &[u8] = b.as_bytes();
    let n_a: usize = a.len();
    let n_b: usize = b.len();
    if n_a == 0 {
        return n_b;
    } else if n_b == 0 {
        return n_a;
    }
    let (a, b, n_a, n_b, mut matrix): (
        &[u8],
        &[u8],
        usize,
        usize,
        Vec<usize>,
    ) = {
        if n_a < n_b {
            (a, b, n_a, n_b, vec![0; n_b + 1])
        } else {
            (b, a, n_b, n_a, vec![0; n_a + 1])
        }
    };
    for j in 0..n_b {
        matrix[j] = j;
    }
    for i in 0..n_a {
        let mut previous = i + 1;
        for j in 0..n_b {
            let penalty: usize = {
                if a[i] == b[j] {
                    0
                } else {
                    1
                }
            };
            let cost: usize = (previous + 1)
                .min(matrix[j + 1] + 1)
                .min(matrix[j] + penalty);
            matrix[j] = previous;
            previous = cost
        }
        matrix[n_b] = previous
    }
    matrix[n_b]
}

#[must_use]
#[allow(clippy::needless_range_loop)]
pub fn lev_1d_arrayvec(a: &str, b: &str) -> usize {
    let a: &[u8] = a.as_bytes();
    let b: &[u8] = b.as_bytes();
    let height: usize = a.len() + 1;
    let width: usize = b.len() + 1;
    let n: usize = height * width;
    let mut matrix: ArrayVec<[usize; CAP_2D]> = ArrayVec::new();
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
            let cost: usize = (matrix[select!(j, i - 1)] + 1)
                .min(matrix[select!(j - 1, i)] + 1)
                .min(matrix[select!(j - 1, i - 1)] + penalty);
            matrix.push(cost);
        }
    }
    matrix[n - 1]
}

#[must_use]
#[allow(clippy::needless_range_loop)]
pub fn lev_1d_arrayvec_min(a: &str, b: &str) -> usize {
    let a: &[u8] = a.as_bytes();
    let b: &[u8] = b.as_bytes();
    let n_a: usize = a.len();
    let n_b: usize = b.len();
    if n_a == 0 {
        return n_b;
    } else if n_b == 0 {
        return n_a;
    }
    let (a, b, n_a, n_b, mut matrix): (
        &[u8],
        &[u8],
        usize,
        usize,
        ArrayVec<[usize; CAP_1D]>,
    ) = {
        if n_a < n_b {
            (a, b, n_a, n_b, ArrayVec::new())
        } else {
            (b, a, n_b, n_a, ArrayVec::new())
        }
    };
    for j in 0..=n_b {
        matrix.push(j);
    }
    for i in 0..n_a {
        let mut previous = i + 1;
        for j in 0..n_b {
            let penalty: usize = {
                if a[i] == b[j] {
                    0
                } else {
                    1
                }
            };
            let cost: usize = (previous + 1)
                .min(matrix[j + 1] + 1)
                .min(matrix[j] + penalty);
            matrix[j] = previous;
            previous = cost
        }
        matrix[n_b] = previous
    }
    matrix[n_b]
}

#[must_use]
#[allow(clippy::missing_safety_doc, clippy::needless_range_loop)]
pub unsafe fn lev_1d_arrayvec_unsafe(a: &str, b: &str) -> usize {
    let a: &[u8] = a.as_bytes();
    let b: &[u8] = b.as_bytes();
    let height: usize = a.len() + 1;
    let width: usize = b.len() + 1;
    let n: usize = height * width;
    assert!(n < CAP_2D);
    let mut matrix: UncheckedIndex<ArrayVec<[usize; CAP_2D]>> =
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
            let cost: usize = (matrix.get_unchecked(select!(j, i - 1)) + 1)
                .min(matrix.get_unchecked(select!(j - 1, i)) + 1)
                .min(matrix.get_unchecked(select!(j - 1, i - 1)) + penalty);
            /* NOTE: This last `push_unchecked` has a *negative* performance
             * impact on `Darwin`. Is this a bug? On `Linux` this provides a
             * small but consistent speed increase.
             */
            matrix.push_unchecked(cost);
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
    let mut matrix: [usize; CAP_2D] = [0; CAP_2D];
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
#[allow(clippy::needless_range_loop)]
pub fn lev_1d_array_min(a: &str, b: &str) -> usize {
    let a: &[u8] = a.as_bytes();
    let b: &[u8] = b.as_bytes();
    let n_a: usize = a.len();
    let n_b: usize = b.len();
    if n_a == 0 {
        return n_b;
    } else if n_b == 0 {
        return n_a;
    }
    let (a, b, n_a, n_b, mut matrix): (
        &[u8],
        &[u8],
        usize,
        usize,
        [usize; CAP_1D],
    ) = {
        if n_a < n_b {
            (a, b, n_a, n_b, [0; CAP_1D])
        } else {
            (b, a, n_b, n_a, [0; CAP_1D])
        }
    };
    for j in 1..=n_b {
        matrix[j] = j;
    }
    for i in 0..n_a {
        let mut previous = i + 1;
        for j in 0..n_b {
            let penalty: usize = {
                if a[i] == b[j] {
                    0
                } else {
                    1
                }
            };
            let cost: usize = (previous + 1)
                .min(matrix[j + 1] + 1)
                .min(matrix[j] + penalty);
            matrix[j] = previous;
            previous = cost
        }
        matrix[n_b] = previous
    }
    matrix[n_b]
}

#[must_use]
#[allow(clippy::missing_safety_doc, clippy::needless_range_loop)]
pub unsafe fn lev_1d_array_unsafe(a: &str, b: &str) -> usize {
    let a: &[u8] = a.as_bytes();
    let b: &[u8] = b.as_bytes();
    let height: usize = a.len() + 1;
    let width: usize = b.len() + 1;
    let n: usize = height * width;
    assert!(n < CAP_2D);
    let mut matrix: UncheckedIndex<[usize; CAP_2D]> =
        unchecked_index([0; CAP_2D]);
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
mod test {
    extern crate test;

    use super::*;
    use test::Bencher;

    macro_rules! both_ways {
        ($fn:expr, $a:expr, $b:expr, $v:expr $(,)?) => {
            assert_eq!($fn($a, $b), $v);
            assert_eq!($fn($b, $a), $v);
        };
    }

    macro_rules! test_cases {
        ($fn:expr $(,)?) => {{
            both_ways!($fn, "foobar", "", 6);
            both_ways!($fn, "sitting", "kitten", 3);
            both_ways!($fn, "flaw", "lawn", 2);
            both_ways!($fn, "saturday", "sunday", 3);
            both_ways!($fn, "gumbo", "gambol", 2);
            both_ways!($fn, "book", "back", 2);
            both_ways!($fn, "edward", "edwin", 3);
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
        lev_1d_vec_min,
        test_lev_1d_vec_min,
        bench_lev_1d_vec_min_short,
        bench_lev_1d_vec_min_long,
    );
    test_and_bench!(
        lev_1d_arrayvec,
        test_lev_1d_arrayvec,
        bench_lev_1d_arrayvec_short,
        bench_lev_1d_arrayvec_long,
    );
    test_and_bench!(
        lev_1d_arrayvec_min,
        test_lev_1d_arrayvec_min,
        bench_lev_1d_arrayvec_min_short,
        bench_lev_1d_arrayvec_min_long,
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
    test_and_bench!(
        lev_1d_array_min,
        test_lev_1d_array_min,
        bench_lev_1d_array_min_short,
        bench_lev_1d_array_min_long,
    );
    test_and_bench_unsafe!(
        lev_1d_array_unsafe,
        test_lev_1d_array_unsafe,
        bench_lev_1d_array_unsafe_short,
        bench_lev_1d_array_unsafe_long,
    );
}
