use rayon::prelude::*;

const MOD: usize = 1_000_000_007;
const BASE: usize = 256;

pub fn rolling_hash_parallel(pattern: &[u8], text: &[u8]) -> Vec<usize> {
    // Length exception
    if pattern.len() > text.len() || pattern.len() == 0 {
        return Vec::new();
    }

    // Calculate hash of pattern
    let pattern_hash: usize = pattern.iter().fold(0, |acc: usize, &x| (acc * BASE + x as usize) & (MOD - 1));

    // Precompute powers of the base
    let mut base_pow: usize = 1;
    for _ in 0..pattern.len() {
        base_pow = (base_pow * BASE) & (MOD - 1);
    }

    // Create an iterator over the indices of the text
    let text_indices = (0..text.len() - pattern.len() + 1).into_par_iter();

    // Filter and map the text indices to matching indices
    let matching_indices = text_indices.filter_map(|i| {
        // Calculate initial hash of text
        let text_hash: usize = text.iter().skip(i).take(pattern.len()).fold(0, |acc: usize, &x| (acc * BASE + x as usize) & (MOD - 1));

        // Check for match in initial substring
        if pattern_hash == text_hash && pattern == &text[i..i + pattern.len()] {
            Some(i)
        } else {
            None
        }
    });

    // Collect the matching indices into a vector
    matching_indices.collect()
}
