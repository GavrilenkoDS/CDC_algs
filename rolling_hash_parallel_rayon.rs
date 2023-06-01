use rayon::prelude::*;

pub fn rolling_hash_parallel_rayon(pattern: &[u8], text: &[u8]) -> Vec<usize> {
    let n: usize = text.len();
    let m: usize = pattern.len();
    let mut results: Vec<usize> = Vec::new();

    if m == 0 || m > n {
        return results;
    }

    // Calculate hash of the pattern
    let pattern_hash: u64 = pattern.iter().fold(0, |acc, &x| (acc << 8) + u64::from(x));

    results = text.par_windows(m)
        .enumerate()
        .filter_map(|(i, window)| {
            let window_hash: u64 = window.iter().fold(0, |acc, &x| (acc << 8) + u64::from(x));
            if pattern_hash == window_hash && pattern == window {
                Some(i)
            } else {
                None
            }
        })
        .collect();

    results
}
