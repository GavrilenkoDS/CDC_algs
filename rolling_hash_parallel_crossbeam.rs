use crossbeam::thread;
use std::sync::{Arc, Mutex};

pub fn rolling_hash_parallel_crossbeam(pattern: &[u8], text: &[u8]) -> Vec<usize> {
    let n: usize = text.len();
    let m: usize = pattern.len();
    let results: Arc<Mutex<Vec<usize>>> = Arc::new(Mutex::new(Vec::new()));

    if m == 0 || m > n {
        return Vec::new();
    }

    let pattern_hash: u64 = pattern.iter().fold(0, |acc, &x| (acc << 8) + u64::from(x));

    thread::scope(|s| {
        for (i, window) in text.windows(m).enumerate() {
            let results = Arc::clone(&results);
            s.spawn(move |_| {
                let window_hash: u64 = window.iter().fold(0, |acc, &x| (acc << 8) + u64::from(x));
                if pattern_hash == window_hash && pattern == window {
                    let mut results = results.lock().unwrap();
                    results.push(i);
                }
            });
        }
    })
    .unwrap();

    Arc::try_unwrap(results)
        .unwrap()
        .into_inner()
        .unwrap()
}
