use std::thread;
use num_cpus;
const MOD: usize = 1_000_000_007;
const BASE: usize = 256;
pub fn rolling_hash_parallel(pattern: &[u8], text: &[u8]) -> Vec<usize> {
    
    let num_threads = num_cpus::get();
    //Length exeption
    if pattern.len() > text.len() || pattern.len() == 0 {
        return Vec::new();
    }

    // Split the text into chunks for each thread
    let chunk_size = text.len() / num_threads;
    let chunks: Vec<Vec<u8>> = (0..num_threads)
    .map(|i| text[i * chunk_size..(i + 1) * chunk_size].to_vec())
    .collect();

    // Define a mutable vector to store the results
    let mut result: Vec<usize> = Vec::new();

    // Spawn a thread for each chunk
    let handles: Vec<_> = chunks
        .into_iter()
        .map(|chunk| {
            let pattern = pattern.to_owned();
            let mut chunk_result = Vec::new();
            let handle = thread::spawn(move || {
                chunk_result = rolling_hash(&pattern, &chunk);
                chunk_result
            });
            handle
        })
        .collect();

    // Collect the results from each thread
    for handle in handles {
        let chunk_result: Vec<usize> = handle.join().unwrap();
        result.extend(chunk_result);
    }

    result
}

fn rolling_hash(pattern: &[u8], text: &[u8]) -> Vec<usize> {

    //Length exeption
    if pattern.len() > text.len() || pattern.len() == 0 {
        return Vec::new();
    }



    let mut result: Vec<usize> = Vec::new();

    // Calculate hash of pattern
    let pattern_hash: usize = pattern.iter().fold(0, |acc: usize, &x| (acc * BASE + x as usize) & (MOD - 1));

    // Calculate initial hash of text
    let mut text_hash: usize = text.iter().take(pattern.len()).fold(0, |acc: usize, &x| (acc * BASE + x as usize) & (MOD - 1));

    // Check for match in initial substring
    if pattern_hash == text_hash && pattern == &text[0..pattern.len()] {
        result.push(0);
    }

    // Precompute powers of the base
    let mut base_pow: usize = 1;
    for _ in 0..pattern.len() {
        base_pow = (base_pow * BASE) & (MOD - 1);
    }

    // Iterate over remaining substrings of text
    for i in 1..text.len() - pattern.len() + 1 {
        // Update hash using sliding window technique
        text_hash = (((text_hash * BASE) & (MOD - 1)) + text[i + pattern.len() - 1] as usize - ((text[i - 1] as usize * base_pow) & (MOD - 1))) & (MOD - 1);

        // Check for match
        if pattern_hash == text_hash && pattern == &text[i..i + pattern.len()] {
            result.push(i);
        }
    }

    result
}
