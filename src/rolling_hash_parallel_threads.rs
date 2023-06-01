use std::thread;
use num_cpus;
pub fn rolling_hash_parallel_threads(pattern: &[u8], text: &[u8]) -> Vec<usize> {
    
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

pub fn rolling_hash(pattern: &[u8], text: &[u8]) -> Vec<usize> {
    let n: usize = text.len();
    let m: usize = pattern.len();
    let mut results: Vec<usize> = Vec::new();

    if m == 0 || m > n {
        return results;
    }

    // Calculate hash of the pattern
    let pattern_hash: u64 = pattern.iter().fold(0, |acc, &x| (acc << 8) + u64::from(x));

    // Calculate initial hash of the first substring in the text
    let mut text_hash: u64 = text.iter().take(m).fold(0, |acc, &x| (acc << 8) + u64::from(x));

    // Check if the first substring matches the pattern
    if pattern_hash == text_hash && pattern == &text[0..m] {
        results.push(0);
    }

    // Calculate the rolling hash for the remaining substrings
    for i in 1..=n - m {
        // Remove the leftmost character from the rolling hash
        text_hash -= u64::from(text[i - 1]) << (8 * (m - 1));

        // Shift the rolling hash left by one position and add the new character
        text_hash = (text_hash << 8) + u64::from(text[i + m - 1]);

        // Check if the rolling hash matches the pattern
        if pattern_hash == text_hash && pattern == &text[i..i + m] {
            results.push(i);
        }
    }

    results
}
