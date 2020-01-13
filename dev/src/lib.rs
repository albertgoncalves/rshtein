#![feature(test)]

/* NOTE: _.as_bytes() is fine as long as the given strings only contain
 * symbols between U+0000 and U+007F.
 */

#[must_use]
pub fn lev_recursive(a: &[u8], b: &[u8], i: usize, j: usize) -> usize {
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
        (lev_recursive(a, b, i - 1, j) + 1)
            .min(lev_recursive(a, b, i, j - 1) + 1)
            .min(lev_recursive(a, b, i - 1, j - 1) + penalty)
    }
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

    macro_rules! lev_recursive {
        ($a:expr, $b:expr $(,)?) => {{
            let a: &[u8] = $a.as_bytes();
            let b: &[u8] = $b.as_bytes();
            lev_recursive(a, b, a.len(), b.len())
        }};
    }

    #[test]
    fn test_lev_recursive() {
        assert!(lev_recursive!("sitting", "kitten") == 3);
        assert!(lev_recursive!("flaw", "lawn") == 2);
        assert!(lev_recursive!("saturday", "sunday") == 3);
        assert!(lev_recursive!("gumbo", "gambol") == 2)
    }

    #[bench]
    fn bench_lev_recursive(b: &mut Bencher) {
        b.iter(|| lev_recursive!("sitting", "kitten"))
    }

    #[test]
    fn test_lev_2d_vec() {
        assert!(lev_2d_vec("sitting".as_bytes(), "kitten".as_bytes()) == 3);
        assert!(lev_2d_vec("flaw".as_bytes(), "lawn".as_bytes()) == 2);
        assert!(lev_2d_vec("saturday".as_bytes(), "sunday".as_bytes()) == 3);
        assert!(lev_2d_vec("gumbo".as_bytes(), "gambol".as_bytes()) == 2)
    }

    #[bench]
    fn bench_lev_2d_vec(b: &mut Bencher) {
        b.iter(|| lev_2d_vec("sitting".as_bytes(), "kitten".as_bytes()))
    }

    #[test]
    fn test_lev_1d_vec() {
        assert!(lev_1d_vec("sitting".as_bytes(), "kitten".as_bytes()) == 3);
        assert!(lev_1d_vec("flaw".as_bytes(), "lawn".as_bytes()) == 2);
        assert!(lev_1d_vec("saturday".as_bytes(), "sunday".as_bytes()) == 3);
        assert!(lev_1d_vec("gumbo".as_bytes(), "gambol".as_bytes()) == 2)
    }

    #[bench]
    fn bench_lev_1d_vec(b: &mut Bencher) {
        b.iter(|| lev_1d_vec("sitting".as_bytes(), "kitten".as_bytes()))
    }
}
