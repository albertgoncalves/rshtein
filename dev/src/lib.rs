#[must_use]
pub fn lev_recursive(
    a: &[u8],
    b: &[u8],
    index_a: usize,
    index_b: usize,
) -> usize {
    /* NOTE: _.as_bytes() is fine as long as the given strings only contain
     * symbols in the range of U+0000 to U+007F.
     */
    if index_a == 0 {
        index_b
    } else if index_b == 0 {
        index_a
    } else {
        let penalty: usize = {
            if a[index_a - 1] == b[index_b - 1] {
                0
            } else {
                1
            }
        };
        (lev_recursive(a, b, index_a - 1, index_b) + 1)
            .min(lev_recursive(a, b, index_a, index_b - 1) + 1)
            .min(lev_recursive(a, b, index_a - 1, index_b - 1) + penalty)
    }
}

#[must_use]
pub fn levenshtein(a: &str, b: &str) -> usize {
    lev_recursive(a.as_bytes(), b.as_bytes(), a.len(), b.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sitting_kitten() {
        assert!(levenshtein("sitting", "kitten") == 3)
    }

    #[test]
    fn flaw_lawn() {
        assert!(levenshtein("flaw", "lawn") == 2)
    }

    #[test]
    fn saturday_sunday() {
        assert!(levenshtein("saturday", "sunday") == 3)
    }
}
