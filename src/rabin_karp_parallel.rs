use rayon::prelude::*;

pub fn rabin_karp_parallel(pattern: &[u8], text: &[u8]) -> Vec<usize> {
    const BASE: usize = 256;
    const MOD: usize = 997; // A prime number

    let pattern_len = pattern.len();
    let text_len = text.len();

    let pattern_hash: usize = pattern.iter().fold(0, |acc, &x| (acc * BASE + x as usize) % MOD);

    let result: Vec<usize> = (0..=text_len - pattern_len)
        .into_par_iter()
        .filter(|&i| {
            let text_hash: usize = text[i..i + pattern_len].iter().fold(0, |acc, &x| (acc * BASE + x as usize) % MOD);
            text_hash == pattern_hash && pattern == &text[i..i + pattern_len]
        })
        .collect();

    result
}