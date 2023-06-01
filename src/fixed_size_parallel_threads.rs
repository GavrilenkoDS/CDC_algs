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
                chunk_result = fixed_size(&pattern, &chunk);
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

pub fn fixed_size(pattern: &[u8], text: &[u8], chunk_size: usize) -> Vec<usize> {
    let n: usize = text.len();
    let m: usize = pattern.len();

    let mut results: Vec<usize> = Vec::new();

    if m > n || m == 0 {
        return Vec::new();
    }

    if m > chunk_size {
        return results;
    }

    let mut start: usize = 0;
    let mut end: usize = chunk_size.min(n);

    while start < n {
        let chunk: &[u8] = &text[start..end];

        for (i, &byte) in chunk.iter().enumerate() {
            if byte == pattern[0] {
                if chunk.len() - i >= m {
                    let window: &[u8] = &chunk[i..i + m];
                    if window == pattern {
                        let offset: usize = start + i;
                        results.push(offset);
                    }
                }
            }
        }

        start += chunk_size;
        end = (end + chunk_size).min(n);
    }

    results
}
