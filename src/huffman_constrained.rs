use std::cmp::Reverse;

use crate::package::Package;

/// generate optimal huffman codes under code-length constraint
/// iter: counts of each symbol
/// returns code-lengths of each symbol
pub fn generate_canonical_huffman_code_with_constraint(
    iter: impl Iterator<Item = usize>,
    max_code_length: usize,
) -> Vec<usize> {
    let mut nsymbols = 0;
    let mut xs = Vec::new();
    for (symbol, count) in iter.enumerate() {
        if count > 0 {
            xs.push(Reverse(Package::new(symbol, count)));
        }
        nsymbols += 1;
    }

    assert!(1 <= max_code_length || xs.is_empty());
    assert!(xs.len() <= 1 << max_code_length);
    let mut code_lengths = vec![0; nsymbols];

    if xs.len() == 1 {
        // corner case
        let s = xs[0].0.symbols()[0];
        code_lengths[s] += 1;
        return code_lengths;
    }

    let mut result = Vec::new();
    for _ in 0..max_code_length {
        result.extend_from_slice(&xs);
        result = merge_pairs(result);
    }

    // choose N - 1 packages with least counts where N = xs.len()
    let idx = result.len() + 1 - xs.len();
    if idx >= result.len() {
        // corner case
        return code_lengths;
    }
    result.select_nth_unstable(idx);

    for x in result.into_iter().skip(idx) {
        for s in x.0.symbols() {
            code_lengths[*s] += 1;
        }
    }

    code_lengths
}

/// given packages, merge two least-count packages into one
fn merge_pairs(mut src: Vec<Reverse<Package>>) -> Vec<Reverse<Package>> {
    src.sort(); // descending order by count
    let mut result = Vec::new();
    while src.len() >= 2 {
        let x = src.pop().unwrap();
        let y = src.pop().unwrap();
        let merged = x.0 + y.0;
        result.push(Reverse(merged));
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test0() {
        let freq = vec![0, 1];
        let bitlen = generate_canonical_huffman_code_with_constraint(freq.into_iter(), 1);
        assert_eq!(bitlen, [0, 1]);
    }

    #[test]
    fn test1() {
        let freq = vec![40, 35, 20, 5];
        let bitlen = generate_canonical_huffman_code_with_constraint(freq.into_iter(), 3);
        assert_eq!(bitlen, [1, 2, 3, 3]);
    }

    #[test]
    fn test1a() {
        let freq = vec![40, 35, 20, 5];
        let bitlen = generate_canonical_huffman_code_with_constraint(freq.into_iter(), 2);
        assert_eq!(bitlen, [2, 2, 2, 2]);
    }

    #[test]
    fn test2() {
        let freq = vec![1];
        let bitlen = generate_canonical_huffman_code_with_constraint(freq.into_iter(), 1);
        assert_eq!(bitlen, [1]);
    }

    #[test]
    fn test3() {
        let freq = vec![40, 35, 20, 0];
        let bitlen = generate_canonical_huffman_code_with_constraint(freq.into_iter(), 2);
        assert_eq!(bitlen, [1, 2, 2, 0]);
    }

    #[test]
    fn test4() {
        let freq = vec![7, 4, 4, 3, 2, 2, 2, 2, 2, 2, 1, 1, 1, 1, 1, 1];
        let bitlen = generate_canonical_huffman_code_with_constraint(freq.into_iter(), 5);
        assert_eq!(bitlen, [3, 3, 3, 4, 4, 4, 4, 4, 4, 4, 5, 5, 5, 5, 5, 5]);
    }

    #[test]
    fn test5() {
        let freq = vec![0, 1, 1, 2, 4, 8, 16, 32, 64, 128, 256, 512];
        let bitlen = generate_canonical_huffman_code_with_constraint(freq.into_iter(), 10);
        assert_eq!(bitlen, [0, 10, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]);
    }

    #[test]
    fn test5a() {
        let freq = vec![0, 1, 1, 2, 4, 8, 16, 32, 64, 128, 256, 512];
        let bitlen = generate_canonical_huffman_code_with_constraint(freq.into_iter(), 9);
        assert_eq!(bitlen, [0, 9, 9, 9, 9, 7, 6, 5, 4, 3, 2, 1]);
    }

    #[test]
    fn test5b() {
        let freq = vec![0, 1, 1, 2, 4, 8, 16, 32, 64, 128, 256, 512];
        let bitlen = generate_canonical_huffman_code_with_constraint(freq.into_iter(), 8);
        assert_eq!(bitlen, [0, 8, 8, 8, 8, 7, 7, 5, 4, 3, 2, 1]);
    }

    #[test]
    fn test5c() {
        let freq = vec![0, 1, 1, 2, 4, 8, 16, 32, 64, 128, 256, 512];
        let bitlen = generate_canonical_huffman_code_with_constraint(freq.into_iter(), 7);
        assert_eq!(bitlen, [0, 7, 7, 7, 7, 7, 7, 6, 4, 3, 2, 1]);
    }

    #[test]
    fn test5d() {
        let freq = vec![0, 1, 1, 2, 4, 8, 16, 32, 64, 128, 256, 512];
        let bitlen = generate_canonical_huffman_code_with_constraint(freq.into_iter(), 6);
        assert_eq!(bitlen, [0, 6, 6, 6, 6, 6, 6, 5, 4, 4, 2, 1]);
    }

    #[test]
    fn test6() {
        let freq = vec![];
        let bitlen = generate_canonical_huffman_code_with_constraint(freq.into_iter(), 0);
        assert_eq!(bitlen, []);
    }

    #[test]
    fn test7() {
        let freq = vec![0, 0, 0, 0, 0];
        let bitlen = generate_canonical_huffman_code_with_constraint(freq.into_iter(), 0);
        assert_eq!(bitlen, [0, 0, 0, 0, 0]);
    }

    #[test]
    fn test8() {
        let freq = vec![0, 0, 0, 1, 10000];
        let bitlen = generate_canonical_huffman_code_with_constraint(freq.into_iter(), 1);
        assert_eq!(bitlen, [0, 0, 0, 1, 1]);
    }

    #[test]
    fn test8b() {
        let freq = vec![0, 0, 0, 1, 10000];
        let bitlen = generate_canonical_huffman_code_with_constraint(freq.into_iter(), 2);
        assert_eq!(bitlen, [0, 0, 0, 1, 1]);
    }
}
