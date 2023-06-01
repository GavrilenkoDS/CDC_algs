const BASE: u64 = 256;
const MOD: u64 = 1_000_000_007;

pub fn rabin_karp(pattern: &[u8], text: &[u8]) -> Vec<usize> {
    let n = text.len();
    let m = pattern.len();
    let mut result: Vec<usize> = Vec::new();

    if m == 0 || m > n {
        return result;
    }

    // Calculate the hash of the pattern
    let pattern_hash = calculate_hash(pattern);

    // Calculate the initial hash of the first window in the text
    let mut window_hash = calculate_hash(&text[0..m]);

    // Check if the initial window matches the pattern
    if window_hash == pattern_hash && &text[0..m] == pattern {
        result.push(0);
    }

    // Precompute the value of BASE raised to the power of (m-1)
    let base_power = BASE.pow((m - 1) as u32) % MOD;

    // Iterate through the remaining windows in the text
    for i in 1..=n - m {
        // Update the rolling hash by removing the contribution of the first character
        window_hash = ((window_hash + MOD) - (base_power * text[i - 1] as u64) % MOD) % MOD;

        // Update the rolling hash by adding the contribution of the next character
        window_hash = (window_hash * BASE % MOD + text[i + m - 1] as u64) % MOD;

        // Check if the current window matches the pattern
        if window_hash == pattern_hash && &text[i..i + m] == pattern {
            result.push(i);
        }
    }

    result
}

fn calculate_hash(data: &[u8]) -> u64 {
    let mut hash: u64 = 0;

    for &ch in data {
        hash = (hash * BASE + ch as u64) % MOD;
    }

    hash
}
