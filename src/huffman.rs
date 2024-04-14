use std::cmp::Reverse;
use std::collections::BinaryHeap;

use crate::package::Package;

/// generate optimal huffman codes
/// iter: counts of each symbol
/// returns code-lengths of each symbol
pub fn generate_canonical_huffman_code(iter: impl Iterator<Item = usize>) -> Vec<usize> {
    let mut queue = BinaryHeap::new(); // max-heap
    let mut nsymbols = 0;
    for (symbol, count) in iter.enumerate() {
        if count > 0 {
            queue.push(Reverse(Package::new(symbol, count))); // min-heap by count
        }
        nsymbols += 1;
    }

    let mut code_lengths = vec![0; nsymbols];
    for x in queue.iter() {
        code_lengths[x.0.symbols()[0]] += 1;
    }

    while queue.len() > 2 {
        let x = queue.pop().unwrap();
        let y = queue.pop().unwrap();
        let merged = x.0 + y.0;
        for symbol in merged.symbols() {
            code_lengths[*symbol] += 1;
        }
        queue.push(Reverse(merged));
    }

    code_lengths
}

#[cfg(test)]
mod test {
    use crate::huffman::generate_canonical_huffman_code;

    #[test]
    fn test0() {
        let freq = vec![0, 1];
        let bitlen = generate_canonical_huffman_code(freq.into_iter());
        assert_eq!(bitlen, [0, 1]);
    }

    #[test]
    fn test1() {
        let freq = vec![40, 35, 20, 5];
        let bitlen = generate_canonical_huffman_code(freq.into_iter());
        assert_eq!(bitlen, [1, 2, 3, 3]);
    }

    #[test]
    fn test2() {
        let freq = vec![1];
        let bitlen = generate_canonical_huffman_code(freq.into_iter());
        assert_eq!(bitlen, [1]);
    }

    #[test]
    fn test3() {
        let freq = vec![40, 35, 20, 0];
        let bitlen = generate_canonical_huffman_code(freq.into_iter());
        assert_eq!(bitlen, [1, 2, 2, 0]);
    }

    #[test]
    fn test4() {
        let freq = vec![7, 4, 4, 3, 2, 2, 2, 2, 2, 2, 1, 1, 1, 1, 1, 1];
        let bitlen = generate_canonical_huffman_code(freq.into_iter());
        assert_eq!(bitlen, [3, 3, 3, 4, 4, 4, 4, 4, 4, 4, 5, 5, 5, 5, 5, 5]);
    }

    #[test]
    fn test5() {
        let freq = vec![0, 1, 1, 2, 4, 8, 16, 32, 64, 128, 256, 512];
        let bitlen = generate_canonical_huffman_code(freq.into_iter());
        assert_eq!(bitlen, [0, 10, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]);
    }

    #[test]
    fn test6() {
        let freq = vec![];
        let bitlen = generate_canonical_huffman_code(freq.into_iter());
        assert_eq!(bitlen, []);
    }

    #[test]
    fn test7() {
        let freq = vec![0, 0, 0, 0, 0];
        let bitlen = generate_canonical_huffman_code(freq.into_iter());
        assert_eq!(bitlen, [0, 0, 0, 0, 0]);
    }

    #[test]
    fn test8() {
        let freq = vec![0, 0, 0, 1, 10000];
        let bitlen = generate_canonical_huffman_code(freq.into_iter());
        assert_eq!(bitlen, [0, 0, 0, 1, 1]);
    }

    #[test]
    fn test9() {
        let freq = vec![
            1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987,
        ];
        println!("{}", freq.iter().sum::<usize>());
        let bitlen = generate_canonical_huffman_code(freq.into_iter());
        assert_eq!(
            bitlen,
            [15, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]
        );
    }
}
