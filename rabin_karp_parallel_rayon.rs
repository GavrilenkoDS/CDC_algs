use rayon::prelude::*;

const BASE: u64 = 256;
const MOD: u64 = 1_000_000_007;

pub fn rabin_karp_parallel_rayon(pattern: &[u8], text: &[u8]) -> Vec<usize> {
    let n = text.len();
    let m = pattern.len();

    if m == 0 || m > n {
        return Vec::new();
    }

    text.par_windows(m)
        .enumerate()
        .filter_map(|(i, window)| {
            let window_hash = calculate_hash(window);
            if window_hash == calculate_hash(pattern) && window == pattern {
                Some(i)
            } else {
                None
            }
        })
        .collect()
}

fn calculate_hash(data: &[u8]) -> u64 {
    let mut hash: u64 = 0;

    for &ch in data {
        hash = (hash * BASE + ch as u64) % MOD;
    }

    hash
}
