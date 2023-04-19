use std::sync::Mutex;
use crossbeam::scope;
mod rabin_karp_constants {
    pub const PRIME: u64 = 101;
}

use rabin_karp_constants::PRIME;

pub fn rabin_karp_parallel(pattern: &[u8], text: &[u8]) -> Vec<usize> {
    let n: usize = text.len();
    let m: usize = pattern.len();

    let results: Mutex<Vec<usize>> = Mutex::new(Vec::new());

    if m == 0 || m > n {
        return Vec::new();
    }

    let num_threads: usize = num_cpus::get();
   
    scope(|s: &crossbeam::thread::Scope| {
        for tid in 0..num_threads {
            let chunk_size: usize = (n + num_threads - 1) / num_threads;
            let start: usize = tid * chunk_size;
            let end: usize = (start + chunk_size).min(n);

            let results: &Mutex<Vec<usize>> = &results;

            s.spawn(move |_| {
                let mut pattern_hash: u64 = 0;
                let mut text_hash: u64 = 0;

                // calculate hash of pattern and first m characters of text
                for i in 0..m {
                    pattern_hash = pattern_hash.wrapping_mul(PRIME).wrapping_add(pattern[i] as u64);
                    text_hash = text_hash.wrapping_mul(PRIME).wrapping_add(text[i + start] as u64);
                }

                // iterate over remaining substrings of text
                for i in 0..end - start - m + 1 {
                    // check if hash values match
                    if pattern_hash == text_hash {
                        // check if the substrings match
                        if pattern == &text[i + start..i + start + m] {
                            results.lock().unwrap().push(i + start);
                        }
                    }

                    // update hash of text for next substring
                    if i < end - start - m {
                        text_hash = text_hash.wrapping_sub((text[i + start] as u64).wrapping_mul(PRIME.pow(m as u32 - 1)));
                        text_hash = text_hash.wrapping_mul(PRIME);
                        text_hash = text_hash.wrapping_add(text[i + start + m] as u64);
                    }
                }
            });
        }
    }).unwrap();

    results.into_inner().unwrap()
}
