use rayon::prelude::*;
use rayon::slice::ParallelSlice;

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

    // Create an iterator over the chunks of the text
    let text_chunks = text.par_chunks(pattern.len());

    // Filter and map the text chunks to matching indices
    let matching_indices = text_chunks.enumerate().filter_map(|(i, chunk)| {
        // Calculate initial hash of text
        let text_hash: usize = chunk.iter().fold(0, |acc: usize, &x| (acc * BASE + x as usize) & (MOD - 1));

        // Check for match in initial substring
        if pattern_hash == text_hash && pattern == chunk {
            Some(i * pattern.len())
        } else {
            None
        }
    });

    // Collect the matching indices into a vector
    matching_indices.collect()
}
